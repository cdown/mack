use id3::Tag;
use std::path::PathBuf;

pub struct Track {
    pub path: PathBuf,
    pub tag: Tag,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TrackFeat {
    pub title: String,
    pub featured_artists: Vec<String>,
    pub original_title: String,
}
