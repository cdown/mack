mod extract;
mod fixers;
mod mtime;
mod rename;
mod track;
mod types;

use std::ffi::OsStr;
use std::path::PathBuf;
use std::time::SystemTime;
use walkdir::WalkDir;
use clap::crate_version;
use lazy_static::lazy_static;

lazy_static! {
    static ref ALLOWED_EXTS: Vec<&'static OsStr> =
        vec![OsStr::new("mp3"), OsStr::new("flac"), OsStr::new("m4a")];
}

fn parse_args() -> clap::ArgMatches {
    clap::App::new("mack")
        .version(crate_version!())
        .about("The opinionated music library organiser.")
        .arg(
            clap::Arg::with_name("PATH")
                .multiple(true)
                .default_value(".")
                .help("Paths to get files from, directories are recursed into"),
        )
        .arg(
            clap::Arg::with_name("dry_run")
                .long("dry-run")
                .short('n')
                .help("Show what we would do, but don't do it"),
        )
        .arg(
            clap::Arg::with_name("force")
                .long("force")
                .short('f')
                .help("Ignore .lastmack timestamp, run on all files present regardless"),
        )
        .arg(
            clap::Arg::with_name("output_dir")
                .long("output-dir")
                .short('o')
                .value_name("DIR")
                .takes_value(true)
                .help("Use a different output directory (default: the same as the input dir)"),
        )
        .get_matches()
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
    match track.tag_file.tag() {
        Ok(tags) => println!(
            "{}: updated tags: artist: '{}', album: '{}', title: '{}'",
            track.path.display(),
            tags.artist().unwrap_or_default(),
            tags.album().unwrap_or_default(),
            tags.title().unwrap_or_default()
        ),
        Err(err) => eprintln!(
            "error getting tag info: {}: {:?}",
            track.path.display(),
            err
        ),
    }
}

fn rename_track(track: &types::Track, output_path: &PathBuf, dry_run: bool) {
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
    path.is_file()
        && (force
            || mtime::mtime_def_now(path) > last_run_time
            || (parent.is_some() && mtime::mtime_def_now(parent.unwrap()) > last_run_time))
}

fn fix_all_tracks(base_path: &PathBuf, output_path: &PathBuf, dry_run: bool, force: bool) {
    let last_run_time;

    // If the output path is different, we don't know if we should run or not, so just do them all
    if output_path == base_path {
        last_run_time = mtime::get_last_run_time(base_path).unwrap_or(SystemTime::UNIX_EPOCH);
    } else {
        last_run_time = SystemTime::UNIX_EPOCH;
    }

    let walker = WalkDir::new(base_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|e| e.path().to_path_buf())
        .filter(|e| ALLOWED_EXTS.contains(&e.extension().unwrap_or_else(|| OsStr::new(""))))
        .filter(|e| is_eligible_for_fixing(e, last_run_time, force));

    for path in walker {
        match track::get_track(path) {
            Ok(mut track) => {
                fix_track(&mut track, dry_run);
                rename_track(&track, output_path, dry_run);
            }
            Err(err) => eprintln!("error: {:?}", err),
        }
    }

    if !dry_run && output_path == base_path {
        mtime::set_last_run_time(base_path).unwrap_or_else(|err| {
            eprintln!(
                "can't set last run time for {}: {:?}",
                base_path.display(),
                err
            )
        });
    }
}

fn main() {
    let args = parse_args();
    let mut output_path = None;

    if args.is_present("output_dir") {
        let mut inner = PathBuf::new();
        inner.push(
            args.value_of("output_dir")
                .expect("BUG: where did output_dir arg go?"),
        );
        output_path = Some(inner);
    }

    for raw_path in args
        .values_of("PATH")
        .expect("BUG: missing default path")
        .collect::<Vec<&str>>()
    {
        let this_output_path;
        let mut path = PathBuf::new();
        path.push(raw_path);

        if let Some(op) = output_path.clone() {
            this_output_path = op;
        } else {
            this_output_path = path.clone();
        }

        fix_all_tracks(
            &path,
            &this_output_path,
            args.is_present("dry_run"),
            args.is_present("force"),
        );
    }
}
