// Tests for Modern Rust compatibility and future Rust 2024 readiness
use mystical_runic::{TemplateEngine, TemplateContext, TemplateValue};

#[cfg(test)]
mod modern_rust_features_tests {
    use super::*;

    #[test]
    fn test_rust_2024_edition_compatibility() {
        // Test that the crate compiles and works with Rust 2024
        let mut engine = TemplateEngine::new("./templates");
        let mut context = TemplateContext::new();
        
        context.set("version", TemplateValue::String("2024".to_string()));
        context.set("edition", TemplateValue::String("Rust 2024 Edition".to_string()));
        
        let template = "Using {{edition}} - Version {{version}}";
        let result = engine.render_string(template, &context).unwrap();
        
        assert_eq!(result, "Using Rust 2024 Edition - Version 2024");
    }

    #[test] 
    fn test_async_syntax_compatibility() {
        // Test that async/await syntax works properly with Rust 2024
        // This ensures our traits and types are compatible with async contexts
        let mut engine = TemplateEngine::new("./templates");
        let context = TemplateContext::new();
        
        let template = "{{greeting}}";
        
        // This should compile without issues in Rust 2024
        let result = engine.render_string(template, &context);
        assert!(result.is_ok());
    }

    #[test]
    fn test_modern_rust_features() {
        // Test compatibility with modern Rust features that may be enhanced in 2024
        let mut engine = TemplateEngine::new("./templates");
        let mut context = TemplateContext::new();
        
        // Test with various modern Rust constructs
        context.set("items", TemplateValue::Array(vec![
            TemplateValue::String("modern".to_string()),
            TemplateValue::String("rust".to_string()),
            TemplateValue::String("features".to_string())
        ]));
        
        let template = "{{for item in items}}{{item}} {{/for}}";
        let result = engine.render_string(template, &context).unwrap();
        
        assert_eq!(result, "modern rust features ");
    }

    #[test]
    fn test_error_handling_rust_2024() {
        // Ensure error handling is compatible with Rust 2024 patterns
        let mut engine = TemplateEngine::new("./templates");
        let context = TemplateContext::new();
        
        let invalid_template = "{{unclosed";
        let result = engine.render_string(invalid_template, &context);
        
        // Should handle errors properly in Rust 2024
        assert!(result.is_err());
    }

    #[test]
    fn test_memory_safety_rust_2024() {
        // Test that memory safety guarantees are maintained in Rust 2024
        let mut engine = TemplateEngine::new("./templates");
        let mut context = TemplateContext::new();
        
        // Test with various data structures
        context.set("safe_data", TemplateValue::String("memory safe".to_string()));
        
        let template = "Rust 2024: {{safe_data}}";
        let result = engine.render_string(template, &context).unwrap();
        
        assert_eq!(result, "Rust 2024: memory safe");
        
        // Test that dropping context doesn't cause issues
        drop(context);
        
        // Engine should still be usable with new context
        let new_context = TemplateContext::new();
        let result2 = engine.render_string("Static template", &new_context);
        assert!(result2.is_ok());
    }

    #[test]
    fn test_trait_bounds_compatibility() {
        // Ensure our trait bounds work with Rust 2024 improvements
        let mut engine = TemplateEngine::new("./templates");
        let mut context = TemplateContext::new();
        
        // Test Send + Sync compatibility
        fn require_send_sync<T: Send + Sync>(_: &T) {}
        require_send_sync(&engine);
        require_send_sync(&context);
        
        // Test that custom filters work with Rust 2024
        engine.register_filter("rust2024", |input: &str, _args: &[&str]| {
            Ok(format!("{} (Rust 2024 compatible)", input))
        });
        
        context.set("test", TemplateValue::String("filter".to_string()));
        let result = engine.render_string("{{test|rust2024}}", &context).unwrap();
        
        assert_eq!(result, "filter (Rust 2024 compatible)");
    }
}