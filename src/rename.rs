use types::{MackError, Track};
use std::path::PathBuf;
use std::fs;

/// TODO: Currently only filters out names guaranteed to be incompatible with POSIX filesystems
fn sanitise_path_part(path_part: String) -> String {
    path_part.replace("\0", "").replace("/", "_")
}

/// artist/album/2digitnum title.ext
fn make_relative_rename_path(track: &Track, base_path: &PathBuf) -> Result<PathBuf, MackError> {
    let tags = track.tag_file.tag()?;
    let mut path = base_path.clone();

    path.push(&sanitise_path_part(tags.artist().unwrap_or("Unknown Artist".to_string())));
    path.push(&sanitise_path_part(tags.album().unwrap_or("Unknown Album".to_string())));

    let extension = track
        .path
        .extension()
        .expect("BUG: ext required in walkbuilder, but missing")
        .to_str()
        .ok_or(MackError::InvalidUnicode)?;

    let raw_filename = format!(
        "{:02} {}.{}",
        tags.track().unwrap_or(0),
        tags.title().unwrap_or("Unknown Title".to_string()),
        extension
    );
    path.push(&sanitise_path_part(raw_filename));

    Ok(path)
}

fn rename_creating_dirs(from: &PathBuf, to: &PathBuf) -> Result<(), MackError> {
    fs::create_dir_all(&to.parent().ok_or(MackError::WouldMoveToFsRoot)?)?;
    Ok(fs::rename(&from, &to)?)
}

pub fn rename_track(
    track: &Track,
    base_path: &PathBuf,
    dry_run: bool,
) -> Result<Option<PathBuf>, MackError> {
    let new_path = make_relative_rename_path(&track, &base_path)?;

    if new_path == track.path {
        return Ok(None);
    }

    if !dry_run {
        rename_creating_dirs(&track.path, &new_path)?;
    }

    return Ok(Some(new_path));
}
