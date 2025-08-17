use runic::{TemplateEngine, TemplateContext, TemplateValue};
use std::fs;

// Helper function to create test template files
fn setup_test_templates() -> tempfile::TempDir {
    let temp_dir = tempfile::tempdir().unwrap();
    let templates_path = temp_dir.path();
    
    // Create a simple test template
    fs::write(
        templates_path.join("test.html"),
        "<h1>{{title}}</h1><p>{{content}}</p>"
    ).unwrap();
    
    // Create a template with includes
    fs::write(
        templates_path.join("header.html"),
        "<header>{{app_name}}</header>"
    ).unwrap();
    
    fs::write(
        templates_path.join("main.html"),
        "{{include \"header.html\"}}<main>{{body}}</main>"
    ).unwrap();
    
    // Create a template with loops
    fs::write(
        templates_path.join("list.html"),
        "<ul>{{for item in items}}<li>{{item.name}}: {{item.value}}</li>{{/for}}</ul>"
    ).unwrap();
    
    temp_dir
}

#[test]
fn test_template_engine_creation() {
    let _engine = TemplateEngine::new("templates");
    // Engine created successfully (no public access to internal fields to test)
}

#[test]
fn test_template_context_creation() {
    let _context = TemplateContext::new();
    // Context created successfully
    
    let _default_context = TemplateContext::default();
    // Default context created successfully
}

#[test]
fn test_template_context_set_operations() {
    let mut context = TemplateContext::new();
    
    // Test string
    context.set_string("name", "Test");
    assert_eq!(context.get_string("name"), Some("Test".to_string()));
    
    // Test boolean
    context.set_bool("enabled", true);
    assert_eq!(context.get_string("enabled"), Some("true".to_string()));
    
    // Test number
    context.set_number("count", 42);
    assert_eq!(context.get_string("count"), Some("42".to_string()));
    
    // Test non-existent
    assert_eq!(context.get_string("nonexistent"), None);
}

#[test]
fn test_template_value_types() {
    let mut context = TemplateContext::new();
    
    // Test array
    let array = vec![
        TemplateValue::String("item1".to_string()),
        TemplateValue::String("item2".to_string()),
    ];
    context.set("items", TemplateValue::Array(array));
    
    // Test object
    let mut object = std::collections::HashMap::new();
    object.insert("key".to_string(), TemplateValue::String("value".to_string()));
    context.set("obj", TemplateValue::Object(object));
    
    // Test that we can create all value types
    let _string_val = TemplateValue::String("test".to_string());
    let _bool_val = TemplateValue::Bool(true);
    let _number_val = TemplateValue::Number(42);
    let _array_val = TemplateValue::Array(vec![]);
    let _object_val = TemplateValue::Object(std::collections::HashMap::new());
}

#[test]
fn test_variable_substitution() {
    let mut engine = TemplateEngine::new("templates");
    let mut context = TemplateContext::new();
    context.set_string("name", "World");
    
    let result = engine.render_string("Hello {{name}}!", &context).unwrap();
    assert_eq!(result, "Hello World!");
}

#[test]
fn test_multiple_variable_substitution() {
    let mut engine = TemplateEngine::new("templates");
    let mut context = TemplateContext::new();
    context.set_string("first", "Hello");
    context.set_string("second", "World");
    context.set_number("count", 3);
    
    let result = engine.render_string("{{first}} {{second}} {{count}} times!", &context).unwrap();
    assert_eq!(result, "Hello World 3 times!");
}

#[test]
fn test_html_escaping() {
    let mut engine = TemplateEngine::new("templates");
    let mut context = TemplateContext::new();
    context.set_string("html", "<script>alert('xss')</script>");
    
    let result = engine.render_string("Content: {{html}}", &context).unwrap();
    assert_eq!(result, "Content: &lt;script&gt;alert(&#x27;xss&#x27;)&lt;/script&gt;");
}

#[test]
fn test_raw_html_variables() {
    let mut engine = TemplateEngine::new("templates");
    let mut context = TemplateContext::new();
    context.set_string("html", "<strong>Bold</strong>");
    
    let result = engine.render_string("Content: {{& html}}", &context).unwrap();
    assert_eq!(result, "Content: <strong>Bold</strong>");
}

#[test]
fn test_conditionals_true() {
    let mut engine = TemplateEngine::new("templates");
    let mut context = TemplateContext::new();
    context.set_bool("show_message", true);
    
    let template = "{{if show_message}}Message visible{{/if}}";
    let result = engine.render_string(template, &context).unwrap();
    assert_eq!(result, "Message visible");
}

#[test]
fn test_conditionals_false() {
    let mut engine = TemplateEngine::new("templates");
    let mut context = TemplateContext::new();
    context.set_bool("show_message", false);
    
    let template = "{{if show_message}}Message visible{{/if}}";
    let result = engine.render_string(template, &context).unwrap();
    assert_eq!(result, "");
}

#[test]
fn test_conditionals_string_values() {
    let mut engine = TemplateEngine::new("templates");
    let mut context = TemplateContext::new();
    
    // Non-empty string should be true
    context.set_string("text", "content");
    let result = engine.render_string("{{if text}}Has content{{/if}}", &context).unwrap();
    assert_eq!(result, "Has content");
    
    // Empty string should be false
    context.set_string("text", "");
    let result = engine.render_string("{{if text}}Has content{{/if}}", &context).unwrap();
    assert_eq!(result, "");
}

#[test]
fn test_loops_with_strings() {
    let mut engine = TemplateEngine::new("templates");
    let mut context = TemplateContext::new();
    
    let items = vec![
        TemplateValue::String("First".to_string()),
        TemplateValue::String("Second".to_string()),
        TemplateValue::String("Third".to_string()),
    ];
    context.set("items", TemplateValue::Array(items));
    
    let template = "{{for item in items}}Item: {{item}} {{/for}}";
    let result = engine.render_string(template, &context).unwrap();
    assert_eq!(result, "Item: First Item: Second Item: Third ");
}

#[test]
fn test_loops_with_objects() {
    let mut engine = TemplateEngine::new("templates");
    let mut context = TemplateContext::new();
    
    // Create objects
    let mut obj1 = std::collections::HashMap::new();
    obj1.insert("name".to_string(), TemplateValue::String("Apple".to_string()));
    obj1.insert("price".to_string(), TemplateValue::Number(5));
    
    let mut obj2 = std::collections::HashMap::new();
    obj2.insert("name".to_string(), TemplateValue::String("Banana".to_string()));
    obj2.insert("price".to_string(), TemplateValue::Number(3));
    
    let items = vec![
        TemplateValue::Object(obj1),
        TemplateValue::Object(obj2),
    ];
    context.set("products", TemplateValue::Array(items));
    
    let template = "{{for product in products}}{{product.name}}: ${{product.price}} {{/for}}";
    let result = engine.render_string(template, &context).unwrap();
    assert_eq!(result, "Apple: $5 Banana: $3 ");
}

#[test]
fn test_comments() {
    let mut engine = TemplateEngine::new("templates");
    let context = TemplateContext::new();
    
    let template = "Before{{! This is a comment }}After";
    let result = engine.render_string(template, &context).unwrap();
    assert_eq!(result, "BeforeAfter");
}

#[test]
fn test_template_loading_and_caching() {
    let temp_dir = setup_test_templates();
    let mut engine = TemplateEngine::new(temp_dir.path().to_str().unwrap());
    
    // First load
    let template1 = engine.load_template("test.html").unwrap();
    assert_eq!(template1, "<h1>{{title}}</h1><p>{{content}}</p>");
    
    // Second load should use cache
    let template2 = engine.load_template("test.html").unwrap();
    assert_eq!(template1, template2);
}

#[test]
fn test_template_rendering_from_file() {
    let temp_dir = setup_test_templates();
    let mut engine = TemplateEngine::new(temp_dir.path().to_str().unwrap());
    let mut context = TemplateContext::new();
    context.set_string("title", "Test Title");
    context.set_string("content", "Test Content");
    
    let result = engine.render("test.html", &context).unwrap();
    assert_eq!(result, "<h1>Test Title</h1><p>Test Content</p>");
}

#[test]
fn test_includes() {
    let temp_dir = setup_test_templates();
    let mut engine = TemplateEngine::new(temp_dir.path().to_str().unwrap());
    let mut context = TemplateContext::new();
    context.set_string("app_name", "My App");
    context.set_string("body", "Welcome!");
    
    let result = engine.render("main.html", &context).unwrap();
    assert_eq!(result, "<header>My App</header><main>Welcome!</main>");
}

#[test]
fn test_complex_template() {
    let mut engine = TemplateEngine::new("templates");
    let mut context = TemplateContext::new();
    
    context.set_string("title", "Shopping Cart");
    context.set_bool("has_items", true);
    
    let mut item1 = std::collections::HashMap::new();
    item1.insert("name".to_string(), TemplateValue::String("Apple".to_string()));
    item1.insert("price".to_string(), TemplateValue::Number(5));
    
    let mut item2 = std::collections::HashMap::new();
    item2.insert("name".to_string(), TemplateValue::String("Banana".to_string()));
    item2.insert("price".to_string(), TemplateValue::Number(3));
    
    let items = vec![
        TemplateValue::Object(item1),
        TemplateValue::Object(item2),
    ];
    context.set("cart_items", TemplateValue::Array(items));
    
    let template = r#"
<h1>{{title}}</h1>
{{if has_items}}
<ul>
{{for item in cart_items}}
  <li>{{item.name}} - ${{item.price}}</li>
{{/for}}
</ul>
{{/if}}
"#;
    
    let result = engine.render_string(template, &context).unwrap();
    let expected = r#"
<h1>Shopping Cart</h1>

<ul>

  <li>Apple - $5</li>

  <li>Banana - $3</li>

</ul>

"#;
    assert_eq!(result, expected);
}

// Error cases

#[test]
fn test_nonexistent_template_file() {
    let mut engine = TemplateEngine::new("nonexistent_dir");
    let context = TemplateContext::new();
    
    let result = engine.render("nonexistent.html", &context);
    assert!(result.is_err());
}

#[test]
fn test_unclosed_variable() {
    let mut engine = TemplateEngine::new("templates");
    let context = TemplateContext::new();
    
    let result = engine.render_string("Hello {{name", &context);
    assert!(result.is_err());
}

#[test]
fn test_unclosed_conditional() {
    let mut engine = TemplateEngine::new("templates");
    let context = TemplateContext::new();
    
    let result = engine.render_string("{{if condition}}content", &context);
    assert!(result.is_err());
}

#[test]
fn test_missing_variable_returns_empty() {
    let mut engine = TemplateEngine::new("templates");
    let context = TemplateContext::new();
    
    let result = engine.render_string("Hello {{nonexistent}}!", &context).unwrap();
    assert_eq!(result, "Hello !");
}