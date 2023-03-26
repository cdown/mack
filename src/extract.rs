use regex::{Regex, RegexBuilder};
use crate::types::TrackFeat;

const AMP_SPLITS: &[&str] = &[" & ", " and "];

lazy_static! {
    static ref FEAT_RE: Regex =
        RegexBuilder::new(r#" [(\[]?(f(ea)?t[a-z]*\.?|f\.) (?P<feat_artists>[^)\]]+)[)\]]?"#)
            .case_insensitive(true)
            .build()
            .expect("BUG: Invalid regex");
    static ref FEAT_ARTIST_SPLIT: Regex =
        Regex::new(r#", (?:and |& )?"#).expect("BUG: Invalid regex");
}

pub fn extract_feat(title: &str) -> TrackFeat {
    let caps = FEAT_RE.captures(&title);

    match caps {
        Some(caps) => {
            let trimmed = caps["feat_artists"].trim();
            let mut feat_artists: Vec<String> = FEAT_ARTIST_SPLIT
                .split(&trimmed)
                .map(|x| x.to_owned())
                .collect();
            let last_artist = feat_artists
                .last()
                .expect("BUG: captured, but no featured artists")
                .clone();

            // If the last artist contains an "&", we'll split on it, even without a comma. This
            // isn't perfect, but is mostly right.
            for amp_split in AMP_SPLITS {
                if last_artist.contains(amp_split) {
                    let mut tmp_last_split: Vec<String> = last_artist
                        .rsplitn(2, amp_split)
                        .map(|x| x.to_owned())
                        .collect();
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
                original_title: title.to_string(),
            }
        }
        None => {
            // There's no "feat" in here, just return the title whole
            TrackFeat {
                title: title.to_string(),
                featured_artists: Vec::new(),
                original_title: title.to_string(),
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
        assert_eq!(extract_feat(&given), expected);
    }

    #[test]
    fn test_extract_feat_with_feat_single() {
        let given = "A plain title feat. Foo Bar".to_owned();
        let expected = TrackFeat {
            title: "A plain title".to_owned(),
            featured_artists: vec!["Foo Bar".to_owned()],
            original_title: given.clone(),
        };
        assert_eq!(extract_feat(&given), expected);
    }

    #[test]
    fn test_extract_feat_with_feat_double() {
        let given = "A plain title Ft. Foo Bar and Baz Qux".to_owned();
        let expected = TrackFeat {
            title: "A plain title".to_owned(),
            featured_artists: vec!["Foo Bar".to_owned(), "Baz Qux".to_owned()],
            original_title: given.clone(),
        };
        assert_eq!(extract_feat(&given), expected);
    }

    #[test]
    fn test_extract_feat_with_feat_as_f() {
        let given = "A plain title f. Foo Bar and Baz Qux".to_owned();
        let expected = TrackFeat {
            title: "A plain title".to_owned(),
            featured_artists: vec!["Foo Bar".to_owned(), "Baz Qux".to_owned()],
            original_title: given.clone(),
        };
        assert_eq!(extract_feat(&given), expected);
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
        assert_eq!(extract_feat(&given), expected);
    }

    #[test]
    fn test_extract_feat_with_surrounding_whitespace() {
        let given = "A plain title feat    Foo Bar, Baz Qux, and Wibble Wobble    ".to_owned();
        let expected = TrackFeat {
            title: "A plain title".to_owned(),
            featured_artists: vec![
                "Foo Bar".to_owned(),
                "Baz Qux".to_owned(),
                "Wibble Wobble".to_owned(),
            ],
            original_title: given.clone(),
        };
        assert_eq!(extract_feat(&given), expected);
    }
}
