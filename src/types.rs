use std::io;
use std::path::PathBuf;
use taglib::{File as TFile, FileError as TFileError};

pub struct Track {
    pub path: PathBuf,
    pub tag_file: TFile,
}

#[derive(Debug)]
pub enum MackError {
    Tag(TFileError),
    Io(io::Error),
    Blacklisted,
    WouldMoveToFsRoot,
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

impl From<io::Error> for MackError {
    fn from(err: io::Error) -> MackError {
        MackError::Io(err)
    }
}
