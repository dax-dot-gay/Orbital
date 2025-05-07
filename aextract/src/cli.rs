use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Clone, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Set Steam library base path (i.e. ~/.steam/steam)
    #[arg(value_parser = clap::value_parser!(std::path::PathBuf))]
    pub steam_library: PathBuf,

    /// Select docs.json locale to extract
    #[arg(short, long)]
    pub locale: Option<String>,

    /// Working directory (should generally be left empty, will default to a tempdir.)
    #[arg(short = 'w', long = "workdir", value_parser = clap::value_parser!(std::path::PathBuf))]
    pub workdir: Option<PathBuf>,

    /// Output directory (defaults to creating a folder in the current directory.)
    #[arg(short = 'o', long = "output", value_parser = clap::value_parser!(std::path::PathBuf))]
    pub output: Option<PathBuf>,
}
