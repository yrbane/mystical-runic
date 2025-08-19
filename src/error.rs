//! Error types for the template engine

use std::fmt;

/// Template engine error type
#[derive(Debug)]
pub enum TemplateError {
    Io(std::io::Error),
    Template(String),
    Parse(String),
    Runtime(String),
    Render(String),
    Security(String),
}

impl fmt::Display for TemplateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TemplateError::Io(err) => write!(f, "IO error: {}", err),
            TemplateError::Template(msg) => write!(f, "Template error: {}", msg),
            TemplateError::Parse(msg) => write!(f, "Parse error: {}", msg),
            TemplateError::Runtime(msg) => write!(f, "Runtime error: {}", msg),
            TemplateError::Render(msg) => write!(f, "Render error: {}", msg),
            TemplateError::Security(msg) => write!(f, "Security error: {}", msg),
        }
    }
}

impl std::error::Error for TemplateError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            TemplateError::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for TemplateError {
    fn from(err: std::io::Error) -> Self {
        TemplateError::Io(err)
    }
}

/// Template engine result type
pub type TemplateResult<T> = Result<T, TemplateError>;