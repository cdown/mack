use types::{MackError, Track};
use std::path::PathBuf;
use taglib::File as TFile;


pub fn get_track(path: PathBuf) -> Result<Track, MackError> {
    let tag_file = TFile::new(path.clone().to_str().ok_or(MackError::InvalidUnicode)?)?;
    Ok(Track {
        path: path,
        tag_file: tag_file,
    })
}
