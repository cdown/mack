extern crate clap;
extern crate ignore;
extern crate taglib;
extern crate regex;
#[macro_use]
extern crate lazy_static;

use regex::{Regex, RegexBuilder};
use std::path::PathBuf;
use std::collections::HashMap;

lazy_static! {
    static ref FEAT_RE: Regex = RegexBuilder::new(
        r#" [(\[]?feat[^.]*\. (?P<artists>[^)]+)[)\]]?"#
    ).case_insensitive(true).build().unwrap();
}

struct Track {
    path: PathBuf,
    tag_file: taglib::File,
}

#[derive(Debug)]
enum Fixer {
    FEAT,
}

#[derive(Debug)]
enum MackError {
    Tag(taglib::FileError),
    Ignore(ignore::Error),
}

impl From<taglib::FileError> for MackError {
    fn from(err: taglib::FileError) -> MackError {
        MackError::Tag(err)
    }
}

impl From<ignore::Error> for MackError {
    fn from(err: ignore::Error) -> MackError {
        MackError::Ignore(err)
    }
}

fn build_music_walker(dir: &str) -> Result<ignore::Walk, MackError> {
    let mut mt_builder = ignore::types::TypesBuilder::new();
    mt_builder.add("music", "*.mp3")?;
    mt_builder.select("music");
    let music_types = mt_builder.build()?;
    Ok(ignore::WalkBuilder::new(dir).types(music_types).build())
}

fn get_track(path: PathBuf) -> Result<Track, MackError> {
    let tl_path = path.clone();
    let file = match tl_path.to_str() {
        Some(file) => file,
        None => {
            let msg =
                format!("Path {:?} contains non-Unicode. This is not supported, so bailing.", path);
            panic!(msg);
        }
    };

    let tag_file = taglib::File::new(file)?;
    Ok(Track {
        path: path,
        tag_file: tag_file,
    })
}

fn run_fixers(track: Track, dry_run: bool) -> Result<Vec<Fixer>, MackError> {
    let mut applied_fixers = Vec::new();
    let mut tags = track.tag_file.tag()?;

    applied_fixers.push(fix_feat(&mut tags)?);

    let applied_fixers: Vec<Fixer> = applied_fixers.into_iter().flat_map(|x| x).collect();

    if !dry_run && !applied_fixers.is_empty() {
        track.tag_file.save();
    }

    Ok(applied_fixers)
}

fn fix_feat(tags: &mut taglib::Tag) -> Result<Option<Fixer>, MackError> {
    let old_title = tags.title();
    let new_title = FEAT_RE.replace_all(&old_title, " (feat. $artists)");
    if old_title != new_title {
        tags.set_title(&new_title);
        return Ok(Some(Fixer::FEAT));
    }
    Ok(None)
}

fn main() {
    let args = clap::App::new("mack")
        .about("The opinionated music library organiser.")
        .arg(clap::Arg::with_name("DIR").index(1))
        .arg(clap::Arg::with_name("dry_run").long("dry-run").short("n"))
        .get_matches();

    let walker = build_music_walker(args.value_of("DIR").unwrap_or(".")).expect(
        "Error building music walker",
    );
    for result in walker {
        match result {
            Ok(entry) => {
                let path = entry.path().to_path_buf();
                if path.is_file() {
                    match get_track(path) {
                        Ok(track) => {
                            {
                                let tags = track.tag_file.tag().expect("Failed to get tags");
                                println!("{} {} {}", tags.artist(), tags.album(), tags.title());
                            }
                            let applied_fixers = run_fixers(track, args.is_present("dry_run"));
                            println!("{:?}", applied_fixers);
                        }
                        Err(err) => eprintln!("error: {:?}", err),
                    }
                }
            }
            Err(err) => eprintln!("error: {}", err),
        }
    }
}
