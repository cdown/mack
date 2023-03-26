use std::path::PathBuf;
use taglib::File as TFile;

pub struct Track {
    pub path: PathBuf,
    pub tag_file: TFile,
}

#[derive(Debug, PartialEq)]
pub struct TrackFeat {
    pub title: String,
    pub featured_artists: Vec<String>,
    pub original_title: String,
}
