pub mod feat;
pub mod fixers;
pub mod loader;
pub mod rename;

pub use loader::get_track;

use id3::Tag;
use std::path::PathBuf;

/// Represents a music track with its file path and associated ID3 tag.
pub struct Track {
    pub path: PathBuf,
    pub tag: Tag,
}
