use taglib;
use regex::Regex;
use types::{MackError, Track, TrackFeat};
use extract::extract_feat;

lazy_static! {
    static ref MULTI_WS_RE: Regex = Regex::new(r#"[ \t]+"#).expect("BUG: Invalid regex");
}

pub fn run_fixers(track: &mut Track, _dry_run: bool) -> Result<bool, MackError> {
    let tags = track.tag_file.tag()?;

    fixer_is_blacklisted(&tags)?;

    let new_title = fix_title(tags.title(), tags.artist());
    let new_artist = fix_artist(tags.artist());

    Ok(new_title.is_some() || new_artist.is_some())
}

fn fix_artist(old_artist: String) -> Option<String> {
    let artist = extract_feat(old_artist);
    if artist.title != artist.original_title { Some(artist.title) } else { None }
}

fn fix_title(old_title: String, old_artist: String) -> Option<String> {
    let old_title = extract_feat(old_title);
    let old_artist = extract_feat(old_artist);

    let new_title = make_title(&old_title, &old_artist);

    if new_title != old_title.original_title { Some(new_title) } else { None }
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

    MULTI_WS_RE.replace_all(&new_title, " ").trim().to_owned()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fix_artist_no_feat() {
        let given = "Foo Bar".to_owned();
        let expected = None;
        assert_eq!(fix_artist(given), expected);
    }

    #[test]
    fn test_fix_artist_with_feat() {
        let given = "Foo Bar (feat. Baz Qux)".to_owned();
        let expected = Some("Foo Bar".to_owned());
        assert_eq!(fix_artist(given), expected);
    }

    #[test]
    fn test_fix_title_no_title_feat_no_artist_feat() {
        let given_title = "Foo Bar".to_owned();
        let given_artist = "Baz Qux".to_owned();
        let expected = None;
        assert_eq!(fix_title(given_title, given_artist), expected);
    }

    #[test]
    fn test_fix_title_with_title_feat_no_artist_feat() {
        let given_title = "Foo Bar (feat. Wibble Wobble)".to_owned();
        let given_artist = "Baz Qux".to_owned();
        let expected = None;
        assert_eq!(fix_title(given_title, given_artist), expected);
    }

    #[test]
    fn test_fix_title_no_title_feat_with_artist_feat() {
        let given_title = "Foo Bar".to_owned();
        let given_artist = "Baz Qux feat. Fizz Buzz".to_owned();
        let expected = Some("Foo Bar (feat. Fizz Buzz)".to_owned());
        assert_eq!(fix_title(given_title, given_artist), expected);
    }

    #[test]
    fn test_fix_title_with_title_feat_and_artist_feat() {
        let given_title = "Foo Bar (feat. Wibble Wobble)".to_owned();
        let given_artist = "Baz Qux feat. Fizz Buzz".to_owned();
        let expected = Some("Foo Bar (feat. Wibble Wobble and Fizz Buzz)".to_owned());
        assert_eq!(fix_title(given_title, given_artist), expected);
    }
}
