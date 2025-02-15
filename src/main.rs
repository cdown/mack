mod config;
mod mtime;
mod track;

use anyhow::Result;
use clap::Parser;
use funcfmt::{fm, FormatPieces, ToFormatPieces};
use id3::TagLike;
use jwalk::WalkDir;
use rayon::prelude::*;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use config::Config;
use track::{get_track, Track};

const ALLOWED_EXTS: &[&str] = &["mp3", "flac", "m4a"];

fn fix_track(track: &mut Track, dry_run: bool) {
    let fix_results = track::fixers::run_fixers(track, dry_run);
    match fix_results {
        Ok(applied_fixers) => {
            if applied_fixers {
                print_updated_tags(track);
            }
        }
        Err(err) => eprintln!("cannot fix {}: {:?}", track.path.display(), err),
    }
}

fn print_updated_tags(track: &Track) {
    println!(
        "{}: updated tags: artist: '{}', album: '{}', title: '{}'",
        track.path.display(),
        track.tag.artist().unwrap_or_default(),
        track.tag.album().unwrap_or_default(),
        track.tag.title().unwrap_or_default()
    );
}

fn rename_track(track: &Track, fp: &FormatPieces<Track>, output_path: &Path, dry_run: bool) {
    let new_path = track::rename::rename_track(track, fp, output_path, dry_run);

    match new_path {
        Ok(Some(new_path)) => println!(
            "{}: renamed to {}",
            track.path.display(),
            new_path.display()
        ),
        Ok(None) => (),
        Err(err) => eprintln!("cannot rename {}: {:?}", track.path.display(), err),
    }
}

const ADDITIONAL_ACCEPTED_CHARS: &[char] = &['.', '-', '(', ')', ','];

fn clean_part(path_part: &str) -> String {
    path_part
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c.is_whitespace() || ADDITIONAL_ACCEPTED_CHARS.contains(&c) {
                c
            } else {
                '_'
            }
        })
        .collect()
}

fn get_format_pieces(tmpl: &str) -> Result<funcfmt::FormatPieces<Track>> {
    let formatters = fm!(
        "artist" => |t: &Track| Some(clean_part(t.tag.artist().unwrap_or("Unknown Artist"))),
        "album" => |t: &Track| Some(clean_part(t.tag.album().unwrap_or("Unknown Album"))),
        "title" => |t: &Track| Some(clean_part(t.tag.title().unwrap_or("Unknown Title"))),
        "track" => |t: &Track| Some(format!("{:02}", t.tag.track().unwrap_or_default())),
    );

    Ok(formatters.to_format_pieces(tmpl)?)
}

fn is_updated_since_last_run(path: &PathBuf, last_run_time: SystemTime) -> bool {
    mtime::mtime_def_now(path) > last_run_time
}

fn fix_all_tracks(cfg: &Config, base_path: &PathBuf, output_path: &Path) {
    // If the output path is different, we don't know if we should run or not, so just do them all
    let last_run_time = if output_path == base_path {
        mtime::get_last_run_time(base_path).unwrap_or(SystemTime::UNIX_EPOCH)
    } else {
        SystemTime::UNIX_EPOCH
    };

    let fp = match get_format_pieces(&cfg.fmt) {
        Ok(fp) => fp,
        Err(err) => {
            eprintln!("fatal: {err}");
            std::panic::set_hook(Box::new(|_| {}));
            panic!(); // Don't use exit() because it does not run destructors
        }
    };

    WalkDir::new(base_path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .map(|e| e.path())
        .filter(|e| {
            let ext = e
                .extension()
                .and_then(OsStr::to_str)
                .unwrap_or("")
                .to_lowercase();
            ALLOWED_EXTS.iter().any(|a| a == &ext)
        })
        .filter(|e| cfg.force || is_updated_since_last_run(e, last_run_time))
        .collect::<Vec<_>>()
        .into_par_iter()
        .for_each(|path| match get_track(path.clone()) {
            Ok(mut track) => {
                fix_track(&mut track, cfg.dry_run);
                rename_track(&track, &fp, output_path, cfg.dry_run);
            }
            Err(err) => eprintln!("error: {}: {err:?}", path.display()),
        });

    if !cfg.dry_run && output_path == base_path {
        mtime::set_last_run_time(base_path).unwrap_or_else(|err| {
            eprintln!(
                "can't set last run time for {}: {:?}",
                base_path.display(),
                err
            );
        });
    }
}

fn main() {
    let mut cfg = Config::parse();

    let paths = cfg.paths.take().unwrap_or_else(|| vec![PathBuf::from(".")]);

    for path in paths {
        let output_path = cfg.output_dir.clone().unwrap_or_else(|| path.clone());
        fix_all_tracks(&cfg, &path, &output_path);
    }
}
