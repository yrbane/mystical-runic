// Unit tests for individual modules and utility functions

use mystical_runic::*;
use std::collections::HashMap;

// Tests for utility functions
mod utils_tests {
    use super::*;

    // We'll test the html_escape function indirectly through the public API
    // since it's not directly exposed
    
    #[test]
    fn test_html_escape_comprehensive() {
        let mut engine = TemplateEngine::new("templates");
        let mut context = TemplateContext::new();
        
        // Test all HTML entities
        let test_cases = vec![
            ("<", "&lt;"),
            (">", "&gt;"),
            ("&", "&amp;"),
            ("'", "&#x27;"),
            ("\"", "&quot;"),
            ("<>&'\"", "&lt;&gt;&amp;&#x27;&quot;"),
        ];
        
        for (input, expected) in test_cases {
            context.set_string("test", input);
            let result = engine.render_string("{{test}}", &context).unwrap();
            assert_eq!(result, expected, "Failed for input: {}", input);
        }
    }

    #[test]
    fn test_html_escape_edge_cases() {
        let mut engine = TemplateEngine::new("templates");
        let mut context = TemplateContext::new();
        
        // Empty string
        context.set_string("empty", "");
        let result = engine.render_string("{{empty}}", &context).unwrap();
        assert_eq!(result, "");
        
        // String with no special characters
        context.set_string("safe", "Hello World 123!");
        let result = engine.render_string("{{safe}}", &context).unwrap();
        assert_eq!(result, "Hello World 123!");
        
        // Very long string with special characters
        let long_dangerous = "&<>\"'".repeat(1000);
        let expected_escaped = "&amp;&lt;&gt;&quot;&#x27;".repeat(1000);
        context.set_string("long", &long_dangerous);
        let result = engine.render_string("{{long}}", &context).unwrap();
        assert_eq!(result, expected_escaped);
    }

    #[test]
    fn test_mixed_safe_and_unsafe_content() {
        let mut engine = TemplateEngine::new("templates");
        let mut context = TemplateContext::new();
        
        let mixed_content = "Normal text <script>alert('xss')</script> more text & symbols";
        let expected = "Normal text &lt;script&gt;alert(&#x27;xss&#x27;)&lt;/script&gt; more text &amp; symbols";
        
        context.set_string("mixed", mixed_content);
        let result = engine.render_string("{{mixed}}", &context).unwrap();
        assert_eq!(result, expected);
    }
}

// Tests for TemplateValue functionality
mod value_tests {
    use super::*;
    
    #[test]
    fn test_template_value_creation() {
        let string_val = TemplateValue::String("test".to_string());
        let bool_val = TemplateValue::Bool(true);
        let number_val = TemplateValue::Number(42);
        let array_val = TemplateValue::Array(vec![]);
        let object_val = TemplateValue::Object(HashMap::new());
        
        // Test that all variants can be created
        match string_val {
            TemplateValue::String(s) => assert_eq!(s, "test"),
            _ => panic!("String value not created correctly"),
        }
        
        match bool_val {
            TemplateValue::Bool(b) => assert!(b),
            _ => panic!("Bool value not created correctly"),
        }
        
        match number_val {
            TemplateValue::Number(n) => assert_eq!(n, 42),
            _ => panic!("Number value not created correctly"),
        }
        
        match array_val {
            TemplateValue::Array(arr) => assert!(arr.is_empty()),
            _ => panic!("Array value not created correctly"),
        }
        
        match object_val {
            TemplateValue::Object(obj) => assert!(obj.is_empty()),
            _ => panic!("Object value not created correctly"),
        }
    }

    #[test]
    fn test_template_value_nested_structures() {
        // Create complex nested structure
        let mut inner_obj = HashMap::new();
        inner_obj.insert("nested_key".to_string(), TemplateValue::String("nested_value".to_string()));
        
        let complex_array = vec![
            TemplateValue::String("string_item".to_string()),
            TemplateValue::Number(123),
            TemplateValue::Bool(false),
            TemplateValue::Object(inner_obj),
        ];
        
        let mut outer_obj = HashMap::new();
        outer_obj.insert("array_prop".to_string(), TemplateValue::Array(complex_array));
        outer_obj.insert("simple_prop".to_string(), TemplateValue::String("simple".to_string()));
        
        let root_value = TemplateValue::Object(outer_obj);
        
        // Verify structure can be created and accessed
        match root_value {
            TemplateValue::Object(obj) => {
                assert!(obj.contains_key("array_prop"));
                assert!(obj.contains_key("simple_prop"));
                
                match obj.get("array_prop").unwrap() {
                    TemplateValue::Array(arr) => assert_eq!(arr.len(), 4),
                    _ => panic!("Array property not found"),
                }
            },
            _ => panic!("Root object not created correctly"),
        }
    }
}

// Tests for TemplateContext functionality
mod context_tests {
    use super::*;
    
    #[test]
    fn test_context_set_and_get_operations() {
        let mut context = TemplateContext::new();
        
        // Test string operations
        context.set_string("str_key", "string_value");
        assert_eq!(context.get_string("str_key"), Some("string_value".to_string()));
        
        // Test boolean operations
        context.set_bool("bool_true", true);
        context.set_bool("bool_false", false);
        assert_eq!(context.get_string("bool_true"), Some("true".to_string()));
        assert_eq!(context.get_string("bool_false"), Some("false".to_string()));
        
        // Test number operations
        context.set_number("num_positive", 42);
        context.set_number("num_negative", -17);
        context.set_number("num_zero", 0);
        assert_eq!(context.get_string("num_positive"), Some("42".to_string()));
        assert_eq!(context.get_string("num_negative"), Some("-17".to_string()));
        assert_eq!(context.get_string("num_zero"), Some("0".to_string()));
        
        // Test non-existent key
        assert_eq!(context.get_string("nonexistent"), None);
    }

    #[test]
    fn test_context_overwrite_values() {
        let mut context = TemplateContext::new();
        
        // Set initial value
        context.set_string("key", "initial");
        assert_eq!(context.get_string("key"), Some("initial".to_string()));
        
        // Overwrite with different type
        context.set_number("key", 123);
        assert_eq!(context.get_string("key"), Some("123".to_string()));
        
        // Overwrite with boolean
        context.set_bool("key", true);
        assert_eq!(context.get_string("key"), Some("true".to_string()));
    }

    #[test]
    fn test_context_complex_values() {
        let mut context = TemplateContext::new();
        
        // Set array value
        let array_val = TemplateValue::Array(vec![
            TemplateValue::String("item1".to_string()),
            TemplateValue::Number(42),
        ]);
        context.set("array_key", array_val);
        
        // Array should return empty string when accessed as string
        assert_eq!(context.get_string("array_key"), Some("".to_string()));
        
        // Set object value
        let mut obj = HashMap::new();
        obj.insert("prop".to_string(), TemplateValue::String("value".to_string()));
        context.set("obj_key", TemplateValue::Object(obj));
        
        // Object should return empty string when accessed as string
        assert_eq!(context.get_string("obj_key"), Some("".to_string()));
    }

    #[test]
    fn test_context_default_creation() {
        let context1 = TemplateContext::new();
        let context2 = TemplateContext::default();
        
        // Both should behave the same for non-existent keys
        assert_eq!(context1.get_string("any_key"), None);
        assert_eq!(context2.get_string("any_key"), None);
    }
}

// Tests for TemplateEngine state management
mod engine_tests {
    use super::*;
    use std::fs;
    
    #[test]
    fn test_engine_creation_with_different_paths() {
        // Test with current directory
        let _engine1 = TemplateEngine::new(".");
        // Should not panic
        
        // Test with templates directory
        let _engine2 = TemplateEngine::new("templates");
        // Should not panic
        
        // Test with non-existent directory
        let _engine3 = TemplateEngine::new("non_existent_dir_12345");
        // Should not panic during creation (only when trying to load templates)
    }

    #[test]
    fn test_engine_render_string_vs_render_file() {
        let temp_dir = tempfile::tempdir().unwrap();
        let template_path = temp_dir.path();
        
        // Create a test template file
        let template_content = "Hello {{name}}!";
        fs::write(template_path.join("test.html"), template_content).unwrap();
        
        let mut engine = TemplateEngine::new(template_path.to_str().unwrap());
        let mut context = TemplateContext::new();
        context.set_string("name", "World");
        
        // Test render_string
        let result1 = engine.render_string(template_content, &context).unwrap();
        
        // Test render from file
        let result2 = engine.render("test.html", &context).unwrap();
        
        // Both should produce the same result
        assert_eq!(result1, result2);
        assert_eq!(result1, "Hello World!");
    }

    #[test]
    fn test_engine_caching_behavior() {
        let temp_dir = tempfile::tempdir().unwrap();
        let template_path = temp_dir.path();
        
        // Create initial template
        fs::write(template_path.join("cached.html"), "Version 1: {{var}}").unwrap();
        
        let mut engine = TemplateEngine::new(template_path.to_str().unwrap());
        let mut context = TemplateContext::new();
        context.set_string("var", "test");
        
        // First load
        let result1 = engine.render("cached.html", &context).unwrap();
        assert_eq!(result1, "Version 1: test");
        
        // Modify the file (in real scenario, caching might prevent seeing changes)
        fs::write(template_path.join("cached.html"), "Version 2: {{var}}").unwrap();
        
        // Second load (behavior depends on caching implementation)
        let result2 = engine.render("cached.html", &context).unwrap();
        // The exact behavior depends on whether the engine implements cache invalidation
        // For now, we just verify it doesn't crash
        assert!(result2.contains("test"));
    }
}

// Performance and stress tests
mod performance_tests {
    use super::*;
    
    #[test]
    fn test_large_context_performance() {
        let mut engine = TemplateEngine::new("templates");
        let mut context = TemplateContext::new();
        
        // Add many variables to context
        for i in 0..1000 {
            context.set_string(&format!("var_{}", i), &format!("value_{}", i));
        }
        
        // Render template with some of these variables
        let template = "{{var_0}} {{var_500}} {{var_999}}";
        let result = engine.render_string(template, &context).unwrap();
        assert_eq!(result, "value_0 value_500 value_999");
    }

    #[test]
    fn test_deep_object_nesting_performance() {
        let mut engine = TemplateEngine::new("templates");
        let mut context = TemplateContext::new();
        
        // Current engine supports only two-level dot notation (object.property)
        let mut obj = HashMap::new();
        obj.insert("value".to_string(), TemplateValue::String("deep_value".to_string()));
        
        context.set("root", TemplateValue::Object(obj));
        
        // Access the nested value (engine supports object.property)
        let result = engine.render_string("{{root.value}}", &context).unwrap();
        assert_eq!(result, "deep_value");
    }

    #[test]
    fn test_many_loops_performance() {
        let mut engine = TemplateEngine::new("templates");
        let mut context = TemplateContext::new();
        
        // Create array with many items
        let items: Vec<TemplateValue> = (0..100)
            .map(|i| TemplateValue::Number(i))
            .collect();
        
        context.set("numbers", TemplateValue::Array(items));
        
        let template = "{{for num in numbers}}{{num}},{{/for}}";
        let result = engine.render_string(template, &context).unwrap();
        
        // Should contain all numbers
        assert!(result.contains("0,"));
        assert!(result.contains("50,"));
        assert!(result.contains("99,"));
        
        // Should have approximately 100 commas
        assert_eq!(result.matches(',').count(), 100);
    }
}