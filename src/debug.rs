//! Debugging capabilities for v0.4.0 Developer Experience
//! 
//! Provides step-through debugging, variable tracking, and execution insights

/// Debug information collected during template rendering
#[derive(Debug, Clone)]
pub struct DebugInfo {
    /// Variables accessed during rendering
    pub variables_accessed: Vec<String>,
    /// Templates processed (including includes)
    pub templates_processed: Vec<String>,
    /// Step-by-step execution trace
    pub execution_steps: Vec<ExecutionStep>,
    /// Performance metrics
    pub performance_metrics: PerformanceMetrics,
}

/// Individual step in template execution
#[derive(Debug, Clone)]
pub struct ExecutionStep {
    /// Type of step (variable, conditional, loop, include, etc.)
    pub step_type: String,
    /// Content being processed
    pub content: String,
    /// Line number in template
    pub line: usize,
    /// Column number in template
    pub column: usize,
    /// Execution time for this step
    pub duration_nanos: u64,
    /// Result of this step
    pub result: Option<String>,
}

/// Performance metrics for debugging
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    /// Total rendering time in nanoseconds
    pub total_time_nanos: u64,
    /// Time spent on variable resolution
    pub variable_resolution_nanos: u64,
    /// Time spent on template parsing
    pub parsing_nanos: u64,
    /// Time spent on includes
    pub include_nanos: u64,
    /// Memory usage in bytes
    pub memory_usage_bytes: usize,
}

/// Result of template rendering with debug information
#[derive(Debug)]
pub struct DebugRenderResult {
    /// The rendered output
    pub output: String,
    /// Debug information collected
    pub debug_info: DebugInfo,
}

impl DebugInfo {
    /// Create new empty debug info
    pub fn new() -> Self {
        Self {
            variables_accessed: Vec::new(),
            templates_processed: Vec::new(),
            execution_steps: Vec::new(),
            performance_metrics: PerformanceMetrics::new(),
        }
    }
    
    /// Add a variable access to the debug trace
    pub fn add_variable_access(&mut self, variable: &str) {
        if !self.variables_accessed.contains(&variable.to_string()) {
            self.variables_accessed.push(variable.to_string());
        }
    }
    
    /// Add a template processing to the debug trace
    pub fn add_template_processed(&mut self, template: &str) {
        self.templates_processed.push(template.to_string());
    }
    
    /// Add an execution step to the debug trace
    pub fn add_execution_step(&mut self, step: ExecutionStep) {
        self.execution_steps.push(step);
    }
}

impl PerformanceMetrics {
    /// Create new empty performance metrics
    pub fn new() -> Self {
        Self {
            total_time_nanos: 0,
            variable_resolution_nanos: 0,
            parsing_nanos: 0,
            include_nanos: 0,
            memory_usage_bytes: 0,
        }
    }
}

impl ExecutionStep {
    /// Create a new execution step
    pub fn new(step_type: &str, content: &str, line: usize, column: usize) -> Self {
        Self {
            step_type: step_type.to_string(),
            content: content.to_string(),
            line,
            column,
            duration_nanos: 0,
            result: None,
        }
    }
    
    /// Set the result of this execution step
    pub fn with_result(mut self, result: String) -> Self {
        self.result = Some(result);
        self
    }
    
    /// Set the execution duration
    pub fn with_duration(mut self, duration_nanos: u64) -> Self {
        self.duration_nanos = duration_nanos;
        self
    }
}