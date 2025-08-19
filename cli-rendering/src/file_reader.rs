use anyhow::{Result, Context};
use std::path::Path;
use std::fs;

pub fn read_file_content(path: &Path) -> Result<String> {
    fs::read_to_string(path)
        .with_context(|| format!("Failed to read file: {}", path.display()))
}