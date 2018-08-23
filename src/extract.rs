use regex::{Regex, RegexBuilder};
use taglib;
use types::TrackTitle;

static AMP_SPLIT: &'static str = " & ";

lazy_static! {
    static ref FEAT_RE: Regex = RegexBuilder::new(
        r#" [(\[]?f(ea)?t[a-z]*\.? (?P<feat_artists>[^)\]]+)[)\]]?"#
    ).case_insensitive(true).build().unwrap();
    static ref FEAT_ARTIST_SPLIT: Regex = Regex::new(r#", (?:and|&)?"#).unwrap();
}

pub fn extract_title(tags: &taglib::Tag) -> TrackTitle {
    let title = tags.title();
    let caps = FEAT_RE.captures(&title);

    match caps {
        Some(caps) => {
            let mut feat_artists: Vec<String> =
                FEAT_ARTIST_SPLIT.split(&caps["feat_artists"]).map(|x| x.to_owned()).collect();
            assert!(!feat_artists.is_empty());

            let last_artist =
                feat_artists.last().expect("BUG: despite assert, no featured artists").clone();

            // If the last artist contains an "&", we'll split on it, even without a comma. This
            // isn't perfect, but is mostly right.
            if last_artist.contains(AMP_SPLIT) {
                let mut tmp_last_split: Vec<String> =
                    last_artist.rsplitn(2, AMP_SPLIT).map(|x| x.to_owned()).collect();
                tmp_last_split.reverse();
                feat_artists.pop();
                feat_artists.append(&mut tmp_last_split);
            }


            let featless_title = FEAT_RE.replace_all(&title, "").trim().to_owned();
            TrackTitle {
                title: featless_title,
                featured_artists: feat_artists,
                original_title: title.clone(),
            }
        }
        None => {
            // There's no "feat" in here, just return the title whole
            TrackTitle {
                title: title.clone(),
                featured_artists: Vec::new(),
                original_title: title.clone(),
            }
        }
    }
}
