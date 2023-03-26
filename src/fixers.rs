use crate::extract::extract_feat;
use crate::types::{Track, TrackFeat};
use anyhow::{bail, Result};
use lazy_static::lazy_static;
use regex::Regex;
use taglib::Tag;

lazy_static! {
    static ref MULTI_WS_RE: Regex = Regex::new(r#"[ \t]+"#).expect("BUG: Invalid regex");
}

pub fn run_fixers(track: &mut Track, dry_run: bool) -> Result<bool> {
    let mut tags = track
        .tag_file
        .tag()
        .map_err(|_| anyhow::Error::msg("Failed to get tag"))?;

    fixer_is_blacklisted(&tags)?;

    let new_title = fix_title(tags.title(), tags.artist());
    let new_artist = fix_artist(tags.artist());
    let new_album = fix_album(tags.album());
    let mut changed = false;

    if let Some(new_artist) = new_artist {
        changed = true;
        tags.set_artist(&new_artist);
    }
    if let Some(new_title) = new_title {
        changed = true;
        tags.set_title(&new_title);
    }
    if let Some(new_album) = new_album {
        changed = true;
        tags.set_album(&new_album);
    }

    if !dry_run && changed {
        track.tag_file.save();
    }

    Ok(changed)
}

fn normalise_field(field: &str) -> String {
    let mut new_field = field.to_owned();

    // Optimisation: Most titles don't have multiple whitespaces. Don't even try to replace with
    // MULTI_WS_RE if we can't find two spaces together.
    if new_field.contains("  ") {
        new_field = MULTI_WS_RE.replace_all(&new_field, " ").to_string()
    }

    new_field = new_field.trim().to_owned();
    new_field
        .replace('[', "(")
        .replace(']', ")")
        .replace('…', "...")
        .replace(['“', '”'], "\"")
        .replace(['‘', '’'], "'")
}

fn fix_artist(old_artist: impl Into<Option<String>>) -> Option<String> {
    let field = normalise_field(&old_artist.into().unwrap_or_default());
    let artist = extract_feat(&field);
    if artist.title != artist.original_title {
        Some(artist.title)
    } else {
        None
    }
}

fn fix_album(old_album: impl Into<Option<String>>) -> Option<String> {
    let old_album = match old_album.into() {
        Some(old_album) => old_album,
        None => return None,
    };
    let new_album = normalise_field(&old_album);

    if new_album != old_album {
        Some(new_album)
    } else {
        None
    }
}

fn fix_title(
    old_title: impl Into<Option<String>>,
    old_artist: impl Into<Option<String>>,
) -> Option<String> {
    let old_title = match old_title.into() {
        Some(old_title) => old_title,
        None => return None,
    };

    let old_title = extract_feat(&old_title);
    let old_artist = extract_feat(&old_artist.into().unwrap_or_default());

    let new_title = make_title(&old_title, &old_artist);

    if new_title != old_title.original_title {
        Some(new_title)
    } else {
        None
    }
}

fn make_title(title: &TrackFeat, artist: &TrackFeat) -> String {
    let mut featured_artists = title.featured_artists.clone();
    featured_artists.extend(artist.featured_artists.clone());

    let mut new_title = title.title.clone();
    if !featured_artists.is_empty() {
        let feat_artists_string = make_feat_string(&featured_artists);
        let feat_string = format!(" (feat. {})", feat_artists_string);
        new_title.push_str(&feat_string);
    }

    normalise_field(&new_title)
}

fn make_feat_string(featured_artists: &[String]) -> String {
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

fn fixer_is_blacklisted(tags: &Tag) -> Result<()> {
    if tags.comment().unwrap_or_default().contains("_NO_MACK") {
        bail!("File contains _NO_MACK");
    } else {
        Ok(())
    }
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
    fn test_fix_title_with_title_feat_no_artist_feat_and_brackets() {
        let given_title = "Foo Bar (feat. Wibble Wobble) [Richard Stallman mix]".to_owned();
        let given_artist = "Baz Qux".to_owned();
        let expected = Some("Foo Bar (Richard Stallman mix) (feat. Wibble Wobble)".to_owned());
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

    #[test]
    fn test_fix_title_with_title_feat_smart_quotes() {
        let given_title = "Foo ‘Bar’ (feat. Wibble “Wabble” Wobble)".to_owned();
        let given_artist = "Baz Qux".to_owned();
        let expected = Some("Foo 'Bar' (feat. Wibble \"Wabble\" Wobble)".to_owned());
        assert_eq!(fix_title(given_title, given_artist), expected);
    }

    #[test]
    fn test_fix_whitespace() {
        let given = "    Foo Bar [feat.    Baz    Qux   ]    ".to_owned();
        let expected = Some("Foo Bar (feat. Baz Qux)".to_owned());
        assert_eq!(fix_title(given, None), expected);
    }
}
