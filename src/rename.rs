use types::{MackError, Track};
use std::path::PathBuf;

/// TODO: Currently only filters out names guaranteed to be incompatible with POSIX filesystems
fn sanitise_path_part(path_part: String) -> String {
    path_part.replace("\0", "").replace("/", "_")
}

/// artist/album/2digitnum title.ext
pub fn make_relative_rename_path(track: &Track, base_path: &PathBuf) -> Result<PathBuf, MackError> {
    let tags = track.tag_file.tag()?;
    let mut path = base_path.clone();

    path.push(&sanitise_path_part(tags.artist()));
    path.push(&sanitise_path_part(tags.album()));

    let extension = track
        .path
        .extension()
        .expect("BUG: ext required in walkbuilder, but missing")
        .to_str()
        .ok_or(MackError::InvalidUnicode)?;

    let raw_filename = format!("{:02} {}.{}", tags.track(), tags.title(), extension);
    path.push(&sanitise_path_part(raw_filename));

    Ok(path)
}
