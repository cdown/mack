extern crate clap;
extern crate failure;
extern crate id3;
extern crate ignore;

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
                if entry.path().is_file() {
                    println!("{}", entry.path().display());
                }
            }
            Err(err) => eprintln!("error: {}", err),
        }
    }
}
