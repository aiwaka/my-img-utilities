use std::path::PathBuf;

use clap::Parser;

/// Convert image file to webp format.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct AppArgs {
    /// target file.
    #[arg(short, long)]
    pub filepath: Option<PathBuf>,
    #[arg(short, long)]
    pub output: Option<PathBuf>,
    // /// specify filter type
    // #[arg(short, long)]
    // pub filter_type: Option<u32>,
}
