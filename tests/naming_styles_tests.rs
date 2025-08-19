// Test to demonstrate both naming styles work identically

#[cfg(test)]
mod professional_style_tests {
    use mystical_runic::{TemplateEngine, TemplateContext, TemplateValue};
    
    #[test]
    fn test_conventional_naming() {
        let mut engine = TemplateEngine::new("./templates");
        let mut context = TemplateContext::new();
        
        context.set("name", TemplateValue::String("Professional".to_string()));
        context.set("role", TemplateValue::String("Developer".to_string()));
        
        let template = "Hello {{name}}, you are a {{role}}!";
        let result = engine.render_string(template, &context).unwrap();
        
        assert_eq!(result, "Hello Professional, you are a Developer!");
    }

    #[test]
    fn test_conventional_with_filters() {
        let mut engine = TemplateEngine::new("./templates");
        let mut context = TemplateContext::new();
        
        context.set("text", TemplateValue::String("hello world".to_string()));
        context.set("price", TemplateValue::Number(100));
        
        // Test built-in filters
        let result1 = engine.render_string("{{text|upper}}", &context).unwrap();
        assert_eq!(result1, "HELLO WORLD");
        
        let result2 = engine.render_string("{{price|add:50}}", &context).unwrap();
        assert_eq!(result2, "150");
    }

    #[test]
    fn test_conventional_custom_filter() {
        let mut engine = TemplateEngine::new("./templates");
        let mut context = TemplateContext::new();
        
        context.set("text", TemplateValue::String("hello".to_string()));
        
        // Register custom filter using professional style
        engine.register_filter("reverse", |input: &str, _args: &[&str]| {
            Ok(input.chars().rev().collect())
        });
        
        let result = engine.render_string("{{text|reverse}}", &context).unwrap();
        assert_eq!(result, "olleh");
    }
}

#[cfg(test)]
mod mystical_style_tests {
    use mystical_runic::{RuneEngine, RuneScroll, RuneSymbol};
    
    #[test]
    fn test_mystical_naming() {
        let mut engine = RuneEngine::new("./sacred_scrolls");
        let mut scroll = RuneScroll::new();
        
        scroll.set("name", RuneSymbol::String("Mystical".to_string()));
        scroll.set("role", RuneSymbol::String("Sorcerer".to_string()));
        
        let template = "Greetings {{name}}, you are a {{role}}!";
        let result = engine.render_string(template, &scroll).unwrap();
        
        assert_eq!(result, "Greetings Mystical, you are a Sorcerer!");
    }

    #[test]
    fn test_mystical_with_filters() {
        let mut engine = RuneEngine::new("./sacred_scrolls");
        let mut scroll = RuneScroll::new();
        
        scroll.set("text", RuneSymbol::String("magic spell".to_string()));
        scroll.set("power", RuneSymbol::Number(42));
        
        // Test built-in filters with mystical style
        let result1 = engine.render_string("{{text|upper}}", &scroll).unwrap();
        assert_eq!(result1, "MAGIC SPELL");
        
        let result2 = engine.render_string("{{power|multiply:2}}", &scroll).unwrap();
        assert_eq!(result2, "84");
    }

    #[test]
    fn test_mystical_custom_filter() {
        let mut engine = RuneEngine::new("./sacred_scrolls");
        let mut scroll = RuneScroll::new();
        
        scroll.set("text", RuneSymbol::String("runes".to_string()));
        
        // Register custom filter using mystical style
        engine.register_filter("enchant", |input: &str, _args: &[&str]| {
            Ok(format!("🔮 {} 🔮", input))
        });
        
        let result = engine.render_string("{{text|enchant}}", &scroll).unwrap();
        assert_eq!(result, "🔮 runes 🔮");
    }
}

#[cfg(test)]
mod interoperability_tests {
    // Test that both naming styles can be mixed and are completely interchangeable
    use mystical_runic::{
        TemplateEngine, RuneEngine, 
        TemplateContext, RuneScroll,
        TemplateValue, RuneSymbol
    };
    
    #[test]
    fn test_mixed_naming_compatibility() {
        // Use conventional engine with mystical context
        let mut engine = TemplateEngine::new("./templates");
        let mut scroll = RuneScroll::new();
        
        scroll.set("name", RuneSymbol::String("Hybrid".to_string()));
        
        let result = engine.render_string("Hello {{name}}!", &scroll).unwrap();
        assert_eq!(result, "Hello Hybrid!");
    }

    #[test]
    fn test_complete_interchangeability() {
        // Show that both styles create identical objects
        let conventional_engine = TemplateEngine::new("./templates");
        let mystical_engine = RuneEngine::new("./templates");
        
        let conventional_context = TemplateContext::new();
        let mystical_scroll = RuneScroll::new();
        
        let conventional_value = TemplateValue::String("test".to_string());
        let mystical_symbol = RuneSymbol::String("test".to_string());
        
        // These are all the same types under the hood
        assert_eq!(std::mem::size_of_val(&conventional_engine), std::mem::size_of_val(&mystical_engine));
        assert_eq!(std::mem::size_of_val(&conventional_context), std::mem::size_of_val(&mystical_scroll));
        assert_eq!(std::mem::size_of_val(&conventional_value), std::mem::size_of_val(&mystical_symbol));
    }
}