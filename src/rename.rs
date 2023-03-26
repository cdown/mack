use crate::types::Track;
use anyhow::{Context, Result};
use id3::TagLike;
use std::fs;
use std::path::{Path, PathBuf};

#[cfg(target_family = "unix")]
use libc::EXDEV as xdev_err;
#[cfg(target_family = "windows")]
use winapi::shared::winerror::ERROR_NOT_SAME_DEVICE as xdev_err;

// Arbitrary limit on path part without extension to try to avoid brushing against PATH_MAX. We
// can't just check PATH_MAX and similar, because we also want to avoid issues when copying
// elsewhere later.
const MAX_PATH_PART_LEN: usize = 64;

const ADDITIONAL_ACCEPTED_CHARS: &[char] = &['.', '-', '(', ')', ','];

fn sanitise_path_part(path_part: &str) -> String {
    let mut out: String = path_part
        .chars()
        .map(|c| {
            if c.is_alphanumeric()
                || c.is_whitespace()
                || ADDITIONAL_ACCEPTED_CHARS.iter().any(|&a| a == c)
            {
                c
            } else {
                '_'
            }
        })
        .collect();
    out.truncate(MAX_PATH_PART_LEN);
    out
}

/// artist/album/2digitnum title.ext
fn make_relative_rename_path(track: &Track, output_path: &Path) -> PathBuf {
    let tags = &track.tag;
    let mut path = output_path.to_path_buf();

    path.push(&sanitise_path_part(
        tags.artist().unwrap_or("Unknown Artist"),
    ));
    path.push(&sanitise_path_part(tags.album().unwrap_or("Unknown Album")));

    let extension = track
        .path
        .extension()
        .expect("BUG: ext required in walkbuilder, but missing");

    let raw_filename = format!(
        "{:02} {}.", // the extra "." is needed for .set_extension in case we already have a "."
        tags.track().unwrap_or(0),
        tags.title().unwrap_or("Unknown Title"),
    );
    path.push(&sanitise_path_part(&raw_filename));
    path.set_extension(extension);
    path
}

fn rename_creating_dirs(from: &PathBuf, to: &PathBuf) -> Result<()> {
    fs::create_dir_all(to.parent().context("Refusing to move to FS root")?)?;

    // Trying to rename cross device? Just copy and unlink the old one
    if let Err(err) = fs::rename(from, to) {
        #[allow(clippy::useless_conversion)] // Necessary for Windows only
        let xdev_err_cast = xdev_err.try_into()?;
        if err.raw_os_error() == Some(xdev_err_cast) {
            fs::copy(from, to)?;
            fs::remove_file(from)?;
        } else {
            Err(err)?;
        }
    }
    Ok(())
}

pub fn rename_track(track: &Track, output_path: &Path, dry_run: bool) -> Result<Option<PathBuf>> {
    let new_path = make_relative_rename_path(track, output_path);

    if new_path == track.path {
        return Ok(None);
    }

    if !dry_run {
        rename_creating_dirs(&track.path, &new_path)?;
    }

    Ok(Some(new_path))
}
