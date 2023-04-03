use crate::types::Track;
use anyhow::{Context, Result};
use funcfmt::{FormatPieces, Render};
use std::fs;
use std::path::{Path, PathBuf};

#[cfg(target_family = "unix")]
use libc::EXDEV as xdev_err;
#[cfg(target_family = "windows")]
use winapi::shared::winerror::ERROR_NOT_SAME_DEVICE as xdev_err;

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

pub fn rename_track(
    track: &Track,
    fp: &FormatPieces<Track>,
    output_path: &Path,
    dry_run: bool,
) -> Result<Option<PathBuf>> {
    let mut new_path = output_path.to_path_buf();
    let partial = fp.render(track)?;
    new_path.push(partial);

    new_path.set_extension(
        track
            .path
            .extension()
            .context("ext required in walkbuilder, but missing")?,
    );

    if new_path == track.path {
        return Ok(None);
    }

    if !dry_run {
        rename_creating_dirs(&track.path, &new_path)?;
    }

    Ok(Some(new_path))
}
