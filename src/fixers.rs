use regex::{Regex, RegexBuilder};
use taglib;
use types::{Fixer, MackError, Track};
use std::borrow::Cow;

lazy_static! {
    static ref FEAT_RE: Regex = RegexBuilder::new(
        r#" [(\[]?f(ea)?t[a-z]*\.? (?P<artists>[^)\]]+)[)\]]?"#
    ).case_insensitive(true).build().unwrap();
    static ref MULTI_WS_RE: Regex = Regex::new(r#"[ \t]+"#).unwrap();
}

pub fn run_fixers(track: &mut Track, dry_run: bool) -> Result<Vec<Fixer>, MackError> {
    let mut applied_fixers = Vec::new();
    let mut tags = track.tag_file.tag()?;

    applied_fixers.push(fix_feat(&mut tags)?);
    applied_fixers.push(fix_tag_whitespace(&mut tags)?);

    let applied_fixers: Vec<Fixer> = applied_fixers.into_iter().flat_map(|x| x).collect();

    if !dry_run && !applied_fixers.is_empty() {
        track.tag_file.save();
    }

    Ok(applied_fixers)
}

fn fix_feat(tags: &mut taglib::Tag) -> Result<Option<Fixer>, MackError> {
    let old_title = tags.title();
    let new_title = normalise_feat(&old_title);
    if old_title != new_title {
        tags.set_title(&new_title);
        return Ok(Some(Fixer::FEAT));
    }
    Ok(None)
}

fn fix_tag_whitespace(tags: &mut taglib::Tag) -> Result<Option<Fixer>, MackError> {
    // TODO(#15): Make this DRY. This had been intended to be implemented in a fairly
    // non-repetitive way using closures, but that has some complications, see the issue.
    let mut any_change = None;

    let old_title = tags.title();
    let new_title = normalise_whitespace(&old_title);
    if old_title != new_title {
        tags.set_title(&new_title);
        any_change = Some(Fixer::WHITESPACE);
    }

    let old_artist = tags.artist();
    let new_artist = normalise_whitespace(&old_artist);
    if old_artist != new_artist {
        tags.set_artist(&new_artist);
        any_change = Some(Fixer::WHITESPACE);
    }

    let old_album = tags.album();
    let new_album = normalise_whitespace(&old_album);
    if old_album != new_album {
        tags.set_album(&new_album);
        any_change = Some(Fixer::WHITESPACE);
    }

    Ok(any_change)
}

fn normalise_feat<'a>(input: &'a str) -> Cow<'a, str> {
    FEAT_RE.replace_all(input, " (feat. $artists)")
}

fn normalise_whitespace<'a>(input: &'a str) -> String {
    MULTI_WS_RE.replace_all(input, " ").trim().to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalise_feat() {
        let title = "我是笨蛋";
        let artist = "傻瓜";

        // Non-feat is passed through with no change
        let exp = title;
        assert_eq!(normalise_feat(exp), exp);

        let exp = format!("{} (feat. {})", title, artist);
        assert_eq!(normalise_feat(&exp), exp);

        for feat in ["featuring", "feat", "ft"].iter() {
            for opening in ["", "(", "["].iter() {
                for closing in ["", ")", "]"].iter() {
                    for dot in ["", "."].iter() {
                        let title =
                            format!("{} {}{}{} {}{}", title, opening, feat, dot, artist, closing);
                        println!("{}", title);
                        assert_eq!(normalise_feat(&title), exp);
                    }
                }
            }
        }
    }

    #[test]
    fn test_normalise_whitespace() {
        // Without extraneous whitespace is passed through with no change
        assert_eq!(normalise_whitespace("foo bar"), "foo bar");
        assert_eq!(normalise_whitespace(" foo  bar "), "foo bar");
    }
}
