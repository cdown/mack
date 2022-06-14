use std::path::PathBuf;

use id3::{self, TagLike};

/// A representation of an audio file, with meta-data and properties.
pub struct File {
    path: PathBuf,
}

impl File {
    /// Creates a new `taglib::File` for the given `path`.
    pub fn new(path: &PathBuf) -> Result<File, FileError> {
        Ok(File { path: path.clone() })
    }

    /// Returns the `taglib::Tag` instance for the given file.
    pub fn tag(&self) -> Result<Tag, FileError> {
        if let Ok(tag) = id3::Tag::read_from_path(&self.path) {
            Ok(Tag { tag })
        } else {
            Err(FileError::NoAvailableTag)
        }
    }

    /// Updates the meta-data of the file.
    pub fn save(&self) -> bool {
        true
    }
}

#[derive(Debug)]
pub enum FileError {
    /// The file is an invalid or an unrecognized audio container
    InvalidFile,
    /// The file name is invalid
    InvalidFileName,
    /// No meta-data is available
    NoAvailableTag,
    /// No audio properties are available
    NoAvailableAudioProperties,
}

/// The abstract meta-data container for audio files
///
/// Each `Tag` instance can only be created by the `taglib::File::tag()`
/// method.
#[allow(dead_code)]
pub struct Tag {
    tag: id3::Tag,
}

impl Tag {
    /// Returns `Some(TRACK NAME)` or `None` if no track name is present.
    pub fn title(&self) -> Option<String> {
        self.tag.title().and_then(|x| Some(x.to_string()))
    }

    /// Sets the track name.
    pub fn set_title(&mut self, title: &str) {
        self.tag.set_title(title)
    }

    /// Returns `Some(ARTIST NAME)` or `None` if no artist name is present.
    pub fn artist(&self) -> Option<String> {
        self.tag.artist().and_then(|x| Some(x.to_string()))
    }

    /// Sets the artist name.
    pub fn set_artist(&mut self, artist: &str) {
        self.tag.set_artist(artist)
    }

    /// Returns `Some(ALBUM NAME)` or `None` if no album name is present.
    pub fn album(&self) -> Option<String> {
        self.tag.album().and_then(|x| Some(x.to_string()))
    }

    /// Sets the album name.
    pub fn set_album(&mut self, album: &str) {
        self.tag.set_album(album)
    }

    /// Returns `Some(TRACK COMMENT)` or `None` if no track comment is present.
    pub fn comment(&self) -> Option<String> {
        if let Some(frame) = self.tag.get("COMM") {
            if let Some(comment) = frame.content().comment() {
                return Some(comment.text.clone());
            }
        }
        None
    }

    /// Returns `Some(TRACK NUMBER)` or `None` if track number is present.
    pub fn track(&self) -> Option<u32> {
        self.tag.track()
    }

    /// Sets the track number.
    pub fn set_track(&mut self, track: u32) {
        self.tag.set_track(track)
    }
}
