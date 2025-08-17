//! Template value types

use std::collections::HashMap;

/// Template value types that can be used in templates
#[derive(Debug, Clone)]
pub enum TemplateValue {
    String(String),
    Bool(bool),
    Number(i64),
    Array(Vec<TemplateValue>),
    Object(HashMap<String, TemplateValue>),
}