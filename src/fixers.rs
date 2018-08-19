use regex::{Regex, RegexBuilder};
use taglib;
use types::{Fixer, MackError, Track};
use std::borrow::Cow;

lazy_static! {
    static ref FEAT_RE: Regex = RegexBuilder::new(
        r#" [(\[]?f(ea)?t[a-z]*\.? (?P<artists>[^)\]]+)[)\]]?"#
    ).case_insensitive(true).build().unwrap();
}

pub fn run_fixers(track: &mut Track, dry_run: bool) -> Result<Vec<Fixer>, MackError> {
    let mut applied_fixers = Vec::new();
    let mut tags = track.tag_file.tag()?;

    applied_fixers.push(fix_feat(&mut tags)?);

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

fn normalise_feat<'a>(input: &'a str) -> Cow<'a, str> {
    FEAT_RE.replace_all(input, " (feat. $artists)")
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
                        let title = format!("{} {}{}{} {}{}", title, opening, feat, dot, artist, closing);
                        println!("{}", title);
                        assert_eq!(normalise_feat(&title), exp);
                    }
                }
            }
        }
    }
}
