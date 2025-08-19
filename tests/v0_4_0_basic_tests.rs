//! Basic tests for v0.4.0 Developer Experience features
//! Following strict TDD methodology: RED → GREEN → REFACTOR

use mystical_runic::{RuneEngine, RuneScroll, TemplateError};

#[cfg(test)]
mod basic_v040_tests {
    use super::*;

    #[test]
    fn test_debug_mode_can_be_enabled() {
        // 🟢 GREEN: Test basic debug functionality
        let mut engine = RuneEngine::new(".");
        
        assert!(!engine.is_debug_enabled());
        
        engine.enable_debug_mode();
        assert!(engine.is_debug_enabled());
        
        engine.disable_debug_mode();
        assert!(!engine.is_debug_enabled());
    }

    #[test]
    fn test_hot_reload_can_be_enabled() {
        // 🟢 GREEN: Test basic hot reload functionality
        let mut engine = RuneEngine::new(".");
        
        assert!(!engine.is_hot_reload_enabled());
        
        engine.enable_hot_reload();
        assert!(engine.is_hot_reload_enabled());
        
        engine.disable_hot_reload();
        assert!(!engine.is_hot_reload_enabled());
    }

    #[test]
    fn test_render_string_with_debug_basic() {
        // 🟢 GREEN: Test debug rendering with simple template
        let mut engine = RuneEngine::new(".");
        engine.enable_debug_mode();
        
        let mut context = RuneScroll::new();
        context.set_string("name", "Alice");
        
        let template = "Hello {{name}}!";
        
        let result = engine.render_string_with_debug(template, &context).unwrap();
        
        assert_eq!(result.output, "Hello Alice!");
        assert!(!result.debug_info.variables_accessed.is_empty());
        assert!(result.debug_info.variables_accessed.contains(&"name".to_string()));
        assert!(result.debug_info.templates_processed.contains(&"inline_template".to_string()));
    }

    #[test]
    fn test_parse_error_contains_location_info() {
        // 🟢 GREEN: Test that parse errors contain line/column info
        let mut engine = RuneEngine::new(".");
        let context = RuneScroll::new();
        
        // Template with unclosed if (will trigger our error)
        let template = "{{name}}\n{{if unclosed\nmore content";
        
        match engine.render_string_v040(template, &context) {
            Err(TemplateError::ParseWithLocation { message, line, column, .. }) => {
                assert!(!message.is_empty());
                assert!(line > 0);
                assert!(column > 0);
            },
            other => panic!("Expected ParseWithLocation error, got: {:?}", other),
        }
    }

    #[test]
    fn test_simple_variable_rendering_still_works() {
        // 🟢 GREEN: Ensure we didn't break basic functionality
        let mut engine = RuneEngine::new(".");
        let mut context = RuneScroll::new();
        context.set_string("greeting", "Hello World");
        
        let result = engine.render_string("{{greeting}}", &context).unwrap();
        assert_eq!(result, "Hello World");
    }
}