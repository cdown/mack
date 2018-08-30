use std::path::PathBuf;
use taglib::File as TFile;
use types::{MackError, Track};

pub fn get_track(path: PathBuf) -> Result<Track, MackError> {
    let tag_file = TFile::new(&path)?;
    Ok(Track { path, tag_file })
}
