use clap::Parser;
use id3::Tag;
use std::path::PathBuf;

pub struct Track {
    pub path: PathBuf,
    pub tag: Tag,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TrackFeat {
    pub title: String,
    pub featured_artists: Vec<String>,
    pub original_title: String,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    #[arg(
        long,
        short = 'n',
        help = "Don't actually rename or tag files, only display what would happen"
    )]
    pub dry_run: bool,

    #[arg(
        long,
        short,
        help = "Ignore .lastmack timestamp, run on all files present regardless"
    )]
    pub force: bool,

    #[arg(
        long,
        short,
        help = "Use a different output directory (by default, it's the same as the input dir)"
    )]
    pub output_dir: Option<PathBuf>,

    #[arg(help = "Directories to find music files in.")]
    pub paths: Vec<PathBuf>,
}
