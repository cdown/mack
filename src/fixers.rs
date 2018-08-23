use taglib;
use types::{MackError, Track, TrackFeat};
use extract::extract_feat;

pub fn run_fixers(track: &mut Track, dry_run: bool) -> Result<bool, MackError> {
    let mut tags = track.tag_file.tag()?;

    fixer_is_blacklisted(&tags)?;

    let old_title = extract_feat(tags.title());
    let old_artist = extract_feat(tags.artist());

    let new_title = make_title(&old_title, &old_artist);

    Ok(new_title != old_title.original_title)
}

fn make_title(title: &TrackFeat, artist: &TrackFeat) -> String {
    let mut featured_artists = title.featured_artists.clone();
    featured_artists.extend(artist.featured_artists.clone());

    let new_title = format!("{} (feat. {:?})", title.title, featured_artists);

    println!("{}", new_title);

    new_title
}

fn fixer_is_blacklisted(tags: &taglib::Tag) -> Result<(), MackError> {
    if tags.comment().contains("_NO_MACK") { Err(MackError::Blacklisted) } else { Ok(()) }
}
