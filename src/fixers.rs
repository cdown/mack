use taglib;
use types::{Fixer, MackError, Track};
use extract::extract_title;

pub fn run_fixers(track: &mut Track, dry_run: bool) -> Result<Vec<Fixer>, MackError> {
    let mut applied_fixers = Vec::new();
    let mut tags = track.tag_file.tag()?;

    fixer_is_blacklisted(&tags)?;

    println!("{:?}", extract_title(&tags));

    // TODO: rework fixers to use TrackTitle and ilk
    applied_fixers.push(None);

    let applied_fixers: Vec<Fixer> = applied_fixers.into_iter().flat_map(|x| x).collect();

    if !dry_run && !applied_fixers.is_empty() {
        track.tag_file.save();
    }

    Ok(applied_fixers)
}

fn fixer_is_blacklisted(tags: &taglib::Tag) -> Result<(), MackError> {
    if tags.comment().contains("_NO_MACK") { Err(MackError::Blacklisted) } else { Ok(()) }
}
