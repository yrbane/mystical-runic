// Advanced Security Tests for mystical-runic
// These tests ensure the template engine is secure against various attack vectors

use mystical_runic::*;
use std::collections::HashMap;
use std::fs;

#[test]
fn test_xss_prevention_comprehensive() {
    let mut engine = TemplateEngine::new("templates");
    let mut context = TemplateContext::new();
    
    // Comprehensive XSS attack vectors
    let xss_payloads = vec![
        "<script>alert('XSS')</script>",
        "<img src='x' onerror='alert(1)'>",
        "<svg onload='alert(1)'>",
        "<iframe src='javascript:alert(1)'></iframe>",
        "<body onload='alert(1)'>",
        "<div onclick='alert(1)'>Click me</div>",
        "<input onfocus='alert(1)' autofocus>",
        "<marquee onstart='alert(1)'>XSS</marquee>",
        "<video><source onerror='alert(1)'>",
        "<audio src='x' onerror='alert(1)'>",
        "javascript:alert('XSS')",
        "data:text/html,<script>alert('XSS')</script>",
        "<object data='javascript:alert(1)'>",
        "<embed src='javascript:alert(1)'>",
        "<meta http-equiv='refresh' content='0;url=javascript:alert(1)'>",
        "<link rel='stylesheet' href='javascript:alert(1)'>",
        "<style>@import 'javascript:alert(1)';</style>",
        "<!--<script>alert('XSS')</script>-->",
        "<![CDATA[<script>alert('XSS')</script>]]>",
        "\"><script>alert('XSS')</script>",
        "'><script>alert('XSS')</script>",
        "<script>eval('alert(1)')</script>",
        "<script>setTimeout('alert(1)',1)</script>",
        "<script>Function('alert(1)')()</script>",
    ];
    
    for payload in xss_payloads {
        context.set_string("xss", payload);
        let result = engine.render_string("{{xss}}", &context).unwrap();
        
        // Ensure dangerous patterns are escaped
        assert!(!result.contains("<script"), "XSS payload not escaped: {}", payload);
        assert!(!result.contains("javascript:"), "JavaScript URL not escaped: {}", payload);
        assert!(!result.contains("onerror="), "Event handler not escaped: {}", payload);
        assert!(!result.contains("onload="), "Event handler not escaped: {}", payload);
        assert!(!result.contains("onclick="), "Event handler not escaped: {}", payload);
        assert!(!result.contains("onfocus="), "Event handler not escaped: {}", payload);
        assert!(!result.contains("onstart="), "Event handler not escaped: {}", payload);
        
        // Ensure proper escaping characters are present
        if payload.contains("<") {
            assert!(result.contains("&lt;"), "< not properly escaped in: {}", payload);
        }
        if payload.contains(">") {
            assert!(result.contains("&gt;"), "> not properly escaped in: {}", payload);
        }
        if payload.contains("\"") {
            assert!(result.contains("&quot;"), "Quote not properly escaped in: {}", payload);
        }
        if payload.contains("'") {
            assert!(result.contains("&#x27;"), "Apostrophe not properly escaped in: {}", payload);
        }
    }
}

#[test]
fn test_injection_attacks_prevention() {
    let mut engine = TemplateEngine::new("templates");
    let mut context = TemplateContext::new();
    
    // SQL injection attempts (should be harmless in templates but test escaping)
    let sql_payloads = vec![
        "'; DROP TABLE users; --",
        "1' OR '1'='1",
        "admin'/*",
        "'; INSERT INTO users VALUES('hacker', 'password'); --",
        "1' UNION SELECT * FROM passwords--",
    ];
    
    for payload in sql_payloads {
        context.set_string("input", payload);
        let result = engine.render_string("User input: {{input}}", &context).unwrap();
        
        // Ensure single quotes are escaped
        assert!(result.contains("&#x27;"), "Single quotes not escaped in SQL payload: {}", payload);
        assert!(!result.contains("DROP TABLE"), "Dangerous SQL not escaped: {}", payload);
        assert!(!result.contains("UNION SELECT"), "SQL injection not escaped: {}", payload);
    }
}

#[test] 
fn test_path_traversal_prevention() {
    let temp_dir = tempfile::tempdir().unwrap();
    let templates_path = temp_dir.path();
    
    // Create a template in a subdirectory
    let sub_dir = templates_path.join("safe");
    fs::create_dir(&sub_dir).unwrap();
    fs::write(sub_dir.join("template.html"), "Safe content").unwrap();
    
    // Create a file outside the templates directory
    let outside_file = temp_dir.path().parent().unwrap().join("secret.txt");
    fs::write(&outside_file, "Secret content").unwrap();
    
    let mut engine = TemplateEngine::new(templates_path.to_str().unwrap());
    let context = TemplateContext::new();
    
    // Test path traversal attempts
    let traversal_attempts = vec![
        "../secret.txt",
        "../../secret.txt",
        "../../../etc/passwd",
        "..\\secret.txt",
        "..\\..\\secret.txt",
        "./../secret.txt",
        "safe/../secret.txt",
        "/etc/passwd",
        "C:\\Windows\\System32\\config\\sam",
        "safe/../../secret.txt",
    ];
    
    for attempt in traversal_attempts {
        let result = engine.render(&attempt, &context);
        // Should fail to load files outside the templates directory
        assert!(result.is_err(), "Path traversal should fail: {}", attempt);
    }
    
    // Verify legitimate file still works
    let result = engine.render("safe/template.html", &context).unwrap();
    assert_eq!(result, "Safe content");
    
    // Clean up
    fs::remove_file(&outside_file).unwrap_or(());
}

#[test]
fn test_template_injection_prevention() {
    let mut engine = TemplateEngine::new("templates");
    let mut context = TemplateContext::new();
    
    // Attempt to inject template syntax through variables
    let injection_attempts = vec![
        "{{malicious_var}}",
        "{{& dangerous_raw}}",
        "{{if always_true}}injected{{/if}}",
        "{{for item in fake_array}}hack{{/for}}",
        "{{include \"../../../etc/passwd\"}}",
        "{{! comment injection }}",
        "}} injected {{",
        "{{}} malformed {{",
        "{{variable}} {{& raw}} {{if condition}}",
    ];
    
    for injection in injection_attempts {
        context.set_string("user_input", injection);
        let result = engine.render_string("User said: {{user_input}}", &context).unwrap();
        
        // Template syntax should be escaped, not executed
        assert!(result.contains("User said:"), "Template structure modified by injection: {}", injection);
        
        // Check that template syntax is escaped
        if injection.contains("{{") {
            assert!(result.contains("{{") || result.contains("&lt;") || result.contains("&#x7B;"), 
                   "Template braces not properly handled: {}", injection);
        }
        
        // Should not contain unescaped dangerous patterns
        assert!(!result.contains("</script>"), "Script tags not escaped: {}", injection);
        assert!(!result.contains("onerror="), "Event handlers not escaped: {}", injection);
    }
}

#[test]
fn test_dos_prevention_large_inputs() {
    let mut engine = TemplateEngine::new("templates");
    let mut context = TemplateContext::new();
    
    // Test with extremely large strings (potential DoS)
    let large_string = "A".repeat(100_000);
    context.set_string("large", &large_string);
    
    let result = engine.render_string("{{large}}", &context).unwrap();
    assert_eq!(result.len(), 100_000);
    assert_eq!(result, large_string);
    
    // Test with many small variables (potential memory DoS)
    for i in 0..10_000 {
        context.set_string(&format!("var_{}", i), "value");
    }
    
    let result = engine.render_string("{{var_0}} {{var_5000}} {{var_9999}}", &context).unwrap();
    assert_eq!(result, "value value value");
}

#[test]
fn test_unicode_security_issues() {
    let mut engine = TemplateEngine::new("templates");
    let mut context = TemplateContext::new();
    
    // Unicode characters that might bypass security filters
    let unicode_attacks = vec![
        "\\u003cscript\\u003ealert(1)\\u003c/script\\u003e", // Encoded <script>
        "<script>alert(1)</script>", // Basic script
        "\\u0000<script>alert(1)</script>", // Null byte
        "<\\u0000script>alert(1)</script>", // Null byte in tag
    ];
    
    for attack in unicode_attacks {
        context.set_string("unicode_attack", attack);
        let result = engine.render_string("{{unicode_attack}}", &context).unwrap();
        
        // Should not contain unescaped script tags regardless of encoding
        assert!(!result.contains("<script"), "Unicode bypass detected: {}", attack);
        assert!(!result.contains("alert("), "JavaScript execution possible: {}", attack);
        
        // Should properly escape dangerous characters
        if attack.contains("<") {
            assert!(result.contains("&lt;") || result.contains("\\\\u"), 
                   "< character not properly escaped: {}", attack);
        }
    }
}

#[test]
fn test_information_disclosure_prevention() {
    let temp_dir = tempfile::tempdir().unwrap();
    let templates_path = temp_dir.path();
    
    // Create some files that shouldn't be accessible
    fs::write(templates_path.join(".env"), "SECRET_KEY=super_secret").unwrap();
    fs::write(templates_path.join("config.json"), r#"{"password": "secret123"}"#).unwrap();
    fs::write(templates_path.join("database.db"), "binary database content").unwrap();
    
    let mut engine = TemplateEngine::new(templates_path.to_str().unwrap());
    let context = TemplateContext::new();
    
    // Attempt to access sensitive files
    let sensitive_files = vec![
        ".env",
        "config.json", 
        "database.db",
        ".git/config",
        ".ssh/id_rsa",
        "backup.sql",
    ];
    
    for file in sensitive_files {
        let result = engine.render(file, &context);
        // Most of these should fail, but if they succeed, content should be treated as template
        match result {
            Ok(content) => {
                // Even if file is loaded, shouldn't contain raw sensitive data
                assert!(!content.contains("SECRET_KEY"), "Secret key exposed from file: {}", file);
                assert!(!content.contains("password"), "Password exposed from file: {}", file);
            },
            Err(_) => {
                // Expected behavior - should not be able to access these files
            }
        }
    }
}

#[test]
fn test_memory_exhaustion_prevention() {
    let mut engine = TemplateEngine::new("templates");
    let mut context = TemplateContext::new();
    
    // Test deeply nested object structure (potential memory bomb)
    let mut deeply_nested = TemplateValue::String("deep".to_string());
    
    // Create 100 levels of nesting
    for i in 0..100 {
        let mut obj = HashMap::new();
        obj.insert(format!("level_{}", i), deeply_nested);
        deeply_nested = TemplateValue::Object(obj);
    }
    
    context.set("nested", deeply_nested);
    
    // This should not crash or consume excessive memory
    let result = engine.render_string("{{nested}}", &context).unwrap();
    assert_eq!(result, ""); // Object should render as empty string
    
    // Test extremely wide object (many properties)
    let mut wide_obj = HashMap::new();
    for i in 0..10_000 {
        wide_obj.insert(format!("prop_{}", i), TemplateValue::String(format!("value_{}", i)));
    }
    context.set("wide", TemplateValue::Object(wide_obj));
    
    let result = engine.render_string("{{wide}}", &context).unwrap();
    assert_eq!(result, ""); // Object should render as empty string
}

#[test]
fn test_context_pollution_prevention() {
    let mut engine = TemplateEngine::new("templates");
    let mut context = TemplateContext::new();
    
    // Set up initial safe context
    context.set_string("safe_var", "safe_value");
    context.set_bool("is_admin", false);
    
    // Attempt to pollute context through template processing
    let template = "{{safe_var}} normal content";
    let result = engine.render_string(template, &context).unwrap();
    assert_eq!(result, "safe_value normal content");
    
    // Verify context hasn't been modified
    assert_eq!(context.get_string("safe_var"), Some("safe_value".to_string()));
    assert_eq!(context.get_string("is_admin"), Some("false".to_string()));
    
    // Test that variables don't leak between renders
    let mut context2 = TemplateContext::new();
    context2.set_string("secret", "should_not_leak");
    
    let _ = engine.render_string("{{secret}}", &context2).unwrap();
    
    // Original context should not have the secret
    assert_eq!(context.get_string("secret"), None);
}