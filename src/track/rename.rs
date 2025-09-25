use crate::track::Track;
use anyhow::{Context, Result};
use funcfmt::{FormatPieces, Render};
use once_cell::sync::Lazy;
use regex::Regex;
use std::ffi::{OsStr, OsString};
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

/// `String::truncate` will panic if not at a char boundary
fn safe_truncate(s: &mut String, max_chars: usize) {
    if let Some((idx, _)) = s.char_indices().nth(max_chars) {
        s.truncate(idx);
    }
}

// Arbitrary limit on path part without extension to try to avoid brushing against PATH_MAX. We
// can't just check PATH_MAX and similar, because we also want to avoid issues when copying
// elsewhere later.
static MULTI_DOT_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\.\.+").expect("BUG: Invalid regex"));
// Illegal characters for Windows filenames, except for / and \ which are path separators and
// handled by `components()`.
static ILLEGAL_CHARS_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"[<>:"|?*]"#).expect("BUG: Invalid regex"));
const MAX_PATH_PART_LEN: usize = 64;
fn normalise_dirs(path_part: String) -> PathBuf {
    let partial = PathBuf::from(path_part);
    partial
        .components()
        .map(|c| {
            let mut s = c
                .as_os_str()
                .to_os_string()
                .into_string()
                .expect("invalid path");
            safe_truncate(&mut s, MAX_PATH_PART_LEN);

            // Replace illegal characters with underscores.
            s = ILLEGAL_CHARS_RE.replace_all(&s, "_").to_string();

            // Trim leading/trailing whitespace, which can be problematic on some filesystems.
            s = s.trim().to_string();

            // exfat normalises this and it confuses adb-sync and other tooling
            s = MULTI_DOT_RE.replace_all(&s, ".").to_string();

            // Disallow leading dots to prevent creating hidden files/directories.
            // Disallow trailing dots as they are invalid on Windows.
            let s = s.trim_matches('.').to_string();

            // If the component is now empty (e.g. it was just "."), use a placeholder.
            if s.is_empty() {
                "_".to_string()
            } else {
                s
            }
        })
        .collect()
}

fn add_extension(path: PathBuf, ext: impl AsRef<OsStr>) -> PathBuf {
    let mut os_string: OsString = path.into();
    os_string.push(".");
    os_string.push(ext.as_ref());
    os_string.into()
}

pub fn rename_track(
    track: &Track,
    fp: &FormatPieces<Track>,
    output_path: &Path,
    dry_run: bool,
) -> Result<Option<PathBuf>> {
    let mut new_path = output_path.to_path_buf();
    let partial = normalise_dirs(fp.render(track)?);
    new_path.push(partial);

    // We might have truncated and have a dot elsewhere, so we can't use set_extension
    new_path = add_extension(
        new_path,
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
