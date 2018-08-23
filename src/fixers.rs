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

    let mut new_title = title.title.clone();
    if !featured_artists.is_empty() {
        let feat_artists_string = make_feat_string(featured_artists);
        let feat_string = format!(" (feat. {})", feat_artists_string);
        new_title.push_str(&feat_string);
    }

    println!("{}", new_title);

    new_title
}

fn make_feat_string(featured_artists: Vec<String>) -> String {
    let mut output = "".to_owned();
    let mut artist_idx = 1;

    let mut artists = featured_artists.iter().peekable();

    while let Some(artist) = artists.next() {
        let is_last = artists.peek().is_none();

        if is_last && artist_idx > 2 {
            output.push_str(", and ");
        } else if is_last && artist_idx == 2 {
            output.push_str(" and ");
        } else if artist_idx != 1 {
            output.push_str(", ");
        }
        output.push_str(artist);
        artist_idx += 1;
    }

    output
}

fn fixer_is_blacklisted(tags: &taglib::Tag) -> Result<(), MackError> {
    if tags.comment().contains("_NO_MACK") { Err(MackError::Blacklisted) } else { Ok(()) }
}
