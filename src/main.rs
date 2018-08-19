extern crate clap;
extern crate failure;
extern crate ignore;
extern crate taglib;

struct TrackMetadata {
    artist: String,
    album: String,
    title: String,
    track_number: u32,
    year: u32,
}

fn build_music_walker(dir: &str) -> Result<ignore::Walk, ignore::Error> {
    let mut mt_builder = ignore::types::TypesBuilder::new();
    mt_builder.add("music", "*.mp3")?;
    mt_builder.select("music");
    let music_types = mt_builder.build()?;
    Ok(ignore::WalkBuilder::new(dir).types(music_types).build())
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
                let path = entry.path();
                if path.is_file() {
                    let file = path.to_str().unwrap();
                    let tag_file = taglib::File::new(file).unwrap();
                    let tag = tag_file.tag().unwrap();
                    println!("{}", tag.artist());
                }
            }
            Err(err) => eprintln!("error: {}", err),
        }
    }
}
