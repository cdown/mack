extern crate clap;
extern crate ignore;
#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate taglib;

mod fixers;
mod track;
mod types;
mod extract;

fn build_music_walker(dir: &str) -> Result<ignore::Walk, types::MackError> {
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
                    match track::get_track(path) {
                        Ok(mut track) => {
                            let fix_results =
                                fixers::run_fixers(&mut track, args.is_present("dry_run"));
                            match fix_results {
                                Ok(applied_fixers) => {
                                    if applied_fixers {
                                        track::print_track_info(&track);
                                    }
                                }
                                Err(err) => {
                                    eprintln!("cannot fix {}: {:?}", track.path.display(), err)
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
