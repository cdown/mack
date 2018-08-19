extern crate ignore;
extern crate id3;
extern crate clap;

use ignore::Walk;
use clap::{App, Arg};

fn main() {
    let args = App::new("mack")
        .about("The opinionated music library organiser.")
        .arg(Arg::with_name("DIR").index(1))
        .get_matches();

    for result in Walk::new(args.value_of("DIR").unwrap_or(".")) {
        match result {
            Ok(entry) => println!("{}", entry.path().display()),
            Err(err) => println!("ERROR: {}", err),
        }
    }
}
