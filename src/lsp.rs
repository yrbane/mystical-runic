//! Language Server Protocol support for v0.4.1 IDE Integration
//! 
//! Provides LSP-compatible structures and functionality for template editing


/// LSP parsing result containing template analysis
#[derive(Debug, Clone)]
pub struct LspParseResult {
    /// All variables found in the template
    pub variables: Vec<String>,
    /// All template blocks (if, for, etc.)
    pub blocks: Vec<TemplateBlock>,
    /// All filters used
    pub filters: Vec<String>,
    /// All macros defined or called
    pub macros: Vec<String>,
}

/// Template block information for LSP
#[derive(Debug, Clone)]
pub struct TemplateBlock {
    /// Type of block (if, for, macro, etc.)
    pub block_type: String,
    /// Starting line (1-based)
    pub start_line: usize,
    /// Starting column (1-based)
    pub start_column: usize,
    /// Ending line (1-based)
    pub end_line: usize,
    /// Ending column (1-based)
    pub end_column: usize,
    /// Block content/condition
    pub content: String,
}

/// Completion item for auto-completion
#[derive(Debug, Clone)]
pub struct CompletionItem {
    /// Display label for the completion
    pub label: String,
    /// Type of completion (variable, filter, directive, etc.)
    pub completion_type: String,
    /// Additional detail about the item
    pub detail: String,
    /// Documentation for the item
    pub documentation: Option<String>,
    /// Text to insert when completing
    pub insert_text: Option<String>,
}

/// Syntax highlighting token
#[derive(Debug, Clone)]
pub struct SyntaxToken {
    /// Token content
    pub content: String,
    /// Token type for highlighting
    pub token_type: String,
    /// Start position in document
    pub start_position: usize,
    /// End position in document
    pub end_position: usize,
    /// Line number (1-based)
    pub line: usize,
    /// Column number (1-based)
    pub column: usize,
}

/// Diagnostic information for error squiggles
#[derive(Debug, Clone)]
pub struct Diagnostic {
    /// Error/warning message
    pub message: String,
    /// Severity (error, warning, info, hint)
    pub severity: String,
    /// Line number (1-based)
    pub line: usize,
    /// Column number (1-based)
    pub column: usize,
    /// End line (1-based)
    pub end_line: usize,
    /// End column (1-based)
    pub end_column: usize,
    /// Error code if applicable
    pub code: Option<String>,
}

/// Hover information for variables
#[derive(Debug, Clone)]
pub struct HoverInfo {
    /// Variable name being hovered
    pub variable_name: String,
    /// Type of the variable
    pub variable_type: String,
    /// Current value as string
    pub current_value: String,
    /// Description/documentation
    pub description: String,
}

/// Definition location information
#[derive(Debug, Clone)]
pub struct DefinitionInfo {
    /// Type of definition (macro, variable, etc.)
    pub definition_type: String,
    /// Name of the defined item
    pub name: String,
    /// Line where defined (1-based)
    pub line: usize,
    /// Column where defined (1-based)
    pub column: usize,
    /// File path (if different)
    pub file_path: Option<String>,
}

impl LspParseResult {
    /// Create new empty LSP parse result
    pub fn new() -> Self {
        Self {
            variables: Vec::new(),
            blocks: Vec::new(),
            filters: Vec::new(),
            macros: Vec::new(),
        }
    }
    
    /// Add a variable to the result
    pub fn add_variable(&mut self, name: &str) {
        if !self.variables.contains(&name.to_string()) {
            self.variables.push(name.to_string());
        }
    }
    
    /// Add a template block to the result
    pub fn add_block(&mut self, block: TemplateBlock) {
        self.blocks.push(block);
    }
    
    /// Add a filter to the result
    pub fn add_filter(&mut self, name: &str) {
        if !self.filters.contains(&name.to_string()) {
            self.filters.push(name.to_string());
        }
    }
}

impl TemplateBlock {
    /// Create a new template block
    pub fn new(block_type: &str, start_line: usize, start_column: usize, content: &str) -> Self {
        Self {
            block_type: block_type.to_string(),
            start_line,
            start_column,
            end_line: start_line, // Will be updated when block ends
            end_column: start_column,
            content: content.to_string(),
        }
    }
}

impl CompletionItem {
    /// Create a new completion item
    pub fn new(label: &str, completion_type: &str, detail: &str) -> Self {
        Self {
            label: label.to_string(),
            completion_type: completion_type.to_string(),
            detail: detail.to_string(),
            documentation: None,
            insert_text: None,
        }
    }
    
    /// Set documentation for the completion
    pub fn with_documentation(mut self, doc: &str) -> Self {
        self.documentation = Some(doc.to_string());
        self
    }
    
    /// Set custom insert text
    pub fn with_insert_text(mut self, text: &str) -> Self {
        self.insert_text = Some(text.to_string());
        self
    }
}

impl SyntaxToken {
    /// Create a new syntax token
    pub fn new(content: &str, token_type: &str, start_pos: usize, line: usize, column: usize) -> Self {
        Self {
            content: content.to_string(),
            token_type: token_type.to_string(),
            start_position: start_pos,
            end_position: start_pos + content.len(),
            line,
            column,
        }
    }
}

impl Diagnostic {
    /// Create a new diagnostic
    pub fn new(message: &str, severity: &str, line: usize, column: usize) -> Self {
        Self {
            message: message.to_string(),
            severity: severity.to_string(),
            line,
            column,
            end_line: line,
            end_column: column,
            code: None,
        }
    }
    
    /// Set the range for the diagnostic
    pub fn with_range(mut self, end_line: usize, end_column: usize) -> Self {
        self.end_line = end_line;
        self.end_column = end_column;
        self
    }
    
    /// Set an error code
    pub fn with_code(mut self, code: &str) -> Self {
        self.code = Some(code.to_string());
        self
    }
}