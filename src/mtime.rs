use crate::types;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

const LASTMACK_NAME: &str = ".lastmack";

fn make_lastmack_path(base_path: &PathBuf) -> PathBuf {
    let mut last_run_path = base_path.clone();
    last_run_path.push(LASTMACK_NAME);
    last_run_path
}

pub fn get_last_run_time(base_path: &PathBuf) -> Option<SystemTime> {
    let last_run_path = make_lastmack_path(base_path);
    get_mtime(last_run_path).ok()
}

fn get_mtime<T: AsRef<Path>>(path: T) -> Result<SystemTime, types::MackError> {
    let stat = fs::metadata(path.as_ref())?;
    Ok(stat.modified()?)
}

pub fn set_last_run_time(base_path: &PathBuf) -> Result<(), types::MackError> {
    let last_run_path = make_lastmack_path(base_path);
    fs::File::create(last_run_path)?;
    Ok(())
}

pub fn mtime_def_now<T: AsRef<Path>>(path: T) -> SystemTime {
    get_mtime(path.as_ref()).unwrap_or_else(|_| SystemTime::now())
}
