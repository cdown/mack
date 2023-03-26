use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

const LASTMACK_NAME: &str = ".lastmack";

fn make_lastmack_path(base_path: &Path) -> PathBuf {
    let mut last_run_path = base_path.to_path_buf();
    last_run_path.push(LASTMACK_NAME);
    last_run_path
}

pub fn get_last_run_time(base_path: &Path) -> Option<SystemTime> {
    let last_run_path = make_lastmack_path(base_path);
    get_mtime(last_run_path).ok()
}

fn get_mtime<T: AsRef<Path>>(path: T) -> Result<SystemTime> {
    let stat = fs::metadata(path.as_ref())?;
    Ok(stat.modified()?)
}

pub fn set_last_run_time(base_path: &Path) -> Result<()> {
    let last_run_path = make_lastmack_path(base_path);
    fs::File::create(last_run_path)?;
    Ok(())
}

pub fn mtime_def_now<T: AsRef<Path>>(path: T) -> SystemTime {
    get_mtime(path.as_ref()).unwrap_or_else(|_| SystemTime::now())
}
