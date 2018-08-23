use ignore;
use std::path::PathBuf;
use taglib;

pub struct Track {
    pub path: PathBuf,
    pub tag_file: taglib::File,
}

#[derive(Debug)]
pub enum Fixer {
    FEAT,
    WHITESPACE,
}

#[derive(Debug)]
pub enum MackError {
    Tag(taglib::FileError),
    Ignore(ignore::Error),
    Blacklisted,
}

#[derive(Debug)]
pub struct TrackFeat {
    pub title: String,
    pub featured_artists: Vec<String>,
    pub original_title: String,
}

impl From<taglib::FileError> for MackError {
    fn from(err: taglib::FileError) -> MackError {
        MackError::Tag(err)
    }
}

impl From<ignore::Error> for MackError {
    fn from(err: ignore::Error) -> MackError {
        MackError::Ignore(err)
    }
}
