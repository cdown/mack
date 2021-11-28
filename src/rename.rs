use std::fs;
use std::path::PathBuf;
use types::{MackError, Track};

// Arbitrary limit on path part without extension to try to avoid brushing against PATH_MAX. We
// can't just check PATH_MAX and similar, because we also want to avoid issues when copying
// elsewhere later.
const MAX_PATH_PART_LEN: usize = 64;

/// TODO: Currently only filters out names guaranteed to be incompatible with POSIX filesystems
fn sanitise_path_part(path_part: &str) -> String {
    let mut out = path_part.replace("\0", "").replace("/", "_");
    out.truncate(MAX_PATH_PART_LEN);
    out
}

/// artist/album/2digitnum title.ext
fn make_relative_rename_path(track: &Track, output_path: &PathBuf) -> Result<PathBuf, MackError> {
    let tags = track.tag_file.tag()?;
    let mut path = output_path.clone();

    path.push(&sanitise_path_part(
        &tags
            .artist()
            .unwrap_or_else(|| "Unknown Artist".to_string()),
    ));
    path.push(&sanitise_path_part(
        &tags.album().unwrap_or_else(|| "Unknown Album".to_string()),
    ));

    let extension = track
        .path
        .extension()
        .expect("BUG: ext required in walkbuilder, but missing");

    let raw_filename = format!(
        "{:02} {}.", // the extra "." is needed for .set_extension in case we already have a "."
        tags.track().unwrap_or(0),
        tags.title().unwrap_or_else(|| "Unknown Title".to_string()),
    );
    path.push(&sanitise_path_part(&raw_filename));
    path.set_extension(extension);

    Ok(path)
}

fn rename_creating_dirs(from: &PathBuf, to: &PathBuf) -> Result<(), MackError> {
    fs::create_dir_all(&to.parent().ok_or(MackError::WouldMoveToFsRoot)?)?;

    // Trying to rename cross device? Just copy and unlink the old one
    if let Err(err) = fs::rename(&from, &to) {
        if let Some(os_err) = err.raw_os_error() {
            if os_err == libc::EXDEV {
                fs::copy(&from, &to)?;
                fs::remove_file(&from)?;
            } else {
                Err(err)?;
            }
        }
    }
    Ok(())
}

pub fn rename_track(
    track: &Track,
    output_path: &PathBuf,
    dry_run: bool,
) -> Result<Option<PathBuf>, MackError> {
    let new_path = make_relative_rename_path(&track, &output_path)?;

    if new_path == track.path {
        return Ok(None);
    }

    if !dry_run {
        rename_creating_dirs(&track.path, &new_path)?;
    }

    Ok(Some(new_path))
}
