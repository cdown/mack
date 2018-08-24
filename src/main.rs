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
        .arg(clap::Arg::with_name("PATH").multiple(true).default_value(".").help(
            "Paths to fix, directories are recursed into (default: the current directory)",
        ))
        .arg(clap::Arg::with_name("dry_run").long("dry-run").short("n").help(
            "Show what we would do, but don't do it",
        ))
        .get_matches()
}

fn fix_track(mut track: types::Track, dry_run: bool) {
    let fix_results = fixers::run_fixers(&mut track, dry_run);
    match fix_results {
        Ok(applied_fixers) => {
            if applied_fixers {
                track::print_track_info(&track);
            }
        }
        Err(err) => eprintln!("cannot fix {}: {:?}", track.path.display(), err),
    }

}

fn main() {
    let args = parse_args();

    for path in args.values_of("PATH").expect("BUG: missing default path").collect::<Vec<&str>>() {
        let walker = build_music_walker(path).expect("BUG: Error building music walker");
        for result in walker {
            match result {
                Ok(entry) => {
                    let path = entry.path().to_path_buf();
                    if path.is_file() {
                        match track::get_track(path) {
                            Ok(mut track) => fix_track(track, args.is_present("dry_run")),
                            Err(err) => eprintln!("error: {:?}", err),
                        }
                    }
                }
                Err(err) => eprintln!("error: {}", err),
            }
        }
    }
}
