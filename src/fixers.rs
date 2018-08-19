use regex::{Regex, RegexBuilder};
use taglib;
use types::{Fixer, MackError, Track};

lazy_static! {
    static ref FEAT_RE: Regex = RegexBuilder::new(
        r#" [(\[]?feat[^.]*\. (?P<artists>[^)]+)[)\]]?"#
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
    let new_title = FEAT_RE.replace_all(&old_title, " (feat. $artists)");
    if old_title != new_title {
        tags.set_title(&new_title);
        return Ok(Some(Fixer::FEAT));
    }
    Ok(None)
}
