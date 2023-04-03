mod extract;
mod fixers;
mod mtime;
mod rename;
mod track;
mod types;

use anyhow::Result;
use clap::Parser;
use funcfmt::{fm, FormatMap, FormatPieces, ToFormatPieces};
use id3::TagLike;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use walkdir::WalkDir;

const ALLOWED_EXTS: &[&str] = &["mp3", "flac", "m4a"];

fn fix_track(track: &mut types::Track, dry_run: bool) {
    let fix_results = fixers::run_fixers(track, dry_run);
    match fix_results {
        Ok(applied_fixers) => {
            if applied_fixers {
                print_updated_tags(track);
            }
        }
        Err(err) => eprintln!("cannot fix {}: {:?}", track.path.display(), err),
    }
}

fn print_updated_tags(track: &types::Track) {
    println!(
        "{}: updated tags: artist: '{}', album: '{}', title: '{}'",
        track.path.display(),
        track.tag.artist().unwrap_or_default(),
        track.tag.album().unwrap_or_default(),
        track.tag.title().unwrap_or_default()
    );
}

fn rename_track(
    track: &types::Track,
    fp: &FormatPieces<types::Track>,
    output_path: &Path,
    dry_run: bool,
) {
    let new_path = rename::rename_track(track, fp, output_path, dry_run);

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

// Arbitrary limit on path part without extension to try to avoid brushing against PATH_MAX. We
// can't just check PATH_MAX and similar, because we also want to avoid issues when copying
// elsewhere later.
const MAX_PATH_PART_LEN: usize = 64;
const ADDITIONAL_ACCEPTED_CHARS: &[char] = &['.', '-', '(', ')', ','];

fn clean_part(path_part: &str) -> String {
    let mut out: String = path_part
        .chars()
        .map(|c| {
            if c.is_alphanumeric()
                || c.is_whitespace()
                || ADDITIONAL_ACCEPTED_CHARS.iter().any(|&a| a == c)
            {
                c
            } else {
                '_'
            }
        })
        .collect();
    out.truncate(MAX_PATH_PART_LEN);
    out
}

fn get_format_pieces(tmpl: &str) -> Result<funcfmt::FormatPieces<types::Track>> {
    let formatters: FormatMap<types::Track> = FormatMap::from([
        fm!("artist", |t: &types::Track| Some(clean_part(
            t.tag.artist().unwrap_or("Unknown Artist")
        ))),
        fm!("album", |t: &types::Track| Some(clean_part(
            t.tag.album().unwrap_or("Unknown Album")
        ))),
        fm!("title", |t: &types::Track| Some(clean_part(
            t.tag.title().unwrap_or("Unknown Title")
        ))),
        fm!("track", |t: &types::Track| Some(format!(
            "{:02}",
            t.tag.track().unwrap_or_default()
        ))),
    ]);

    Ok(formatters.to_format_pieces(tmpl)?)
}

fn is_updated_since_last_run(path: &PathBuf, last_run_time: SystemTime) -> bool {
    mtime::mtime_def_now(path) > last_run_time
}

fn fix_all_tracks(cfg: &types::Config, base_path: &PathBuf, output_path: &Path) {
    // If the output path is different, we don't know if we should run or not, so just do them all
    let last_run_time = if output_path == base_path {
        mtime::get_last_run_time(base_path).unwrap_or(SystemTime::UNIX_EPOCH)
    } else {
        SystemTime::UNIX_EPOCH
    };

    let fp = match get_format_pieces(&cfg.fmt) {
        Ok(fp) => fp,
        Err(err) => {
            eprintln!("fatal: {}", err);
            std::panic::set_hook(Box::new(|_| {}));
            panic!(); // Don't use exit() because it does not run destructors
        }
    };

    let walker = WalkDir::new(base_path)
        .into_iter()
        .filter_map(std::result::Result::ok)
        .filter(|e| e.file_type().is_file())
        .map(|e| e.path().to_path_buf())
        .filter(|e| {
            ALLOWED_EXTS
                .iter()
                .any(|ext| &e.extension().and_then(OsStr::to_str).unwrap_or("") == ext)
        })
        .filter(|e| cfg.force || is_updated_since_last_run(e, last_run_time));

    for path in walker {
        match track::get_track(path.clone()) {
            Ok(mut track) => {
                fix_track(&mut track, cfg.dry_run);
                rename_track(&track, &fp, output_path, cfg.dry_run);
            }
            Err(err) => eprintln!("error: {}: {err:?}", path.display()),
        }
    }

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
    let cfg = types::Config::parse();

    let paths = if cfg.paths.is_empty() {
        vec![PathBuf::from(".")]
    } else {
        cfg.paths.clone()
    };

    for path in paths {
        let this_output_path;

        if let Some(op) = cfg.output_dir.clone() {
            this_output_path = op;
        } else {
            this_output_path = path.clone();
        }

        fix_all_tracks(&cfg, &path, &this_output_path);
    }
}
