extern crate clap;
extern crate ignore;
extern crate taglib;

use std::path::PathBuf;

#[derive(Debug)]
struct TrackMetadata {
    path: PathBuf,
    artist: String,
    album: String,
    title: String,
    track_number: u32,
    year: u32,
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

fn get_track_metadata(path: PathBuf) -> Result<TrackMetadata, MackError> {
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
    let tags = tag_file.tag()?;
    Ok(TrackMetadata {
        path: path,
        artist: tags.artist(),
        album: tags.album(),
        title: tags.title(),
        track_number: tags.track(),
        year: tags.year(),
    })
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
                    println!("{:?}", get_track_metadata(path));
                }
            }
            Err(err) => eprintln!("error: {}", err),
        }
    }
}
