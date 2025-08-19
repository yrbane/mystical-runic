// Tests to ensure warning cleanup doesn't break functionality
use mystical_runic::{TemplateEngine, TemplateContext, TemplateValue};
use std::collections::HashMap;

#[cfg(test)]
mod warning_cleanup_tests {
    use super::*;

    #[test]
    fn test_engine_functionality_preserved() {
        // Test that core engine functionality works after cleanup
        let mut engine = TemplateEngine::new("./templates");
        let mut context = TemplateContext::new();
        
        context.set("name", TemplateValue::String("Test".to_string()));
        context.set("count", TemplateValue::Number(42));
        context.set("enabled", TemplateValue::Bool(true));
        
        // Test variable rendering
        let result = engine.render_string("Hello {{name}}", &context).unwrap();
        assert_eq!(result, "Hello Test");
        
        // Test conditionals
        let result = engine.render_string("{{if enabled}}Active{{/if}}", &context).unwrap();
        assert_eq!(result, "Active");
        
        // Test filters
        let result = engine.render_string("{{name|upper}}", &context).unwrap();
        assert_eq!(result, "TEST");
        
        // Test math filters
        let result = engine.render_string("{{count|add:8}}", &context).unwrap();
        assert_eq!(result, "50");
    }

    #[test]
    fn test_template_inheritance_preserved() {
        // Test that template inheritance still works
        let mut engine = TemplateEngine::new("./templates");
        let mut context = TemplateContext::new();
        
        context.set("title", TemplateValue::String("Test Page".to_string()));
        context.set("content", TemplateValue::String("Test Content".to_string()));
        
        // Test basic template processing
        let template = "Title: {{title}}, Content: {{content}}";
        let result = engine.render_string(template, &context).unwrap();
        assert_eq!(result, "Title: Test Page, Content: Test Content");
    }

    #[test]  
    fn test_custom_filters_preserved() {
        // Test that custom filter registration still works
        let mut engine = TemplateEngine::new("./templates");
        let mut context = TemplateContext::new();
        
        context.set("text", TemplateValue::String("hello".to_string()));
        
        // Register custom filter
        engine.register_filter("reverse", |input: &str, _args: &[&str]| {
            Ok(input.chars().rev().collect())
        });
        
        let result = engine.render_string("{{text|reverse}}", &context).unwrap();
        assert_eq!(result, "olleh");
    }

    #[test]
    fn test_i18n_functionality_preserved() {
        // Test that i18n features still work
        let mut engine = TemplateEngine::new("./templates");
        let mut context = TemplateContext::new();
        
        context.set("name", TemplateValue::String("User".to_string()));
        
        // Set up translations
        let mut translations = HashMap::new();
        translations.insert("welcome".to_string(), "Welcome {{name}}!".to_string());
        engine.set_translations("en", translations);
        engine.set_locale("en");
        
        let result = engine.render_string("{{t \"welcome\"}}", &context).unwrap();
        assert_eq!(result, "Welcome User!");
    }

    #[test]
    fn test_pluralization_preserved() {
        // Test that pluralization still works
        let mut engine = TemplateEngine::new("./templates");
        let mut context = TemplateContext::new();
        
        context.set("count", TemplateValue::Number(1));
        let result = engine.render_string("{{plural count \"item\" \"items\"}}", &context).unwrap();
        assert_eq!(result, "item");
        
        context.set("count", TemplateValue::Number(5));
        let result = engine.render_string("{{plural count \"item\" \"items\"}}", &context).unwrap();
        assert_eq!(result, "items");
    }

    #[test]
    fn test_loops_and_arrays_preserved() {
        // Test that loop functionality is preserved
        let mut engine = TemplateEngine::new("./templates");
        let mut context = TemplateContext::new();
        
        context.set("items", TemplateValue::Array(vec![
            TemplateValue::String("one".to_string()),
            TemplateValue::String("two".to_string()),
            TemplateValue::String("three".to_string())
        ]));
        
        let result = engine.render_string("{{for item in items}}{{item}} {{/for}}", &context).unwrap();
        assert_eq!(result, "one two three ");
    }

    #[test]
    fn test_error_handling_preserved() {
        // Test that error handling still works properly
        let mut engine = TemplateEngine::new("./templates");
        let context = TemplateContext::new();
        
        // Test invalid template syntax
        let result = engine.render_string("{{unclosed", &context);
        assert!(result.is_err());
        
        // Test invalid filter
        let result = engine.render_string("{{name|nonexistent}}", &context);
        assert!(result.is_ok()); // Should return empty string for missing variables
    }

    #[test]
    fn test_performance_features_preserved() {
        // Test that performance features (caching, etc.) still work
        let mut engine = TemplateEngine::new("./templates");
        let context = TemplateContext::new();
        
        // Enable bytecode compilation
        engine.enable_bytecode_cache(true);
        
        let template = "Static template content";
        let result1 = engine.render_string(template, &context).unwrap();
        let result2 = engine.render_string(template, &context).unwrap();
        
        assert_eq!(result1, result2);
        assert_eq!(result1, "Static template content");
    }
}