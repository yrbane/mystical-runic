//! CLI tools for v0.5.0
//!
//! Command-line template processing utilities

#[cfg(feature = "cli")]
use crate::{TemplateEngine, TemplateContext, TemplateResult, TemplateError};

#[cfg(feature = "cli")]
use clap::{Parser, Subcommand};

#[cfg(feature = "cli")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "cli")]
use std::collections::HashMap;

#[cfg(feature = "cli")]
// Path is available for future path operations

#[cfg(feature = "cli")]
use std::fs;

/// CLI configuration structure
#[cfg(feature = "cli")]
#[derive(Debug, Deserialize, Serialize)]
pub struct CliConfig {
    pub template: TemplateConfig,
    pub output: OutputConfig,
    pub data: DataConfig,
}

#[cfg(feature = "cli")]
#[derive(Debug, Deserialize, Serialize)]
pub struct TemplateConfig {
    pub directory: String,
    pub extension: String,
}

#[cfg(feature = "cli")]
#[derive(Debug, Deserialize, Serialize)]
pub struct OutputConfig {
    pub directory: String,
    pub minify: bool,
}

#[cfg(feature = "cli")]
#[derive(Debug, Deserialize, Serialize)]
pub struct DataConfig {
    pub format: String,
    pub file: String,
}

/// CLI command structure
#[cfg(feature = "cli")]
#[derive(Parser)]
#[command(name = "runic")]
#[command(about = "🔮 Mystical-Runic CLI - Ancient template processing magic")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[cfg(feature = "cli")]
#[derive(Subcommand)]
pub enum Commands {
    /// Render a template with data
    Render {
        /// Template file or string
        template: String,
        /// Data file (JSON/TOML)
        #[arg(short, long)]
        data: Option<String>,
        /// Output file
        #[arg(short, long)]
        output: Option<String>,
    },
    /// Watch templates for changes and auto-render
    Watch {
        /// Template directory to watch
        directory: String,
        /// Data file to use
        #[arg(short, long)]
        data: Option<String>,
    },
    /// Batch process multiple templates
    Batch {
        /// Configuration file
        #[arg(short, long)]
        config: String,
    },
}

/// Process a template string with JSON data
#[cfg(feature = "cli")]
pub fn process_template(template_content: &str, data: &str) -> TemplateResult<String> {
    let mut engine = TemplateEngine::new(".");
    let context = parse_data(data)?;
    
    engine.render_string(template_content, &context)
}

/// Process template and data files
#[cfg(feature = "cli")]
pub fn process_files(template_file: &str, data_file: &str) -> TemplateResult<String> {
    let template_content = fs::read_to_string(template_file)
        .map_err(|e| TemplateError::Io(e))?;
    
    let data_content = fs::read_to_string(data_file)
        .map_err(|e| TemplateError::Io(e))?;
    
    process_template(&template_content, &data_content)
}

/// Batch process multiple templates
#[cfg(feature = "cli")]
pub fn batch_process(templates: Vec<(&str, &str)>, data: &HashMap<&str, &str>) -> TemplateResult<Vec<String>> {
    let mut engine = TemplateEngine::new(".");
    let mut context = TemplateContext::new();
    
    // Load data into context
    for (key, value) in data {
        context.set_string(key, value);
    }
    
    let mut results = Vec::new();
    
    for (_template_name, template_content) in templates {
        let rendered = engine.render_string(template_content, &context)?;
        results.push(rendered);
    }
    
    Ok(results)
}

/// Load CLI configuration from TOML
#[cfg(feature = "cli")]
pub fn load_config(toml_content: &str) -> TemplateResult<CliConfig> {
    toml::from_str(toml_content)
        .map_err(|e| TemplateError::Parse(format!("Config parse error: {}", e)))
}

/// Template file watcher for auto-recompilation
#[cfg(feature = "cli")]
pub struct TemplateWatcher {
    directory: String,
    last_modified: HashMap<String, std::time::SystemTime>,
}

#[cfg(feature = "cli")]
impl TemplateWatcher {
    pub fn new(directory: &str) -> TemplateResult<Self> {
        Ok(Self {
            directory: directory.to_string(),
            last_modified: HashMap::new(),
        })
    }
    
    pub fn start_watching(&mut self) -> TemplateResult<()> {
        // Initialize file timestamps
        self.scan_directory()?;
        Ok(())
    }
    
    pub fn check_changes(&mut self) -> TemplateResult<Vec<String>> {
        let mut changed_files = Vec::new();
        
        // Scan directory for changes
        let entries = fs::read_dir(&self.directory)
            .map_err(|e| TemplateError::Io(e))?;
        
        for entry in entries {
            let entry = entry.map_err(|e| TemplateError::Io(e))?;
            let path = entry.path();
            
            if path.is_file() {
                if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                    let metadata = entry.metadata().map_err(|e| TemplateError::Io(e))?;
                    let modified = metadata.modified().map_err(|e| TemplateError::Io(e))?;
                    
                    if let Some(&last_modified) = self.last_modified.get(filename) {
                        if modified > last_modified {
                            changed_files.push(filename.to_string());
                            self.last_modified.insert(filename.to_string(), modified);
                        }
                    } else {
                        // New file
                        changed_files.push(filename.to_string());
                        self.last_modified.insert(filename.to_string(), modified);
                    }
                }
            }
        }
        
        Ok(changed_files)
    }
    
    pub fn stop_watching(&self) -> TemplateResult<()> {
        // Cleanup watching
        Ok(())
    }
    
    fn scan_directory(&mut self) -> TemplateResult<()> {
        let entries = fs::read_dir(&self.directory)
            .map_err(|e| TemplateError::Io(e))?;
        
        for entry in entries {
            let entry = entry.map_err(|e| TemplateError::Io(e))?;
            let path = entry.path();
            
            if path.is_file() {
                if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                    let metadata = entry.metadata().map_err(|e| TemplateError::Io(e))?;
                    let modified = metadata.modified().map_err(|e| TemplateError::Io(e))?;
                    
                    self.last_modified.insert(filename.to_string(), modified);
                }
            }
        }
        
        Ok(())
    }
}

/// Parse data from JSON or TOML string
#[cfg(feature = "cli")]
fn parse_data(data: &str) -> TemplateResult<TemplateContext> {
    let mut context = TemplateContext::new();
    
    // Try JSON first
    if data.trim().starts_with('{') {
        if let Ok(value) = serde_json::from_str::<serde_json::Value>(data) {
            add_json_to_context(&mut context, &value)?;
            return Ok(context);
        }
    }
    
    // Try TOML
    if let Ok(value) = toml::from_str::<toml::Value>(data) {
        add_toml_to_context(&mut context, &value)?;
        return Ok(context);
    }
    
    Err(TemplateError::Parse("Failed to parse data as JSON or TOML".to_string()))
}

#[cfg(feature = "cli")]
fn add_json_to_context(context: &mut TemplateContext, value: &serde_json::Value) -> TemplateResult<()> {
    use serde_json::Value;
    
    match value {
        Value::Object(map) => {
            for (key, val) in map {
                match val {
                    Value::String(s) => context.set_string(key, s),
                    Value::Number(n) => {
                        if let Some(i) = n.as_i64() {
                            context.set_number(key, i);
                        } else if let Some(f) = n.as_f64() {
                            context.set_string(key, &f.to_string());
                        }
                    }
                    Value::Bool(b) => context.set_bool(key, *b),
                    _ => context.set_string(key, &val.to_string()),
                }
            }
        }
        _ => return Err(TemplateError::Parse("Expected JSON object".to_string())),
    }
    
    Ok(())
}

#[cfg(feature = "cli")]
fn add_toml_to_context(context: &mut TemplateContext, value: &toml::Value) -> TemplateResult<()> {
    use toml::Value;
    
    match value {
        Value::Table(map) => {
            for (key, val) in map {
                match val {
                    Value::String(s) => context.set_string(key, s),
                    Value::Integer(i) => context.set_number(key, *i),
                    Value::Boolean(b) => context.set_bool(key, *b),
                    _ => context.set_string(key, &val.to_string()),
                }
            }
        }
        _ => return Err(TemplateError::Parse("Expected TOML table".to_string())),
    }
    
    Ok(())
}

#[cfg(not(feature = "cli"))]
/// Placeholder when CLI feature is not enabled
pub struct CliPlaceholder;