use ignore;
use std::path::PathBuf;
use taglib::{File as TFile, FileError as TFileError};

pub struct Track {
    pub path: PathBuf,
    pub tag_file: TFile,
}

#[derive(Debug)]
pub enum MackError {
    Tag(TFileError),
    Ignore(ignore::Error),
    Blacklisted,
    InvalidUnicode,
}

#[derive(Debug, PartialEq)]
pub struct TrackFeat {
    pub title: String,
    pub featured_artists: Vec<String>,
    pub original_title: String,
}

impl From<TFileError> for MackError {
    fn from(err: TFileError) -> MackError {
        MackError::Tag(err)
    }
}

impl From<ignore::Error> for MackError {
    fn from(err: ignore::Error) -> MackError {
        MackError::Ignore(err)
    }
}
