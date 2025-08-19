use anyhow::{Result, Context};
use std::path::Path;
use serde_json::Value;

pub fn load_data_from_file(path: &Path) -> Result<Value> {
    let file_content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read data file: {}", path.display()))?;
    
    serde_json::from_str(&file_content)
        .with_context(|| format!("Failed to parse JSON from data file: {}", path.display()))
}