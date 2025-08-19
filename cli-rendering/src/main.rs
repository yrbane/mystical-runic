mod cli;
mod core;
mod data_loader;
mod file_reader;

use crate::cli::Args;
use crate::core::{AppRunicRenderer, AppTemplateRenderer};
use crate::data_loader::load_data_from_file;
use crate::file_reader::read_file_content;
use anyhow::{Context, Result};
use clap::Parser;
use std::path::PathBuf;
use std::fs::File;
use std::io::Write;
use tempfile::NamedTempFile;
use webbrowser::{self, Browser};

const DEFAULT_TEMPLATE_PATH: &str = "templates/main.rune";
const DEFAULT_DATA_PATH: &str = "data/main.json";

fn main() -> Result<()> {
    let args = Args::parse();

    let template_path = args.template.unwrap_or_else(|| PathBuf::from(DEFAULT_TEMPLATE_PATH));
    let data_path = args.data.unwrap_or_else(|| PathBuf::from(DEFAULT_DATA_PATH));

    let template_content = read_file_content(&template_path)?;
    let data = load_data_from_file(&data_path)?;

    let engine = AppRunicRenderer;
    let rendered_html = engine.render(&template_content, &data)?;

    if args.browser {
        let mut temp_file = NamedTempFile::new()?;
        temp_file.write_all(rendered_html.as_bytes())?;
        let temp_path = temp_file.path().to_path_buf();
        let url = temp_path.to_str().context("Failed to convert path to string")?;

        // Try to open with Firefox first
        if webbrowser::open_browser(Browser::Firefox, url).is_err() {
            // If Firefox fails, try the default browser
            webbrowser::open(url)?;
        }
        // Keep the file alive until the program exits
        temp_file.keep()?;
    } else if let Some(output_path) = args.output {
        let mut file = File::create(&output_path)?;
        file.write_all(rendered_html.as_bytes())?;
    } else {
        println!("{}", rendered_html);
    }

    Ok(())
}