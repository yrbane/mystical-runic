//! Bytecode compilation and execution for templates

use crate::error::{TemplateError, TemplateResult};
use crate::context::TemplateContext;

/// Bytecode instruction for template execution
#[derive(Debug, Clone)]
pub enum BytecodeInstruction {
    /// Output literal text
    OutputLiteral(String),
    /// Output variable value (with path for dot notation)
    OutputVariable(Vec<String>),
    /// Output raw variable value (unescaped)
    OutputRaw(Vec<String>),
    /// Conditional jump if variable is falsy
    JumpIfFalsy(Vec<String>, usize),
    /// Unconditional jump
    Jump(usize),
    /// Start loop over array variable
    StartLoop(String, Vec<String>),
    /// End loop and jump back
    EndLoop(usize),
    /// No operation
    Nop,
}

/// Compiled template bytecode
#[derive(Debug, Clone)]
pub struct CompiledTemplate {
    pub name: String,
    pub instructions: Vec<BytecodeInstruction>,
    pub compilation_time: std::time::Instant,
}

impl CompiledTemplate {
    pub fn new(name: String, instructions: Vec<BytecodeInstruction>) -> Self {
        Self {
            name,
            instructions,
            compilation_time: std::time::Instant::now(),
        }
    }
}

/// Simple template compiler (minimal implementation for GREEN phase)
#[derive(Debug, Clone)]
pub struct TemplateCompiler;

impl TemplateCompiler {
    pub fn new() -> Self {
        Self
    }

    /// Compile template string to bytecode
    pub fn compile(&self, template: &str) -> TemplateResult<Vec<BytecodeInstruction>> {
        let mut instructions = Vec::new();
        let mut pos = 0;
        let chars: Vec<char> = template.chars().collect();
        
        while pos < chars.len() {
            if pos < chars.len() - 1 && chars[pos] == '{' && chars[pos + 1] == '{' {
                // Find the end of the directive
                let mut end_pos = pos + 2;
                while end_pos < chars.len() - 1 {
                    if chars[end_pos] == '}' && chars[end_pos + 1] == '}' {
                        break;
                    }
                    end_pos += 1;
                }
                
                if end_pos >= chars.len() - 1 {
                    return Err(TemplateError::Parse("Unclosed directive".to_string()));
                }
                
                let directive: String = chars[pos + 2..end_pos].iter().collect();
                let directive = directive.trim();
                
                // Simple parsing (minimal for GREEN phase)
                if let Some(stripped) = directive.strip_prefix("if ") {
                    let var_name = stripped.trim();
                    let path = Self::parse_variable_path(var_name);
                    instructions.push(BytecodeInstruction::JumpIfFalsy(path, 0)); // Will be fixed up later
                } else if directive == "/if" {
                    instructions.push(BytecodeInstruction::Nop);
                } else if directive.starts_with("for ") {
                    // Parse for loop
                    let parts: Vec<&str> = directive.split_whitespace().collect();
                    if parts.len() >= 4 && parts[2] == "in" {
                        let item_var = parts[1].to_string();
                        let array_path = Self::parse_variable_path(parts[3]);
                        instructions.push(BytecodeInstruction::StartLoop(item_var, array_path));
                    }
                } else if directive == "/for" {
                    instructions.push(BytecodeInstruction::EndLoop(0)); // Will be fixed up later
                } else if let Some(stripped) = directive.strip_prefix("& ") {
                    let var_name = stripped.trim();
                    let path = Self::parse_variable_path(var_name);
                    instructions.push(BytecodeInstruction::OutputRaw(path));
                } else {
                    // Regular variable
                    let path = Self::parse_variable_path(directive);
                    instructions.push(BytecodeInstruction::OutputVariable(path));
                }
                
                pos = end_pos + 2;
            } else {
                // Find next directive or end of string
                let mut text_end = pos;
                while text_end < chars.len() {
                    if text_end < chars.len() - 1 && chars[text_end] == '{' && chars[text_end + 1] == '{' {
                        break;
                    }
                    text_end += 1;
                }
                
                if text_end > pos {
                    let literal: String = chars[pos..text_end].iter().collect();
                    instructions.push(BytecodeInstruction::OutputLiteral(literal));
                }
                
                pos = text_end;
            }
        }
        
        Ok(instructions)
    }
    
    fn parse_variable_path(var_name: &str) -> Vec<String> {
        var_name.split('.').map(|s| s.to_string()).collect()
    }
}

/// Bytecode executor (minimal implementation for GREEN phase)
#[derive(Debug, Clone)]
pub struct BytecodeExecutor;

impl BytecodeExecutor {
    pub fn new() -> Self {
        Self
    }
    
    /// Execute compiled bytecode
    pub fn execute(&self, instructions: &[BytecodeInstruction], context: &TemplateContext) -> TemplateResult<String> {
        let mut output = String::new();
        let mut pc = 0; // program counter
        
        while pc < instructions.len() {
            match &instructions[pc] {
                BytecodeInstruction::OutputLiteral(text) => {
                    output.push_str(text);
                }
                BytecodeInstruction::OutputVariable(path) => {
                    let value = self.resolve_variable_path(path, context);
                    output.push_str(&self.escape_html(&value));
                }
                BytecodeInstruction::OutputRaw(path) => {
                    let value = self.resolve_variable_path(path, context);
                    output.push_str(&value);
                }
                BytecodeInstruction::JumpIfFalsy(path, _target) => {
                    // Check if the condition is truthy using proper value evaluation
                    let is_truthy = if path.len() == 1 {
                        if let Some(value) = context.variables.get(&path[0]) {
                            self.is_truthy_value(value)
                        } else {
                            false
                        }
                    } else {
                        // Deep dot notation
                        if let Some(root_value) = context.variables.get(&path[0]) {
                            let nested_value = self.get_nested_value(root_value, &path[1..]);
                            nested_value.is_some_and(|v| self.is_truthy_value(v))
                        } else {
                            false
                        }
                    };
                    
                    if !is_truthy {
                        // Skip to /if (simplified - find next Nop)
                        while pc < instructions.len() && !matches!(instructions[pc], BytecodeInstruction::Nop) {
                            pc += 1;
                        }
                    }
                }
                BytecodeInstruction::Jump(_target) => {
                    // Simplified jump
                }
                BytecodeInstruction::StartLoop(_item_var, _array_path) => {
                    // Simplified loop handling - just continue for now
                }
                BytecodeInstruction::EndLoop(_target) => {
                    // Simplified loop handling
                }
                BytecodeInstruction::Nop => {
                    // Do nothing
                }
            }
            pc += 1;
        }
        
        Ok(output)
    }
    
    fn resolve_variable_path(&self, path: &[String], context: &TemplateContext) -> String {
        if path.is_empty() {
            return String::new();
        }
        
        if path.len() == 1 {
            // Simple variable lookup
            context.get_string(&path[0]).unwrap_or_default()
        } else {
            // Deep dot notation traversal
            if let Some(root_value) = context.variables.get(&path[0]) {
                self.traverse_nested_value(root_value, &path[1..])
            } else {
                String::new()
            }
        }
    }
    
    #[allow(clippy::only_used_in_recursion)]
    fn traverse_nested_value(&self, current_value: &crate::value::TemplateValue, remaining_parts: &[String]) -> String {
        use crate::value::TemplateValue;
        
        if remaining_parts.is_empty() {
            return match current_value {
                TemplateValue::String(s) => s.clone(),
                TemplateValue::Bool(b) => b.to_string(),
                TemplateValue::Number(n) => n.to_string(),
                TemplateValue::Array(_) => String::new(),
                TemplateValue::Object(_) => String::new(),
            };
        }

        let current_part = &remaining_parts[0];
        let next_parts = &remaining_parts[1..];

        match current_value {
            TemplateValue::Object(obj) => {
                if let Some(next_value) = obj.get(current_part) {
                    self.traverse_nested_value(next_value, next_parts)
                } else {
                    String::new()
                }
            }
            TemplateValue::Array(arr) => {
                if let Ok(index) = current_part.parse::<usize>() {
                    if let Some(element) = arr.get(index) {
                        self.traverse_nested_value(element, next_parts)
                    } else {
                        String::new()
                    }
                } else {
                    String::new()
                }
            }
            _ => String::new(),
        }
    }
    
    fn escape_html(&self, text: &str) -> String {
        text.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&#x27;")
    }
    
    #[allow(dead_code)]
    fn is_truthy_string(&self, value: &str) -> bool {
        !value.is_empty() && value != "0" && value != "false"
    }
    
    fn is_truthy_value(&self, value: &crate::value::TemplateValue) -> bool {
        use crate::value::TemplateValue;
        
        match value {
            TemplateValue::Bool(b) => *b,
            TemplateValue::String(s) => !s.is_empty(),
            TemplateValue::Number(n) => *n != 0,
            TemplateValue::Array(a) => !a.is_empty(),
            TemplateValue::Object(o) => !o.is_empty(),
        }
    }
    
    #[allow(clippy::only_used_in_recursion)]
    fn get_nested_value<'a>(&self, current_value: &'a crate::value::TemplateValue, remaining_parts: &[String]) -> Option<&'a crate::value::TemplateValue> {
        use crate::value::TemplateValue;
        
        if remaining_parts.is_empty() {
            return Some(current_value);
        }

        let current_part = &remaining_parts[0];
        let next_parts = &remaining_parts[1..];

        match current_value {
            TemplateValue::Object(obj) => {
                if let Some(next_value) = obj.get(current_part) {
                    self.get_nested_value(next_value, next_parts)
                } else {
                    None
                }
            }
            TemplateValue::Array(arr) => {
                if let Ok(index) = current_part.parse::<usize>() {
                    if let Some(element) = arr.get(index) {
                        self.get_nested_value(element, next_parts)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}