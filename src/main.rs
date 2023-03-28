mod extract;
mod fixers;
mod mtime;
mod rename;
mod track;
mod types;

use clap::Parser;
use id3::TagLike;
use lazy_static::lazy_static;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use walkdir::WalkDir;

lazy_static! {
    static ref ALLOWED_EXTS: Vec<&'static OsStr> =
        vec![OsStr::new("mp3"), OsStr::new("flac"), OsStr::new("m4a")];
}

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

fn rename_track(track: &types::Track, output_path: &Path, dry_run: bool) {
    let new_path = rename::rename_track(track, output_path, dry_run);

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

fn is_eligible_for_fixing(path: &PathBuf, last_run_time: SystemTime, force: bool) -> bool {
    let parent = path.parent();
    force
        || mtime::mtime_def_now(path) > last_run_time
        || (parent.is_some() && mtime::mtime_def_now(parent.unwrap()) > last_run_time)
}

fn fix_all_tracks(base_path: &PathBuf, output_path: &Path, dry_run: bool, force: bool) {
    // If the output path is different, we don't know if we should run or not, so just do them all
    let last_run_time = if output_path == base_path {
        mtime::get_last_run_time(base_path).unwrap_or(SystemTime::UNIX_EPOCH)
    } else {
        SystemTime::UNIX_EPOCH
    };

    let walker = WalkDir::new(base_path)
        .into_iter()
        .filter_map(std::result::Result::ok)
        .filter(|e| e.file_type().is_file())
        .map(|e| e.path().to_path_buf())
        .filter(|e| ALLOWED_EXTS.contains(&e.extension().unwrap_or_else(|| OsStr::new(""))))
        .filter(|e| is_eligible_for_fixing(e, last_run_time, force));

    for path in walker {
        match track::get_track(path) {
            Ok(mut track) => {
                fix_track(&mut track, dry_run);
                rename_track(&track, output_path, dry_run);
            }
            Err(err) => eprintln!("error: {err:?}"),
        }
    }

    if !dry_run && output_path == base_path {
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
    let args = types::Config::parse();

    let paths = if args.paths.is_empty() {
        vec![PathBuf::from(".")]
    } else {
        args.paths
    };

    for path in paths {
        let this_output_path;

        if let Some(op) = args.output_dir.clone() {
            this_output_path = op;
        } else {
            this_output_path = path.clone();
        }

        fix_all_tracks(&path, &this_output_path, args.dry_run, args.force);
    }
}
