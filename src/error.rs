//! Error types for the template engine

use thiserror::Error;

/// Template engine error type
#[derive(Error, Debug)]
pub enum TemplateError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Template error: {0}")]
    Template(String),
    
    #[error("Parse error: {0}")]
    Parse(String),
    
    #[error("Runtime error: {0}")]
    Runtime(String),
}

/// Template engine result type
pub type TemplateResult<T> = Result<T, TemplateError>;