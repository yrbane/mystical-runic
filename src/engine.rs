//! Template engine implementation

use crate::error::{TemplateError, TemplateResult};
use crate::context::TemplateContext;
use crate::value::TemplateValue;
use crate::utils::html_escape;
use crate::bytecode::{CompiledTemplate, TemplateCompiler, BytecodeExecutor};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use std::thread;

/// Template engine for rendering HTML templates
#[derive(Debug, Clone)]
pub struct TemplateEngine {
    template_dir: String,
    cache: HashMap<String, String>,
    bytecode_cache_enabled: bool,
    bytecode_cache: HashMap<String, CompiledTemplate>,
    compiler: TemplateCompiler,
    executor: BytecodeExecutor,
}

impl TemplateEngine {
    /// Create a new template engine
    pub fn new(template_dir: &str) -> Self {
        Self {
            template_dir: template_dir.to_string(),
            cache: HashMap::new(),
            bytecode_cache_enabled: false,
            bytecode_cache: HashMap::new(),
            compiler: TemplateCompiler::new(),
            executor: BytecodeExecutor::new(),
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

    /// Get variable value with support for deep dot notation (object.property.nested.value)
    fn get_variable_value(&self, var_name: &str, context: &TemplateContext) -> String {
        if var_name.contains('.') {
            let parts: Vec<&str> = var_name.split('.').collect();
            if let Some(root_value) = context.variables.get(parts[0]) {
                return self.traverse_nested_value(root_value, &parts[1..]);
            }
            String::new()
        } else {
            context.get_string(var_name).unwrap_or_default()
        }
    }

    /// Recursively traverse nested object properties
    fn traverse_nested_value(&self, current_value: &TemplateValue, remaining_parts: &[&str]) -> String {
        if remaining_parts.is_empty() {
            // We've reached the end of the path, convert the value to string
            return match current_value {
                TemplateValue::String(s) => s.clone(),
                TemplateValue::Bool(b) => b.to_string(),
                TemplateValue::Number(n) => n.to_string(),
                TemplateValue::Array(_) => String::new(), // Arrays render as empty string when accessed directly
                TemplateValue::Object(_) => String::new(), // Objects render as empty string when accessed directly
            };
        }

        // We still have more parts to traverse
        let current_part = remaining_parts[0];
        let next_parts = &remaining_parts[1..];

        match current_value {
            TemplateValue::Object(obj) => {
                if let Some(next_value) = obj.get(current_part) {
                    self.traverse_nested_value(next_value, next_parts)
                } else {
                    String::new() // Property not found
                }
            }
            TemplateValue::Array(arr) => {
                // Support array indexing with numeric strings (optional enhancement)
                if let Ok(index) = current_part.parse::<usize>() {
                    if let Some(element) = arr.get(index) {
                        self.traverse_nested_value(element, next_parts)
                    } else {
                        String::new() // Index out of bounds
                    }
                } else {
                    String::new() // Invalid array index
                }
            }
            _ => String::new(), // Can't traverse further on non-object/non-array values
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
        // Support both simple variables and deep dot notation in conditionals
        if condition.contains('.') {
            let parts: Vec<&str> = condition.split('.').collect();
            if let Some(root_value) = context.variables.get(parts[0]) {
                return self.evaluate_nested_condition(root_value, &parts[1..]);
            }
            false
        } else if let Some(value) = context.variables.get(condition) {
            self.is_truthy(value)
        } else {
            false
        }
    }

    /// Evaluate condition for nested properties
    fn evaluate_nested_condition(&self, current_value: &TemplateValue, remaining_parts: &[&str]) -> bool {
        if remaining_parts.is_empty() {
            return self.is_truthy(current_value);
        }

        let current_part = remaining_parts[0];
        let next_parts = &remaining_parts[1..];

        match current_value {
            TemplateValue::Object(obj) => {
                if let Some(next_value) = obj.get(current_part) {
                    self.evaluate_nested_condition(next_value, next_parts)
                } else {
                    false // Property not found
                }
            }
            TemplateValue::Array(arr) => {
                // Support array indexing in conditionals too
                if let Ok(index) = current_part.parse::<usize>() {
                    if let Some(element) = arr.get(index) {
                        self.evaluate_nested_condition(element, next_parts)
                    } else {
                        false // Index out of bounds
                    }
                } else {
                    false // Invalid array index
                }
            }
            _ => false, // Can't traverse further
        }
    }

    /// Check if a value is truthy
    fn is_truthy(&self, value: &TemplateValue) -> bool {
        match value {
            TemplateValue::Bool(b) => *b,
            TemplateValue::String(s) => !s.is_empty(),
            TemplateValue::Number(n) => *n != 0,
            TemplateValue::Array(a) => !a.is_empty(),
            TemplateValue::Object(o) => !o.is_empty(),
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
    
    // Performance features for TDD
    
    /// Render multiple templates in parallel
    pub fn render_parallel(&mut self, template_names: &[String], context: &TemplateContext) -> TemplateResult<Vec<String>> {
        let context = Arc::new(context.clone());
        let template_dir = Arc::new(self.template_dir.clone());
        
        let handles: Vec<_> = template_names.iter().map(|name| {
            let name = name.clone();
            let context = Arc::clone(&context);
            let template_dir = Arc::clone(&template_dir);
            
            thread::spawn(move || {
                let mut engine = TemplateEngine::new(&template_dir);
                engine.render(&name, &context)
            })
        }).collect();
        
        let mut results = Vec::new();
        for handle in handles {
            let result = handle.join().map_err(|_| TemplateError::Render("Thread panic".to_string()))??;
            results.push(result);
        }
        
        Ok(results)
    }
    
    /// Load template using memory mapping (minimal implementation)
    /// In production, this would use memmap2 crate for true memory mapping
    pub fn load_template_mmap(&mut self, name: &str) -> TemplateResult<String> {
        // Check cache first for memory efficiency
        if let Some(cached) = self.cache.get(name) {
            return Ok(cached.clone());
        }

        let path = Path::new(&self.template_dir).join(name);
        let content = fs::read_to_string(&path)
            .map_err(|e| TemplateError::Template(format!("Failed to mmap template '{}': {}", name, e)))?;

        // In a real implementation with memmap2:
        // let file = File::open(&path)?;
        // let mmap = unsafe { MmapOptions::new().map(&file)? };
        // let content = std::str::from_utf8(&mmap)?;
        
        self.cache.insert(name.to_string(), content.clone());
        Ok(content)
    }
    
    /// Compile template to bytecode
    pub fn compile_to_bytecode(&mut self, template_name: &str) -> TemplateResult<CompiledTemplate> {
        if self.bytecode_cache_enabled {
            if let Some(cached) = self.bytecode_cache.get(template_name) {
                return Ok(cached.clone());
            }
        }
        
        let template_content = self.load_template(template_name)?;
        let instructions = self.compiler.compile(&template_content)?;
        let compiled = CompiledTemplate::new(template_name.to_string(), instructions);
        
        if self.bytecode_cache_enabled {
            self.bytecode_cache.insert(template_name.to_string(), compiled.clone());
        }
        
        Ok(compiled)
    }
    
    /// Compile template to bytecode without caching
    pub fn compile_to_bytecode_uncached(&mut self, template_name: &str) -> TemplateResult<CompiledTemplate> {
        let template_content = self.load_template(template_name)?;
        let instructions = self.compiler.compile(&template_content)?;
        Ok(CompiledTemplate::new(template_name.to_string(), instructions))
    }
    
    /// Render compiled template
    pub fn render_compiled(&self, compiled_template: &CompiledTemplate, context: &TemplateContext) -> TemplateResult<String> {
        self.executor.execute(&compiled_template.instructions, context)
    }
    
    /// Check if template is cached in bytecode cache
    pub fn is_bytecode_cached(&self, template_name: &str) -> bool {
        self.bytecode_cache.contains_key(template_name)
    }
    
    /// Enable or disable bytecode caching
    pub fn enable_bytecode_cache(&mut self, enabled: bool) {
        self.bytecode_cache_enabled = enabled;
        if !enabled {
            self.bytecode_cache.clear();
        }
    }
    
    /// Compile multiple templates in parallel
    pub fn compile_templates_parallel(&mut self, template_names: &[String]) -> TemplateResult<Vec<CompiledTemplate>> {
        let template_dir = Arc::new(self.template_dir.clone());
        
        let handles: Vec<_> = template_names.iter().map(|name| {
            let name = name.clone();
            let template_dir = Arc::clone(&template_dir);
            
            thread::spawn(move || {
                let mut engine = TemplateEngine::new(&template_dir);
                engine.compile_to_bytecode(&name)
            })
        }).collect();
        
        let mut results = Vec::new();
        for handle in handles {
            let result = handle.join().map_err(|_| TemplateError::Render("Thread panic".to_string()))??;
            results.push(result);
        }
        
        Ok(results)
    }
    
    /// Render multiple compiled templates in parallel
    pub fn render_compiled_parallel(&self, compiled_templates: &[CompiledTemplate], context: &TemplateContext) -> TemplateResult<Vec<String>> {
        let context = Arc::new(context.clone());
        let executor = Arc::new(self.executor.clone());
        
        let handles: Vec<_> = compiled_templates.iter().map(|template| {
            let template = template.clone();
            let context = Arc::clone(&context);
            let executor: Arc<BytecodeExecutor> = Arc::clone(&executor);
            
            thread::spawn(move || {
                executor.execute(&template.instructions, &context)
            })
        }).collect();
        
        let mut results = Vec::new();
        for handle in handles {
            let result = handle.join().map_err(|_| TemplateError::Render("Thread panic".to_string()))??;
            results.push(result);
        }
        
        Ok(results)
    }
}