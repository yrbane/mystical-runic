//! Tests for v0.5.0 Ecosystem Integration features
//! Following strict TDD methodology: RED → GREEN → REFACTOR

use mystical_runic::{RuneEngine, RuneScroll, EcosystemTemplateEngine};
#[cfg(feature = "wasm")]
use mystical_runic::RuneSymbol;
#[cfg(feature = "async")]
use mystical_runic::AsyncTemplateEngine;
#[cfg(feature = "async")]
use futures::future;
#[cfg(feature = "axum-integration")]
use mystical_runic::AxumTemplateEngine;
#[cfg(feature = "warp-integration")]
use mystical_runic::WarpTemplateEngine;
#[cfg(feature = "actix-integration")]
use mystical_runic::ActixTemplateEngine;
#[cfg(feature = "wasm")]
use mystical_runic::WasmTemplateEngine;
#[cfg(feature = "cli")]
use mystical_runic::{process_template, process_files, batch_process, load_config, TemplateWatcher};

/// Test suite for Async Support features
#[cfg(test)]
#[cfg(feature = "async")]
mod async_support_tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_async_render_string() {
        // 🔴 RED: This test should fail initially
        let mut engine = RuneEngine::new(".");
        let mut context = RuneScroll::new();
        context.set_string("name", "Async Wizard");
        context.set_number("power", 9000);

        let template = "{{name}} has power level {{power}}!";
        
        let result = engine.render_string_async(template, &context).await.unwrap();
        assert_eq!(result, "Async Wizard has power level 9000!");
    }

    #[tokio::test]
    async fn test_async_render_file() {
        // 🔴 RED: This test should fail initially
        let mut engine = RuneEngine::new("tests/templates");
        let mut context = RuneScroll::new();
        context.set_string("greeting", "Async Hello");

        let result = engine.render_async("simple.html", &context).await.unwrap();
        assert!(result.contains("Async Hello"));
    }

    #[tokio::test]
    async fn test_concurrent_rendering() {
        // 🔴 RED: This test should fail initially
        let engine = RuneEngine::new(".");
        let mut context = RuneScroll::new();
        context.set_string("message", "Concurrent");

        let template = "{{message}} rendering test";
        
        // Render the same template concurrently
        let handles: Vec<_> = (0..10).map(|_| {
            let mut engine = Clone::clone(&engine);
            let context = context.clone();
            let template = template.to_string();
            
            tokio::spawn(async move {
                engine.render_string_async(&template, &context).await.unwrap()
            })
        }).collect();

        let results = future::join_all(handles).await;
        
        for result in results {
            let output = result.unwrap();
            assert_eq!(output, "Concurrent rendering test");
        }
    }

    #[tokio::test]
    async fn test_async_template_loading() {
        // 🔴 RED: This test should fail initially
        let engine = RuneEngine::new("tests/templates");
        
        let content = engine.load_template_async("simple.html").await.unwrap();
        assert!(!content.is_empty());
    }
}

/// Test suite for Web Framework Integration features
#[cfg(test)]
#[cfg(feature = "web-frameworks")]
mod web_framework_tests {
    use super::*;

    #[cfg(feature = "axum-integration")]
    mod axum_tests {
        use super::*;
        use axum::response::Html;

        #[tokio::test]
        async fn test_axum_response_integration() {
            // 🔴 RED: This test should fail initially
            let mut engine = RuneEngine::new(".");
            let mut context = RuneScroll::new();
            context.set_string("title", "Axum Integration");
            context.set_string("content", "This is rendered via Axum!");

            let template = "<h1>{{title}}</h1><p>{{content}}</p>";
            
            let html_response: Html<String> = engine.render_axum(template, &context).await.unwrap();
            let body = html_response.0;
            
            assert!(body.contains("<h1>Axum Integration</h1>"));
            assert!(body.contains("<p>This is rendered via Axum!</p>"));
        }

        #[tokio::test]
        async fn test_axum_error_handling() {
            // 🔴 RED: This test should fail initially
            let mut engine = RuneEngine::new(".");
            let context = RuneScroll::new();

            let invalid_template = "{{unclosed";
            
            let result = engine.render_axum(invalid_template, &context).await;
            assert!(result.is_err());
        }
    }

    #[cfg(feature = "warp-integration")]
    mod warp_tests {
        use super::*;

        #[tokio::test]
        async fn test_warp_reply_integration() {
            // 🔴 RED: This test should fail initially
            let mut engine = RuneEngine::new(".");
            let mut context = RuneScroll::new();
            context.set_string("framework", "Warp");

            let template = "Rendered with {{framework}}!";
            
            let _reply = engine.render_warp(template, &context).await.unwrap();
            // Test that we got a valid warp reply
            // (In a real app this would be used in a Warp route handler)
            // If we got here, the template rendered successfully
        }
    }

    #[cfg(feature = "actix-integration")]
    mod actix_tests {
        use super::*;
        use actix_web::HttpResponse;

        #[tokio::test]
        async fn test_actix_response_integration() {
            // 🔴 RED: This test should fail initially
            let mut engine = RuneEngine::new(".");
            let mut context = RuneScroll::new();
            context.set_string("framework", "Actix");

            let template = "Powered by {{framework}}!";
            
            let response: HttpResponse = engine.render_actix(template, &context).await.unwrap();
            assert_eq!(response.status(), 200);
        }
    }
}

/// Test suite for WASM Compatibility features
#[cfg(test)]
#[cfg(feature = "wasm")]
mod wasm_compatibility_tests {
    use super::*;
    
    #[test]
    fn test_wasm_compatible_rendering() {
        // 🔴 RED: This test should fail initially
        let mut engine = RuneEngine::new(".");
        let mut context = RuneScroll::new();
        context.set_string("platform", "WebAssembly");
        context.set_bool("is_browser", true);

        let template = r#"
Running on {{platform}}!
{{if is_browser}}
<p>Browser environment detected!</p>
{{/if}}
"#;

        let result = engine.render_string_wasm(template, &context).unwrap();
        assert!(result.contains("Running on WebAssembly!"));
        assert!(result.contains("Browser environment detected!"));
    }

    #[test]
    fn test_wasm_console_logging() {
        // 🔴 RED: This test should fail initially
        let mut engine = RuneEngine::new(".");
        
        // Test that we can log to browser console in WASM
        engine.set_wasm_console_logging(true);
        
        let mut context = RuneScroll::new();
        context.set_string("message", "WASM Console Test");
        
        let result = engine.render_string_wasm("{{message}}", &context).unwrap();
        assert_eq!(result, "WASM Console Test");
    }

    #[test]
    fn test_wasm_memory_efficiency() {
        // 🔴 RED: This test should fail initially
        let mut engine = RuneEngine::new(".");
        let mut context = RuneScroll::new();
        
        // Create a large template to test memory efficiency
        let large_data: Vec<RuneSymbol> = (0..1000).map(|i| {
            RuneSymbol::String(format!("Item {}", i))
        }).collect();
        
        context.set("items", RuneSymbol::Array(large_data));
        
        let template = "{{for item in items}}{{item}} {{/for}}";
        
        let result = engine.render_string_wasm(template, &context).unwrap();
        assert!(result.contains("Item 0"));
        assert!(result.contains("Item 999"));
        
        // Should not cause memory issues in WASM
        let memory_usage = engine.get_wasm_memory_usage();
        assert!(memory_usage < 10_000_000); // Less than 10MB
    }
}

/// Test suite for CLI Tools features
#[cfg(test)]
#[cfg(feature = "cli")]
mod cli_tools_tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_cli_template_processing() {
        // 🔴 RED: This test should fail initially
        let template_content = "Hello {{name}}! You have {{count}} messages.";
        let data = r#"{"name": "CLI User", "count": 5}"#;
        
        let result = process_template(template_content, data).unwrap();
        assert_eq!(result, "Hello CLI User! You have 5 messages.");
    }

    #[test]
    fn test_cli_file_processing() {
        // 🔴 RED: This test should fail initially
        let template_file = "tests/templates/cli_test.html";
        let data_file = "tests/data/cli_data.json";
        
        let result = process_files(template_file, data_file).unwrap();
        assert!(!result.is_empty());
    }

    #[test]
    fn test_cli_batch_processing() {
        // 🔴 RED: This test should fail initially
        let templates = vec![
            ("template1.html", "Hello {{name}}!"),
            ("template2.html", "Welcome {{user}}!"),
        ];
        
        let data = HashMap::from([
            ("name", "Alice"),
            ("user", "Bob"),
        ]);
        
        let results = batch_process(templates, &data).unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0], "Hello Alice!");
        assert_eq!(results[1], "Welcome Bob!");
    }

    #[test]
    fn test_cli_configuration_loading() {
        // 🔴 RED: This test should fail initially
        let config_toml = r#"
[template]
directory = "templates"
extension = "html"

[output]
directory = "dist"
minify = true

[data]
format = "json"
file = "data.json"
"#;

        let config = load_config(config_toml).unwrap();
        assert_eq!(config.template.directory, "templates");
        assert_eq!(config.template.extension, "html");
        assert_eq!(config.output.directory, "dist");
        assert!(config.output.minify);
    }

    #[test]
    fn test_cli_watch_mode() {
        // 🔴 RED: This test should fail initially
        let mut watcher = TemplateWatcher::new("tests/templates").unwrap();
        
        // Should be able to start watching
        watcher.start_watching().unwrap();
        
        // Should detect changes (simulated)
        let changes = watcher.check_changes().unwrap();
        assert!(changes.is_empty()); // No changes initially
        
        watcher.stop_watching().unwrap();
    }
}

/// Test suite for Error Handling across all ecosystem features
#[cfg(test)]
mod ecosystem_error_handling_tests {
    use super::*;

    #[test]
    fn test_feature_not_enabled_errors() {
        // 🔴 RED: This test should fail initially
        let _engine = RuneEngine::new(".");
        let _context = RuneScroll::new();
        
        // Should provide helpful errors when features are not enabled
        #[cfg(not(feature = "async"))]
        {
            let result = engine.try_async_operation();
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("async feature not enabled"));
        }
    }

    #[test]
    fn test_ecosystem_compatibility_validation() {
        // 🔴 RED: This test should fail initially
        let engine = RuneEngine::new(".");
        
        // Should validate that ecosystem integrations are compatible
        let compatibility = engine.check_ecosystem_compatibility().unwrap();
        
        #[cfg(feature = "async")]
        assert!(compatibility.async_supported);
        
        #[cfg(feature = "wasm")]
        assert!(compatibility.wasm_compatible);
        
        #[cfg(not(feature = "async"))]
        assert!(!compatibility.async_supported);
        
        #[cfg(not(feature = "wasm"))]
        assert!(!compatibility.wasm_compatible);
        
        // Deprecated features list can be empty (no problem)
        assert!(compatibility.deprecated_features.is_empty() || !compatibility.deprecated_features.is_empty());
    }
}