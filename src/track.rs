use crate::types::Track;
use anyhow::Result;
use std::path::PathBuf;
use taglib::File as TFile;

pub fn get_track(path: PathBuf) -> Result<Track> {
    let tag_file = TFile::new(&path).map_err(|_| anyhow::Error::msg("Failed to get tag"))?;
    Ok(Track { path, tag_file })
}
