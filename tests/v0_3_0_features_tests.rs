use mystical_runic::{RuneEngine, RuneScroll, RuneSymbol};
use std::collections::HashMap;

// 🔴 RED PHASE - Writing failing tests first for v0.3.0 features

#[cfg(test)]
mod i18n_tests {
    use super::*;

    #[test]
    fn test_basic_translation() {
        let mut engine = RuneEngine::new("./templates");
        let context = RuneScroll::new();
        
        // Set up translations
        let mut translations = HashMap::new();
        translations.insert("welcome".to_string(), "Welcome".to_string());
        translations.insert("goodbye".to_string(), "Goodbye".to_string());
        engine.set_translations("en", translations);
        engine.set_locale("en");
        
        let template = "{{t \"welcome\"}} to our site!";
        let result = engine.render_string(template, &context);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Welcome to our site!");
    }

    #[test]
    fn test_translation_with_variables() {
        let mut engine = RuneEngine::new("./templates");
        let mut context = RuneScroll::new();
        context.set("name", RuneSymbol::String("Alice".to_string()));
        
        // Set up translations with variables
        let mut translations = HashMap::new();
        translations.insert("hello_user".to_string(), "Hello {{name}}!".to_string());
        engine.set_translations("en", translations);
        engine.set_locale("en");
        
        let template = "{{t \"hello_user\"}}";
        let result = engine.render_string(template, &context);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello Alice!");
    }

    #[test]
    fn test_missing_translation_fallback() {
        let mut engine = RuneEngine::new("./templates");
        let context = RuneScroll::new();
        
        let mut translations = HashMap::new();
        translations.insert("existing".to_string(), "Exists".to_string());
        engine.set_translations("en", translations);
        engine.set_locale("en");
        
        let template = "{{t \"missing_key\"}}";
        let result = engine.render_string(template, &context);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "missing_key"); // Fallback to key itself
    }

    #[test]
    fn test_locale_switching() {
        let mut engine = RuneEngine::new("./templates");
        let context = RuneScroll::new();
        
        // English translations
        let mut en_translations = HashMap::new();
        en_translations.insert("hello".to_string(), "Hello".to_string());
        engine.set_translations("en", en_translations);
        
        // French translations
        let mut fr_translations = HashMap::new();
        fr_translations.insert("hello".to_string(), "Bonjour".to_string());
        engine.set_translations("fr", fr_translations);
        
        let template = "{{t \"hello\"}}";
        
        // Test English
        engine.set_locale("en");
        let result_en = engine.render_string(template, &context);
        assert_eq!(result_en.unwrap(), "Hello");
        
        // Test French
        engine.set_locale("fr");
        let result_fr = engine.render_string(template, &context);
        assert_eq!(result_fr.unwrap(), "Bonjour");
    }
}

#[cfg(test)]
mod pluralization_tests {
    use super::*;

    #[test]
    fn test_simple_pluralization() {
        let mut engine = RuneEngine::new("./templates");
        let mut context = RuneScroll::new();
        context.set("count", RuneSymbol::Number(1));
        
        let template = "{{plural count \"item\" \"items\"}}";
        let result = engine.render_string(template, &context);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "item");
        
        // Test plural form
        context.set("count", RuneSymbol::Number(5));
        let result = engine.render_string(template, &context);
        assert_eq!(result.unwrap(), "items");
    }

    #[test]
    fn test_pluralization_with_count_display() {
        let mut engine = RuneEngine::new("./templates");
        let mut context = RuneScroll::new();
        context.set("count", RuneSymbol::Number(3));
        
        let template = "{{count}} {{plural count \"apple\" \"apples\"}}";
        let result = engine.render_string(template, &context);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "3 apples");
    }

    #[test]
    fn test_zero_pluralization() {
        let mut engine = RuneEngine::new("./templates");
        let mut context = RuneScroll::new();
        context.set("count", RuneSymbol::Number(0));
        
        let template = "{{plural count \"file\" \"files\"}}";
        let result = engine.render_string(template, &context);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "files"); // 0 uses plural form
    }
}

#[cfg(test)]
mod custom_filter_tests {
    use super::*;

    #[test]
    fn test_register_custom_filter() {
        let mut engine = RuneEngine::new("./templates");
        let mut context = RuneScroll::new();
        context.set("text", RuneSymbol::String("hello world".to_string()));
        
        // Register a custom reverse filter
        engine.register_filter("reverse", |input: &str, _args: &[&str]| {
            Ok(input.chars().rev().collect())
        });
        
        let template = "{{text|reverse}}";
        let result = engine.render_string(template, &context);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "dlrow olleh");
    }

    #[test]
    fn test_custom_filter_with_args() {
        let mut engine = RuneEngine::new("./templates");
        let mut context = RuneScroll::new();
        context.set("text", RuneSymbol::String("hello".to_string()));
        
        // Register a custom repeat filter
        engine.register_filter("repeat", |input: &str, args: &[&str]| {
            let times = args.get(0).map_or("1", |v| v).parse::<usize>().unwrap_or(1);
            Ok(input.repeat(times))
        });
        
        let template = "{{text|repeat:3}}";
        let result = engine.render_string(template, &context);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "hellohellohello");
    }

    #[test]
    fn test_custom_filter_chaining() {
        let mut engine = RuneEngine::new("./templates");
        let mut context = RuneScroll::new();
        context.set("text", RuneSymbol::String("hello".to_string()));
        
        // Register custom filters
        engine.register_filter("reverse", |input: &str, _args: &[&str]| {
            Ok(input.chars().rev().collect())
        });
        
        engine.register_filter("exclaim", |input: &str, _args: &[&str]| {
            Ok(format!("{}!", input))
        });
        
        let template = "{{text|reverse|upper|exclaim}}";
        let result = engine.render_string(template, &context);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "OLLEH!");
    }
}

#[cfg(test)]
mod math_filter_tests {
    use super::*;

    #[test]
    fn test_add_filter() {
        let mut engine = RuneEngine::new("./templates");
        let mut context = RuneScroll::new();
        context.set("price", RuneSymbol::Number(100));
        
        let template = "{{price|add:50}}";
        let result = engine.render_string(template, &context);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "150");
    }

    #[test]
    fn test_multiply_filter() {
        let mut engine = RuneEngine::new("./templates");
        let mut context = RuneScroll::new();
        context.set("price", RuneSymbol::Number(25));
        
        let template = "{{price|multiply:4}}";
        let result = engine.render_string(template, &context);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "100");
    }

    #[test]
    fn test_percentage_filter() {
        let mut engine = RuneEngine::new("./templates");
        let mut context = RuneScroll::new();
        context.set("value", RuneSymbol::Number(75));
        
        let template = "{{value|percentage}}";
        let result = engine.render_string(template, &context);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "75%");
    }

    #[test]
    fn test_round_filter() {
        let mut engine = RuneEngine::new("./templates");
        let mut context = RuneScroll::new();
        context.set("price", RuneSymbol::Number(1234)); // Representing 12.34
        
        let template = "{{price|divide:100|round:2}}";
        let result = engine.render_string(template, &context);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "12.34");
    }

    #[test]
    fn test_math_filter_chaining() {
        let mut engine = RuneEngine::new("./templates");
        let mut context = RuneScroll::new();
        context.set("base", RuneSymbol::Number(10));
        
        let template = "{{base|multiply:3|add:5|percentage}}";
        let result = engine.render_string(template, &context);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "35%");
    }
}