extern crate clap;
extern crate ignore;
#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate taglib;

mod extract;
mod fixers;
mod mtime;
mod rename;
mod track;
mod types;

use std::path::PathBuf;
use std::time::SystemTime;

fn build_music_walker(dir: &PathBuf) -> Result<ignore::Walk, types::MackError> {
    let mut mt_builder = ignore::types::TypesBuilder::new();
    for glob in &["*.mp3", "*.flac"] {
        mt_builder.add("music", glob)?;
    }
    mt_builder.select("music");
    let music_types = mt_builder.build()?;
    Ok(ignore::WalkBuilder::new(dir).types(music_types).build())
}

fn parse_args<'a>() -> clap::ArgMatches<'a> {
    clap::App::new("mack")
        .version("0.1.0")
        .about("The opinionated music library organiser.")
        .arg(
            clap::Arg::with_name("PATH")
                .multiple(true)
                .default_value(".")
                .help(
                    "Paths to fix, directories are recursed into (default: the current directory)",
                ),
        ).arg(
            clap::Arg::with_name("dry_run")
                .long("dry-run")
                .short("n")
                .help("Show what we would do, but don't do it"),
        ).arg(
            clap::Arg::with_name("force")
                .long("force")
                .short("f")
                .help("Check files that appear unchanged since last run"),
        ).get_matches()
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

fn print_updated_tags(track: &types::Track) -> () {
    match track.tag_file.tag() {
        Ok(tags) => println!(
            "{}: updated tags: artist: '{}', title: '{}'",
            track.path.display(),
            tags.artist().unwrap_or_default(),
            tags.title().unwrap_or_default()
        ),
        Err(err) => eprintln!(
            "error getting tag info: {}: {:?}",
            track.path.display(),
            err
        ),
    }
}

fn rename_track(track: &types::Track, base_path: &PathBuf, dry_run: bool) {
    let new_path = rename::rename_track(&track, &base_path, dry_run);

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
    let parent = path.clone();
    let parent = parent.parent();
    path.is_file()
        && (force
            || mtime::mtime_def_now(&path) > last_run_time
            || (parent.is_some() && mtime::mtime_def_now(parent.unwrap()) > last_run_time))
}

fn fix_all_tracks(base_path: &PathBuf, dry_run: bool, force: bool) {
    let last_run_time = mtime::get_last_run_time(&base_path).unwrap_or(SystemTime::UNIX_EPOCH);
    let walker = build_music_walker(&base_path).expect("BUG: Error building music walker");

    for result in walker {
        match result {
            Ok(entry) => {
                let path = entry.path().to_path_buf();
                if is_eligible_for_fixing(&path, last_run_time, force) {
                    match track::get_track(path) {
                        Ok(mut track) => {
                            fix_track(&mut track, dry_run);
                            rename_track(&track, &base_path, dry_run);
                        }
                        Err(err) => eprintln!("error: {:?}", err),
                    }
                }
            }
            Err(err) => eprintln!("error: {}", err),
        }
    }

    if !dry_run {
        mtime::set_last_run_time(&base_path).unwrap_or_else(|err| {
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
    for raw_path in args
        .values_of("PATH")
        .expect("BUG: missing default path")
        .collect::<Vec<&str>>()
    {
        let mut path = PathBuf::new();
        path.push(raw_path);
        fix_all_tracks(&path, args.is_present("dry_run"), args.is_present("force"));
    }
}
