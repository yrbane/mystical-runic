use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path to the template file
    #[arg(short, long)]
    pub template: Option<PathBuf>,

    /// Path to the data file (JSON format)
    #[arg(short, long)]
    pub data: Option<PathBuf>,

    /// Path to the output HTML file
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Open the rendered HTML in the default web browser
    #[arg(long)]
    pub browser: bool,
}