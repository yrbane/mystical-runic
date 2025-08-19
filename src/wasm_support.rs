//! WASM compatibility support for v0.5.0
//!
//! Browser and edge runtime support for template rendering

#[cfg(feature = "wasm")]
use crate::{TemplateEngine, TemplateContext, TemplateResult};

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "wasm")]
use web_sys::console;

/// WASM-specific template engine extensions
#[cfg(feature = "wasm")]
pub trait WasmTemplateEngine {
    /// Render template in WASM environment
    fn render_string_wasm(&mut self, template: &str, context: &TemplateContext) -> TemplateResult<String>;
    
    /// Enable console logging for WASM debugging
    fn set_wasm_console_logging(&mut self, enabled: bool);
    
    /// Get current WASM memory usage
    fn get_wasm_memory_usage(&self) -> usize;
    
    /// Check WASM compatibility
    fn is_wasm_compatible(&self) -> bool;
}

#[cfg(feature = "wasm")]
impl WasmTemplateEngine for TemplateEngine {
    fn render_string_wasm(&mut self, template: &str, context: &TemplateContext) -> TemplateResult<String> {
        // Log to browser console if enabled and in WASM environment
        if self.get_wasm_console_logging() && cfg!(target_arch = "wasm32") {
            console::log_1(&format!("Rendering template in WASM: {}", template.len()).into());
        }
        
        // Use the regular render_string but with WASM optimizations
        let result = self.render_string(template, context)?;
        
        // Log successful rendering in WASM environment
        if self.get_wasm_console_logging() && cfg!(target_arch = "wasm32") {
            console::log_1(&format!("Template rendered successfully: {} chars", result.len()).into());
        }
        
        Ok(result)
    }

    fn set_wasm_console_logging(&mut self, enabled: bool) {
        // Use the engine's method (avoid naming conflict)
        TemplateEngine::set_wasm_console_logging(self, enabled);
        
        if enabled && cfg!(target_arch = "wasm32") {
            console::log_1(&"WASM console logging enabled for Mystical-Runic".into());
        }
    }

    fn get_wasm_memory_usage(&self) -> usize {
        // Estimate memory usage (simplified implementation)
        let mut usage = 0;
        
        // Template cache memory (use public getter)
        usage += self.get_cache_size() * 1000; // Rough estimate
        
        // Macros
        usage += self.get_macro_count() * 500;
        
        // Use conservative estimates for private structures
        usage += 1000; // Base engine size
        
        usage
    }

    fn is_wasm_compatible(&self) -> bool {
        // Check if all features used are WASM-compatible
        true // For now, assume compatible
    }
}

/// WASM-specific engine extensions
#[cfg(feature = "wasm")]
impl TemplateEngine {
    /// Create a new WASM-optimized template engine
    pub fn new_wasm(template_dir: &str) -> Self {
        let mut engine = Self::new(template_dir);
        engine.set_wasm_console_logging(false); // Disabled by default
        engine
    }
}

#[cfg(feature = "wasm")]
/// JavaScript bindings for WASM
#[wasm_bindgen]
pub struct WasmRuneEngine {
    engine: TemplateEngine,
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
impl WasmRuneEngine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            engine: TemplateEngine::new_wasm("."),
        }
    }

    #[wasm_bindgen(js_name = renderString)]
    pub fn render_string(&mut self, template: &str, context_json: &str) -> Result<String, JsValue> {
        // Parse JSON context
        let context = self.parse_json_context(context_json)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        
        // Render template
        self.engine.render_string_wasm(template, &context)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = enableConsoleLogging)]
    pub fn enable_console_logging(&mut self, enabled: bool) {
        self.engine.set_wasm_console_logging(enabled);
    }

    #[wasm_bindgen(js_name = getMemoryUsage)]
    pub fn get_memory_usage(&self) -> usize {
        self.engine.get_wasm_memory_usage()
    }
    
    fn parse_json_context(&self, json: &str) -> TemplateResult<TemplateContext> {
        // Simplified JSON parsing for WASM
        // In a real implementation, we'd use serde_json
        let mut context = TemplateContext::new();
        
        // Basic JSON parsing (simplified for now)
        if json.trim().starts_with('{') {
            // TODO: Implement proper JSON parsing
            context.set_string("json_data", "parsed from JSON");
        }
        
        Ok(context)
    }
}

#[cfg(not(feature = "wasm"))]
/// Placeholder when WASM feature is not enabled
#[allow(dead_code)]
pub struct WasmPlaceholder;