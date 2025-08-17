//! Template engine implementation

use crate::error::{TemplateError, TemplateResult};
use crate::context::TemplateContext;
use crate::value::TemplateValue;
use crate::utils::html_escape;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Template engine for rendering HTML templates
#[derive(Debug, Clone)]
pub struct TemplateEngine {
    template_dir: String,
    cache: HashMap<String, String>,
}

impl TemplateEngine {
    /// Create a new template engine
    pub fn new(template_dir: &str) -> Self {
        Self {
            template_dir: template_dir.to_string(),
            cache: HashMap::new(),
        }
    }

    /// Load and cache a template
    pub fn load_template(&mut self, name: &str) -> TemplateResult<String> {
        if let Some(cached) = self.cache.get(name) {
            return Ok(cached.clone());
        }

        let path = Path::new(&self.template_dir).join(name);
        let content = fs::read_to_string(&path)
            .map_err(|e| TemplateError::Template(format!("Failed to read template '{}': {}", name, e)))?;

        self.cache.insert(name.to_string(), content.clone());
        Ok(content)
    }

    /// Render a template with context
    pub fn render(&mut self, template_name: &str, context: &TemplateContext) -> TemplateResult<String> {
        let template = self.load_template(template_name)?;
        self.render_string(&template, context)
    }

    /// Render a template string with context
    pub fn render_string(&mut self, template: &str, context: &TemplateContext) -> TemplateResult<String> {
        let mut result = template.to_string();
        
        // Process includes first
        result = self.process_includes(&result)?;
        
        // Process conditionals
        result = self.process_conditionals(&result, context)?;
        
        // Process loops
        result = self.process_loops(&result, context)?;
        
        // Process variables
        result = self.process_variables(&result, context)?;
        
        // Remove comments
        result = self.process_comments(&result);
        
        Ok(result)
    }

    /// Process include directives
    fn process_includes(&mut self, template: &str) -> TemplateResult<String> {
        let mut result = template.to_string();
        
        while let Some(start) = result.find("{{include ") {
            let end = result[start..].find("}}")
                .ok_or_else(|| TemplateError::Parse("Unclosed include directive".to_string()))?;
            
            let directive = &result[start + 10..start + end];
            let include_name = directive.trim().trim_matches('"').trim_matches('\'');
            
            let included_content = self.load_template(include_name)?;
            result.replace_range(start..start + end + 2, &included_content);
        }
        
        Ok(result)
    }

    /// Process conditional blocks
    fn process_conditionals(&self, template: &str, context: &TemplateContext) -> TemplateResult<String> {
        let mut result = template.to_string();
        
        while let Some(if_start) = result.find("{{if ") {
            let if_end = result[if_start..].find("}}")
                .ok_or_else(|| TemplateError::Parse("Unclosed if directive".to_string()))?;
            
            let condition = &result[if_start + 5..if_start + if_end].trim();
            
            let block_start = if_start + if_end + 2;
            let block_end = result[block_start..].find("{{/if}}")
                .ok_or_else(|| TemplateError::Parse("Missing {{/if}} directive".to_string()))?;
            
            let block_content = result[block_start..block_start + block_end].to_string();
            
            let should_include = self.evaluate_condition(condition, context);
            let replacement = if should_include { &block_content } else { "" };
            
            result.replace_range(if_start..block_start + block_end + 7, replacement);
        }
        
        Ok(result)
    }

    /// Process loop blocks
    fn process_loops(&self, template: &str, context: &TemplateContext) -> TemplateResult<String> {
        let mut result = template.to_string();
        
        while let Some(for_start) = result.find("{{for ") {
            let for_end = result[for_start..].find("}}")
                .ok_or_else(|| TemplateError::Parse("Unclosed for directive".to_string()))?;
            
            let loop_def = &result[for_start + 6..for_start + for_end].trim();
            let parts: Vec<&str> = loop_def.split(" in ").collect();
            
            if parts.len() != 2 {
                return Err(TemplateError::Parse("Invalid for loop syntax".to_string()));
            }
            
            let item_var = parts[0].trim();
            let array_var = parts[1].trim();
            
            let block_start = for_start + for_end + 2;
            let block_end = result[block_start..].find("{{/for}}")
                .ok_or_else(|| TemplateError::Parse("Missing {{/for}} directive".to_string()))?;
            
            let block_content = &result[block_start..block_start + block_end];
            
            let replacement = self.render_loop(item_var, array_var, block_content, context)?;
            
            result.replace_range(for_start..block_start + block_end + 8, &replacement);
        }
        
        Ok(result)
    }

    /// Process variable substitutions
    fn process_variables(&self, template: &str, context: &TemplateContext) -> TemplateResult<String> {
        let mut result = template.to_string();
        
        // Process raw variables {{& variable}}
        while let Some(start) = result.find("{{& ") {
            let end = result[start..].find("}}")
                .ok_or_else(|| TemplateError::Parse("Unclosed variable directive".to_string()))?;
            
            let var_name = &result[start + 4..start + end].trim();
            let value = self.get_variable_value(var_name, context);
            
            result.replace_range(start..start + end + 2, &value);
        }
        
        // Process escaped variables {{variable}}
        while let Some(start) = result.find("{{") {
            if result[start..].starts_with("{{if ") || 
               result[start..].starts_with("{{for ") ||
               result[start..].starts_with("{{include ") ||
               result[start..].starts_with("{{!") ||
               result[start..].starts_with("{{/") {
                // Skip processed directives
                if let Some(skip_end) = result[start..].find("}}") {
                    result = result[..start].to_string() + &result[start + skip_end + 2..];
                    continue;
                } else {
                    break;
                }
            }
            
            let end = result[start..].find("}}")
                .ok_or_else(|| TemplateError::Parse("Unclosed variable directive".to_string()))?;
            
            let var_name = &result[start + 2..start + end].trim();
            let value = self.get_variable_value(var_name, context);
            let escaped_value = html_escape(&value);
            
            result.replace_range(start..start + end + 2, &escaped_value);
        }
        
        Ok(result)
    }

    /// Get variable value with support for dot notation (object.property)
    fn get_variable_value(&self, var_name: &str, context: &TemplateContext) -> String {
        if var_name.contains('.') {
            let parts: Vec<&str> = var_name.split('.').collect();
            if parts.len() == 2 {
                let object_name = parts[0];
                let property_name = parts[1];
                
                if let Some(TemplateValue::Object(obj)) = context.variables.get(object_name) {
                    if let Some(value) = obj.get(property_name) {
                        return match value {
                            TemplateValue::String(s) => s.clone(),
                            TemplateValue::Bool(b) => b.to_string(),
                            TemplateValue::Number(n) => n.to_string(),
                            _ => String::new(),
                        };
                    }
                }
            }
            String::new()
        } else {
            context.get_string(var_name).unwrap_or_default()
        }
    }

    /// Process comments
    fn process_comments(&self, template: &str) -> String {
        let mut result = template.to_string();
        
        while let Some(start) = result.find("{{!") {
            if let Some(end) = result[start..].find("}}") {
                result.replace_range(start..start + end + 2, "");
            } else {
                break;
            }
        }
        
        result
    }

    /// Evaluate a condition
    fn evaluate_condition(&self, condition: &str, context: &TemplateContext) -> bool {
        // Simple condition evaluation - can be extended
        if let Some(value) = context.variables.get(condition) {
            match value {
                TemplateValue::Bool(b) => *b,
                TemplateValue::String(s) => !s.is_empty(),
                TemplateValue::Number(n) => *n != 0,
                TemplateValue::Array(a) => !a.is_empty(),
                TemplateValue::Object(o) => !o.is_empty(),
            }
        } else {
            false
        }
    }

    /// Render a loop
    fn render_loop(&self, item_var: &str, array_var: &str, block: &str, context: &TemplateContext) -> TemplateResult<String> {
        if let Some(TemplateValue::Array(items)) = context.variables.get(array_var) {
            let mut result = String::new();
            
            for item in items {
                let mut loop_context = context.clone();
                loop_context.set(item_var, item.clone());
                
                let rendered_block = self.process_variables(block, &loop_context)?;
                result.push_str(&rendered_block);
            }
            
            Ok(result)
        } else {
            Ok(String::new())
        }
    }
}