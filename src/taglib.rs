use std::path::PathBuf;

/// A representation of an audio file, with meta-data and properties.
pub struct File {
	path: PathBuf,
}

impl File {
	/// Creates a new `taglib::File` for the given `path`.
	pub fn new(path: &PathBuf) -> Result<File, FileError> {
		Ok(File {
			path: path.clone()
		})
	}

	/// Returns the `taglib::Tag` instance for the given file.
	pub fn tag(&self) -> Result<Tag, FileError> {
		Ok(Tag {
			path: &self.path,
		})
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
pub struct Tag<'a> {
	path: &'a PathBuf,
}

impl Tag<'_> {
	/// Returns the track name, or an empty string if no track name is present.
	pub fn title(&self) -> Option<String> {
		Some(format!("Title of '{:?}'", self.path))
	}

	/// Sets the track name.
	pub fn set_title(&mut self, title: &str) {}

	/// Returns the artist name, or an empty string if no artist name is present.
	pub fn artist(&self) -> Option<String> {
		Some(format!("Artist of '{:?}'", self.path))
	}

	/// Sets the artist name.
	pub fn set_artist(&mut self, artist: &str) {}

	/// Returns the album name, or an empty string if no album name is present.
	pub fn album(&self) -> Option<String> {
		Some(format!("Album of '{:?}'", self.path))
	}

	/// Sets the album name.
	pub fn set_album(&mut self, album: &str) {}

	/// Returns the track comment, or an empty string if no track comment is
	/// present.
	pub fn comment(&self) -> Option<String> {
		Some(format!("Comment of '{:?}'", self.path))
	}

	/// Sets the track comment.
	pub fn set_comment(&mut self, comment: &str) {}

	/// Returns the track number, or 0 if no track number is present.
	pub fn track(&self) -> Option<u32> {
		Some(2)
	}

	/// Sets the track number.
	pub fn set_track(&mut self, track: u32) {}
}
