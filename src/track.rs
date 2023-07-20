use crate::types::Track;
use anyhow::Result;
use audiotags::Tag;
use std::path::PathBuf;

pub fn get_track(path: PathBuf) -> Result<Track> {
    let tag = Tag::new().read_from_path(&path)?;
    Ok(Track { path, tag })
}
