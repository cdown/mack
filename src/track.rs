use types::{MackError, Track};
use std::path::PathBuf;
use taglib::File as TFile;


pub fn get_track(path: PathBuf) -> Result<Track, MackError> {
    let tag_file = TFile::new(path.clone().to_str().ok_or(MackError::InvalidUnicode)?)?;
    Ok(Track {
        path: path,
        tag_file: tag_file,
    })
}

/// We don't intend to print *all* metadata, only ones we might actually try to apply fixes to
pub fn print_track_info(track: &Track) -> () {
    let tags = track.tag_file.tag();

    match tags {
        Ok(tags) => {
            println!("{}:", track.path.display());
            println!("- Album:   {}", tags.album().unwrap_or("Unknown".to_owned()));
            println!("- Artist:  {}", tags.artist().unwrap_or("Unknown".to_owned()));
            println!("- Title:   {}", tags.title().unwrap_or("Unknown".to_owned()));
            println!("- Track #: {}", tags.track().unwrap_or(0));
            println!("- Year:    {}\n", tags.year().unwrap_or(0));
        }
        Err(err) => eprintln!("error printing track info: {}: {:?}", track.path.display(), err),
    }
}
