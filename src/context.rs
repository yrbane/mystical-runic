//! Template context for variable storage

use crate::value::TemplateValue;
use std::collections::HashMap;

/// Template context containing variables for rendering
#[derive(Debug, Clone)]
pub struct TemplateContext {
    pub(crate) variables: HashMap<String, TemplateValue>,
}

impl TemplateContext {
    /// Create a new empty context
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    /// Set a variable
    pub fn set(&mut self, name: &str, value: TemplateValue) {
        self.variables.insert(name.to_string(), value);
    }

    /// Set a string variable
    pub fn set_string(&mut self, name: &str, value: &str) {
        self.set(name, TemplateValue::String(value.to_string()));
    }

    /// Set a boolean variable
    pub fn set_bool(&mut self, name: &str, value: bool) {
        self.set(name, TemplateValue::Bool(value));
    }

    /// Set a number variable
    pub fn set_number(&mut self, name: &str, value: i64) {
        self.set(name, TemplateValue::Number(value));
    }

    /// Get a variable value
    pub fn get(&self, name: &str) -> Option<&TemplateValue> {
        self.variables.get(name)
    }

    /// Get a string value
    pub fn get_string(&self, name: &str) -> Option<String> {
        self.variables.get(name).map(|v| match v {
            TemplateValue::String(s) => s.clone(),
            TemplateValue::Bool(b) => b.to_string(),
            TemplateValue::Number(n) => n.to_string(),
            _ => String::new(),
        })
    }
}

impl Default for TemplateContext {
    fn default() -> Self {
        Self::new()
    }
}