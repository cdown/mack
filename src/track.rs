use crate::types::Track;
use anyhow::Result;
use id3::Tag;
use std::path::PathBuf;

pub fn get_track(path: PathBuf) -> Result<Track> {
    let tag = Tag::read_from_path(&path)?;
    Ok(Track { path, tag })
}
