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
}

#[derive(Debug)]
pub enum MackError {
    Tag(taglib::FileError),
    Ignore(ignore::Error),
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
