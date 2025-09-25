use once_cell::sync::Lazy;
use regex::Regex;

const FEAT_KEYWORDS: &[&str] = &["feat", "ft", "f.", "featuring"];

/// Represents a track's title after extracting featured artists.
#[derive(Debug, PartialEq, Eq)]
pub struct TrackFeat {
    pub title: String,
    pub featured_artists: Vec<String>,
    pub original_title: String,
}

/// Holds the indices and inner content of a bracketed feature section.
struct BracketedFeat<'a> {
    open_idx: usize,
    close_idx: usize,
    content: &'a str,
}

/// Extracts featured artist information from a track title.
pub fn extract_feat(title: &str) -> TrackFeat {
    if let Some(bf) = find_bracketed_feat(title, FEAT_KEYWORDS) {
        let artist_part = remove_keyword_from_content(bf.content, FEAT_KEYWORDS);
        let featured_artists = split_artists(artist_part);
        let base_title = format!(
            "{} {}",
            &title[..bf.open_idx].trim_end(),
            &title[bf.close_idx + 1..].trim_start()
        )
        .trim()
        .to_string();

        return TrackFeat {
            title: base_title,
            featured_artists,
            original_title: title.to_string(),
        };
    }

    if let Some(pos) = find_non_bracketed_feat(title) {
        let inner = &title[pos..].trim();
        let artist_part = remove_keyword_from_content(inner, FEAT_KEYWORDS);
        let featured_artists = split_artists(artist_part);
        let base_title = title[..pos].trim_end().to_string();
        return TrackFeat {
            title: base_title,
            featured_artists,
            original_title: title.to_string(),
        };
    }

    // No feature found, return as is
    TrackFeat {
        title: title.to_string(),
        featured_artists: Vec::new(),
        original_title: title.to_string(),
    }
}

/// Searches for a bracketed section (using '(' or '[') whose inner content, when trimmed
/// and lowercased, starts with one of the specified keywords. If found, returns a `BracketedFeat`
/// containing the opening index, closing index, and the inner content.
fn find_bracketed_feat<'a>(title: &'a str, keywords: &[&str]) -> Option<BracketedFeat<'a>> {
    let mut chars = title.char_indices().peekable();
    while let Some((i, ch)) = chars.next() {
        if ch == '(' || ch == '[' {
            let closing = if ch == '(' { ')' } else { ']' };
            let mut depth = 1;
            let mut j = i;
            for (k, ch2) in chars.by_ref() {
                if ch2 == ch {
                    depth += 1;
                } else if ch2 == closing {
                    depth -= 1;
                    if depth == 0 {
                        j = k;
                        break;
                    }
                }
            }
            if depth == 0 {
                // Get the inner content (without the brackets).
                let content = &title[i + ch.len_utf8()..j];
                if keywords
                    .iter()
                    .any(|&kw| content.trim().to_lowercase().starts_with(kw))
                {
                    return Some(BracketedFeat {
                        open_idx: i,
                        close_idx: j,
                        content,
                    });
                }
            }
        }
    }
    None
}

/// Searches for a non-bracketed occurrence of any feature keyword (case–insensitively)
/// in the title. Returns the earliest index if found.
fn find_non_bracketed_feat(title: &str) -> Option<usize> {
    static FEAT_RE: Lazy<Regex> = Lazy::new(|| {
        let escaped_keywords: Vec<String> =
            FEAT_KEYWORDS.iter().map(|&s| regex::escape(s)).collect();

        let pattern = format!(r"(?i)(?:^|\W)({})(:?$|\W)", escaped_keywords.join("|"));
        Regex::new(&pattern).expect("BUG: Invalid feat regex")
    });

    FEAT_RE
        .captures(title)
        .and_then(|caps| caps.get(1))
        .map(|m| m.start())
}

/// Given some content (either from inside a bracket or not) that starts with a feature keyword,
/// remove that keyword (choosing the longest matching one) and any immediately following period.
fn remove_keyword_from_content<'a>(content: &'a str, keywords: &[&str]) -> &'a str {
    let trimmed = content.trim();
    let lower = trimmed.to_lowercase();
    if let Some(&kw) = keywords
        .iter()
        .filter(|&&k| lower.starts_with(k))
        .max_by_key(|k| k.len())
    {
        let after = trimmed.get(kw.len()..).unwrap_or("").trim_start();
        if let Some(rest) = after.strip_prefix('.') {
            rest.trim_start()
        } else {
            after
        }
    } else {
        trimmed
    }
}

fn split_artists(artists_str: &str) -> Vec<String> {
    artists_str
        .split(',')
        .flat_map(|s| s.split(" and ").flat_map(|s| s.split(" & ")))
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect()
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
    fn test_extract_feat_no_feat_in_word() {
        let given = "Lift Method".to_owned();
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

    #[test]
    fn test_extract_feat_nested_parentheses() {
        let given = "比較大的大提琴 [Featuring Lara Veronin (梁心頤) & Gary Yang (楊瑞代)]";
        let expected = TrackFeat {
            title: "比較大的大提琴".to_string(),
            featured_artists: vec![
                "Lara Veronin (梁心頤)".to_string(),
                "Gary Yang (楊瑞代)".to_string(),
            ],
            original_title: given.to_string(),
        };
        assert_eq!(extract_feat(given), expected);
    }
}
