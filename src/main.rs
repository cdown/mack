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

fn run_linters(track: Track) -> Result<(), MackError> {
    let mut tags = track.tag_file.tag()?;
    fix_feat(&track);
    Ok(())
}

fn fix_feat(track: &Track) -> Result<(), MackError> {
    let old_title = track.tag_file.tag()?.title();
    let new_title = FEAT_RE.replace_all(&old_title, " (replaced. $artists)");
    println!("{}", new_title);
    Ok(())
}

fn main() {
    let args = clap::App::new("mack")
        .about("The opinionated music library organiser.")
        .arg(clap::Arg::with_name("DIR").index(1))
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
                            run_linters(track);
                        }
                        Err(err) => eprintln!("error: {:?}", err),
                    }
                }
            }
            Err(err) => eprintln!("error: {}", err),
        }
    }
}
