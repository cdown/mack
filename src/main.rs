extern crate clap;
extern crate failure;
extern crate id3;
extern crate ignore;

use clap::{App, Arg};
//use failure::{err_msg, Error};
use ignore::WalkBuilder;
use ignore::types::{Types, TypesBuilder};

fn get_supported_music_types() -> Result<Types, ignore::Error> {
    let mut ig_types = TypesBuilder::new();
    ig_types.add("music", "*.mp3")?;
    ig_types.select("music");
    ig_types.build()
}

fn main() {
    let args = App::new("mack")
        .about("The opinionated music library organiser.")
        .arg(Arg::with_name("DIR").index(1))
        .get_matches();

    let music_types = get_supported_music_types().expect("Failed to build music type map");
    let walker = WalkBuilder::new(args.value_of("DIR").unwrap_or(".")).types(music_types).build();

    for result in walker {
        match result {
            Ok(entry) => {
                if entry.path().is_file() {
                    println!("{}", entry.path().display());
                }
            }
            Err(err) => eprintln!("error: {}", err),
        }
    }
}
