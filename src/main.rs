extern crate clap;
extern crate ignore;
#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate taglib;

use std::path::PathBuf;

mod types;
mod fixers;

fn build_music_walker(dir: &str) -> Result<ignore::Walk, types::MackError> {
    let mut mt_builder = ignore::types::TypesBuilder::new();
    mt_builder.add("music", "*.mp3")?;
    mt_builder.select("music");
    let music_types = mt_builder.build()?;
    Ok(ignore::WalkBuilder::new(dir).types(music_types).build())
}

fn get_track(path: PathBuf) -> Result<types::Track, types::MackError> {
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
    Ok(types::Track {
        path: path,
        tag_file: tag_file,
    })
}

/// We don't intend to print *all* metadata, only ones we might actually try to apply fixes to
fn print_track_info(track: &types::Track) -> () {
    let tags = track.tag_file.tag();

    match tags {
        Ok(tags) => {
            println!("{}:", track.path.display());
            println!("- Album:   {}", tags.album());
            println!("- Artist:  {}", tags.artist());
            println!("- Title:   {}", tags.title());
            println!("- Track #: {}", tags.track());
            println!("- Year:    {}\n", tags.year());
        }
        Err(err) => eprintln!("error printing track info: {}: {:?}", track.path.display(), err),
    }
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
                        Ok(mut track) => {
                            let fix_results =
                                fixers::run_fixers(&mut track, args.is_present("dry_run"));
                            match fix_results {
                                Ok(applied_fixers) => {
                                    if !applied_fixers.is_empty() {
                                        print_track_info(&track);
                                    }
                                }
                                Err(err) => {
                                    eprintln!("error fixing {}: {:?}", track.path.display(), err)
                                }
                            }
                        }
                        Err(err) => eprintln!("error: {:?}", err),
                    }
                }
            }
            Err(err) => eprintln!("error: {}", err),
        }
    }
}
