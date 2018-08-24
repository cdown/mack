use regex::{Regex, RegexBuilder};
use types::TrackFeat;

const AMP_SPLITS: &'static [&'static str] = &[" & ", " and "];

lazy_static! {
    static ref FEAT_RE: Regex = RegexBuilder::new(
        r#" [(\[]?f(ea)?t[a-z]*\.? (?P<feat_artists>[^)\]]+)[)\]]?"#
    ).case_insensitive(true).build().expect("BUG: Invalid regex");
    static ref FEAT_ARTIST_SPLIT: Regex = Regex::new(
        r#", (?:and |& )?"#
    ).expect("BUG: Invalid regex");
}

pub fn extract_feat(title: String) -> TrackFeat {
    let caps = FEAT_RE.captures(&title);

    match caps {
        Some(caps) => {
            let mut feat_artists: Vec<String> =
                FEAT_ARTIST_SPLIT.split(&caps["feat_artists"]).map(|x| x.to_owned()).collect();
            assert!(!feat_artists.is_empty());

            let last_artist =
                feat_artists.last().expect("BUG: assert failed to ensure featured artists").clone();

            // If the last artist contains an "&", we'll split on it, even without a comma. This
            // isn't perfect, but is mostly right.
            for amp_split in AMP_SPLITS {
                if last_artist.contains(amp_split) {
                    let mut tmp_last_split: Vec<String> =
                        last_artist.rsplitn(2, amp_split).map(|x| x.to_owned()).collect();
                    tmp_last_split.reverse();
                    feat_artists.pop();
                    feat_artists.append(&mut tmp_last_split);
                    break;
                }
            }


            let featless_title = FEAT_RE.replace_all(&title, "").trim().to_owned();
            TrackFeat {
                title: featless_title,
                featured_artists: feat_artists,
                original_title: title.clone(),
            }
        }
        None => {
            // There's no "feat" in here, just return the title whole
            TrackFeat {
                title: title.clone(),
                featured_artists: Vec::new(),
                original_title: title.clone(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_feat_no_feat() {
        let given = "A plain title".to_owned();
        let expected = TrackFeat {
            title: given.clone(),
            featured_artists: Vec::new(),
            original_title: given.clone(),
        };
        assert_eq!(extract_feat(given), expected);
    }

    #[test]
    fn test_extract_feat_with_feat_single() {
        let given = "A plain title feat. Foo Bar".to_owned();
        let expected = TrackFeat {
            title: "A plain title".to_owned(),
            featured_artists: vec!["Foo Bar".to_owned()],
            original_title: given.clone(),
        };
        assert_eq!(extract_feat(given), expected);
    }

    #[test]
    fn test_extract_feat_with_feat_double() {
        let given = "A plain title Ft. Foo Bar and Baz Qux".to_owned();
        let expected = TrackFeat {
            title: "A plain title".to_owned(),
            featured_artists: vec!["Foo Bar".to_owned(), "Baz Qux".to_owned()],
            original_title: given.clone(),
        };
        assert_eq!(extract_feat(given), expected);
    }

    #[test]
    fn test_extract_feat_with_feat_many() {
        let given = "A plain title feat Foo Bar, Baz Qux, and Wibble Wobble".to_owned();
        let expected = TrackFeat {
            title: "A plain title".to_owned(),
            featured_artists: vec![
                "Foo Bar".to_owned(),
                "Baz Qux".to_owned(),
                "Wibble Wobble".to_owned(),
            ],
            original_title: given.clone(),
        };
        assert_eq!(extract_feat(given), expected);
    }
}
