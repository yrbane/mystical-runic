//! Error types for the template engine

use std::fmt;

/// Template engine error type with enhanced developer experience
#[derive(Debug)]
pub enum TemplateError {
    Io(std::io::Error),
    Template(String),
    Parse(String),
    Runtime(String),
    Render(String),
    Security(String),
    
    // v0.5.1 Professional Security Enhancements
    /// Advanced security violation with detailed threat analysis
    SecurityViolation {
        violation_type: SecurityViolationType,
        attempted_path: String,
        threat_level: ThreatLevel,
        mitigation: String,
        request_id: Option<String>,
    },
    
    /// Rate limiting violation
    RateLimit {
        limit_type: String,
        current_count: u64,
        max_allowed: u64,
        reset_time: std::time::SystemTime,
    },
    
    /// Template size or complexity violation
    ResourceExhaustion {
        resource_type: String,
        current_usage: u64,
        max_allowed: u64,
    },
    
    // v0.4.0 Enhanced Error Types for Better Developer Experience
    /// Parse error with precise location information
    ParseWithLocation {
        message: String,
        line: usize,
        column: usize,
        template_name: Option<String>,
        context_lines: Vec<String>,
    },
    
    /// Template file not found with helpful suggestions
    TemplateNotFoundWithSuggestions {
        template_name: String,
        template_dir: String,
        suggestions: Vec<String>,
        available_templates: Vec<String>,
    },
    
    /// Variable not found with context and suggestions
    VariableNotFoundWithSuggestions {
        variable_name: String,
        line: usize,
        column: usize,
        available_variables: Vec<String>,
        suggestions: Vec<String>,
        template_name: Option<String>,
    },
    
    /// Nested template error with full stack trace
    NestedTemplateError {
        template_stack: Vec<String>,
        root_error: Box<TemplateError>,
        current_template: String,
    },
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
            
            // v0.4.0 Enhanced Error Messages
            TemplateError::ParseWithLocation { 
                message, 
                line, 
                column, 
                template_name,
                context_lines,
            } => {
                if let Some(template) = template_name {
                    write!(f, "Parse error in template '{}' at line {}, column {}: {}", 
                           template, line, column, message)?;
                } else {
                    write!(f, "Parse error at line {}, column {}: {}", line, column, message)?;
                }
                
                if !context_lines.is_empty() {
                    writeln!(f, "\n\nContext:")?;
                    for (i, context_line) in context_lines.iter().enumerate() {
                        let line_num = line.saturating_sub(context_lines.len() / 2) + i;
                        if line_num == *line {
                            writeln!(f, " -> {}: {}", line_num, context_line)?;
                        } else {
                            writeln!(f, "    {}: {}", line_num, context_line)?;
                        }
                    }
                }
                
                // Add helpful suggestions based on common mistakes
                if message.contains("Expected '}}'") {
                    write!(f, "\nSuggestion: Add '}}' after the variable or directive")?;
                } else if message.contains("unclosed") && message.contains("{{if") {
                    write!(f, "\nSuggestion: Add '{{{{/if}}}}' to close the conditional")?;
                }
                
                Ok(())
            },
            
            TemplateError::TemplateNotFoundWithSuggestions { 
                template_name,
                template_dir, 
                suggestions,
                available_templates,
            } => {
                write!(f, "Template '{}' not found in directory '{}'", template_name, template_dir)?;
                
                if !suggestions.is_empty() {
                    write!(f, "\n\nDid you mean:")?;
                    for suggestion in suggestions {
                        write!(f, "\n  - {}", suggestion)?;
                    }
                }
                
                if !available_templates.is_empty() {
                    write!(f, "\n\nAvailable templates:")?;
                    for template in available_templates {
                        write!(f, "\n  - {}", template)?;
                    }
                }
                
                Ok(())
            },
            
            TemplateError::VariableNotFoundWithSuggestions { 
                variable_name,
                line,
                column,
                available_variables,
                suggestions,
                template_name,
            } => {
                if let Some(template) = template_name {
                    write!(f, "Variable '{}' not found in template '{}' at line {}, column {}", 
                           variable_name, template, line, column)?;
                } else {
                    write!(f, "Variable '{}' not found at line {}, column {}", 
                           variable_name, line, column)?;
                }
                
                if !suggestions.is_empty() {
                    write!(f, "\n\nDid you mean:")?;
                    for suggestion in suggestions {
                        write!(f, "\n  - {}", suggestion)?;
                    }
                }
                
                if !available_variables.is_empty() {
                    write!(f, "\n\nAvailable variables:")?;
                    for variable in available_variables {
                        write!(f, "\n  - {}", variable)?;
                    }
                }
                
                Ok(())
            },
            
            TemplateError::NestedTemplateError { 
                template_stack,
                root_error,
                current_template,
            } => {
                write!(f, "Error in template '{}'\n\nTemplate stack:", current_template)?;
                for (i, template) in template_stack.iter().enumerate() {
                    if i == template_stack.len() - 1 {
                        write!(f, "\n  -> {} (error here)", template)?;
                    } else {
                        write!(f, "\n     {}", template)?;
                    }
                }
                write!(f, "\n\nRoot cause: {}", root_error)?;
                
                Ok(())
            },
            
            // v0.5.1 Professional Security Enhancements
            TemplateError::SecurityViolation {
                violation_type,
                attempted_path,
                threat_level,
                mitigation,
                request_id,
            } => {
                writeln!(f, "🔴 SECURITY VIOLATION [{:?}]: {:?}", threat_level, violation_type)?;
                writeln!(f, "Attempted Path: {}", attempted_path)?;
                writeln!(f, "Mitigation: {}", mitigation)?;
                if let Some(id) = request_id {
                    writeln!(f, "Request ID: {}", id)?;
                }
                Ok(())
            },
            
            TemplateError::RateLimit {
                limit_type,
                current_count,
                max_allowed,
                reset_time,
            } => {
                writeln!(f, "⚠️ RATE LIMIT EXCEEDED: {}", limit_type)?;
                writeln!(f, "Current: {} / Max: {}", current_count, max_allowed)?;
                writeln!(f, "Reset time: {:?}", reset_time)?;
                Ok(())
            },
            
            TemplateError::ResourceExhaustion {
                resource_type,
                current_usage,
                max_allowed,
            } => {
                writeln!(f, "📊 RESOURCE EXHAUSTION: {}", resource_type)?;
                writeln!(f, "Usage: {} / Limit: {}", current_usage, max_allowed)?;
                Ok(())
            },
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

/// Security violation types for comprehensive threat classification
#[derive(Debug, Clone)]
pub enum SecurityViolationType {
    /// Path traversal attempt (../ or absolute paths)
    PathTraversal,
    /// Template injection attempt
    TemplateInjection,
    /// XSS attempt through unescaped content
    CrossSiteScripting,
    /// Server-side template injection
    ServerSideTemplateInjection,
    /// Unauthorized file access
    UnauthorizedFileAccess,
    /// Malicious payload in template variable
    MaliciousPayload,
    /// Resource exhaustion attack
    ResourceExhaustion,
}

/// Threat severity levels for security incidents
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ThreatLevel {
    /// Low severity - potential issue, logged but processing continues
    Low,
    /// Medium severity - security concern, additional validation applied
    Medium,
    /// High severity - clear attack attempt, request blocked
    High,
    /// Critical severity - sophisticated attack, immediate blocking and alerting
    Critical,
}

/// Security incident details for audit logging
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct SecurityIncident {
    pub timestamp: std::time::SystemTime,
    pub violation_type: SecurityViolationType,
    pub threat_level: ThreatLevel,
    pub source_ip: Option<String>,
    pub user_agent: Option<String>,
    pub request_path: String,
    pub payload: String,
    pub mitigation_applied: String,
}

/// Template engine result type
pub type TemplateResult<T> = Result<T, TemplateError>;