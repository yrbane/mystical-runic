//! Tests for v0.4.1 IDE Integration features
//! Following strict TDD methodology: RED → GREEN → REFACTOR

use mystical_runic::{RuneEngine, RuneScroll};

/// Test suite for Language Server Protocol features
#[cfg(test)]
mod language_server_protocol_tests {
    use super::*;

    #[test]
    fn test_lsp_parse_document() {
        // 🔴 RED: This test should fail initially
        let mut engine = RuneEngine::new(".");
        
        let template_content = r#"
{{name}}
{{if condition}}
    {{for item in items}}
        {{item.name}}
    {{/for}}
{{/if}}
"#;

        let lsp_result = engine.parse_for_lsp(template_content, "test.html").unwrap();
        
        // Should identify all template tokens
        assert_eq!(lsp_result.variables.len(), 4); // name, condition, items, item.name
        assert!(lsp_result.variables.contains(&"name".to_string()));
        assert!(lsp_result.variables.contains(&"condition".to_string()));
        assert!(lsp_result.variables.contains(&"items".to_string()));
        assert!(lsp_result.variables.contains(&"item.name".to_string()));
        
        // Should identify template blocks
        assert_eq!(lsp_result.blocks.len(), 2); // if block, for block
        assert_eq!(lsp_result.blocks[0].block_type, "if");
        assert_eq!(lsp_result.blocks[0].start_line, 3);
        assert_eq!(lsp_result.blocks[1].block_type, "for");
        assert_eq!(lsp_result.blocks[1].start_line, 4);
    }

    #[test]
    fn test_lsp_get_completions() {
        // 🔴 RED: This test should fail initially
        let mut engine = RuneEngine::new(".");
        let mut context = RuneScroll::new();
        context.set_string("user_name", "Alice");
        context.set_string("user_email", "alice@example.com");
        context.set_number("user_level", 42);
        
        let template = "{{user_}}"; // Incomplete variable name
        let position = 7; // Position after "user_"
        
        let completions = engine.get_completions_at_position(template, position, &context).unwrap();
        
        assert!(!completions.is_empty());
        assert!(completions.iter().any(|c| c.label == "user_name"));
        assert!(completions.iter().any(|c| c.label == "user_email"));
        assert!(completions.iter().any(|c| c.label == "user_level"));
        
        // Should include completion details
        let user_name_completion = completions.iter().find(|c| c.label == "user_name").unwrap();
        assert_eq!(user_name_completion.completion_type, "variable");
        assert_eq!(user_name_completion.detail, "String: Alice");
    }

    #[test]
    fn test_lsp_get_filter_completions() {
        // 🔴 RED: This test should fail initially  
        let mut engine = RuneEngine::new(".");
        let context = RuneScroll::new();
        
        let template = "{{name|up}}"; // Incomplete filter name
        let position = 9; // Position after "up"
        
        let completions = engine.get_completions_at_position(template, position, &context).unwrap();
        
        assert!(!completions.is_empty());
        assert!(completions.iter().any(|c| c.label == "upper"));
        
        let upper_completion = completions.iter().find(|c| c.label == "upper").unwrap();
        assert_eq!(upper_completion.completion_type, "filter");
        assert!(upper_completion.detail.contains("Convert text to uppercase"));
    }
}

/// Test suite for Syntax Highlighting features
#[cfg(test)]
mod syntax_highlighting_tests {
    use super::*;

    #[test]
    fn test_tokenize_template_for_highlighting() {
        // 🔴 RED: This test should fail initially
        let mut engine = RuneEngine::new(".");
        
        let template = r#"<h1>{{title|upper}}</h1>
{{if show_content}}
    <p>{{description}}</p>
{{/if}}"#;

        let tokens = engine.tokenize_for_syntax_highlighting(template).unwrap();
        
        // Should identify different token types
        assert!(tokens.iter().any(|t| t.token_type == "html_tag"));
        assert!(tokens.iter().any(|t| t.token_type == "template_variable"));
        assert!(tokens.iter().any(|t| t.token_type == "template_filter"));
        assert!(tokens.iter().any(|t| t.token_type == "template_directive"));
        
        // Check specific tokens
        let title_token = tokens.iter().find(|t| t.content == "title").unwrap();
        assert_eq!(title_token.token_type, "template_variable");
        assert_eq!(title_token.start_position, 6);
        
        let upper_token = tokens.iter().find(|t| t.content == "upper").unwrap();
        assert_eq!(upper_token.token_type, "template_filter");
    }

    #[test]
    fn test_get_syntax_theme_info() {
        // 🔴 RED: This test should fail initially
        let engine = RuneEngine::new(".");
        
        let theme_info = engine.get_syntax_theme_info().unwrap();
        
        // Should provide color information for different token types
        assert!(theme_info.contains_key("template_variable"));
        assert!(theme_info.contains_key("template_filter"));
        assert!(theme_info.contains_key("template_directive"));
        assert!(theme_info.contains_key("html_tag"));
        
        // Should have semantic color information
        let variable_color = &theme_info["template_variable"];
        assert!(variable_color.contains("#") || variable_color.starts_with("rgb"));
    }
}

/// Test suite for Auto-completion features
#[cfg(test)]
mod auto_completion_tests {
    use super::*;

    #[test]
    fn test_complete_nested_object_properties() {
        // 🔴 RED: This test should fail initially
        let mut engine = RuneEngine::new(".");
        let mut context = RuneScroll::new();
        
        // Set up nested object structure
        context.set_nested_object("user", vec![
            ("profile", vec![
                ("name", "Alice")
            ]),
            ("settings", vec![
                ("theme", "dark"),
                ("language", "en")
            ])
        ]);
        
        let template = "{{user}}"; // Simple completion test
        let position = 6; // Position after "user"
        
        let completions = engine.get_completions_at_position(template, position, &context).unwrap();
        
        // Should complete the user variable
        assert!(completions.iter().any(|c| c.label == "user"));
        
        let user_completion = completions.iter().find(|c| c.label == "user").unwrap();
        assert_eq!(user_completion.completion_type, "variable");
        assert!(user_completion.detail.contains("Object"));
    }

    #[test]
    fn test_complete_template_directives() {
        // 🔴 RED: This test should fail initially
        let mut engine = RuneEngine::new(".");
        let context = RuneScroll::new();
        
        let template = "{{i}}"; // Should suggest "if"
        let position = 3; // Position after "i"
        
        let completions = engine.get_completions_at_position(template, position, &context).unwrap();
        
        assert!(completions.iter().any(|c| c.label == "if"));
        assert!(completions.iter().any(|c| c.label == "include"));
        
        let if_completion = completions.iter().find(|c| c.label == "if").unwrap();
        assert_eq!(if_completion.completion_type, "directive");
        assert!(if_completion.detail.contains("Conditional"));
    }
}

/// Test suite for Error Squiggles (real-time error highlighting)
#[cfg(test)]
mod error_squiggles_tests {
    use super::*;

    #[test]
    fn test_get_real_time_diagnostics() {
        // 🔴 RED: This test should fail initially
        let mut engine = RuneEngine::new(".");
        let context = RuneScroll::new();
        
        let template = r#"
{{if unclosed_condition}}
{{missing_variable}}
{{unknown_filter|nonexistent}}
"#;

        let diagnostics = engine.get_diagnostics_for_editor(template, &context).unwrap();
        
        
        assert_eq!(diagnostics.len(), 4);
        
        // Should detect unclosed if
        let unclosed_error = diagnostics.iter().find(|d| d.message.contains("Unclosed")).unwrap();
        assert_eq!(unclosed_error.severity, "error");
        assert_eq!(unclosed_error.line, 2);
        
        // Should detect unknown variable  
        let unknown_var_error = diagnostics.iter().find(|d| d.message.contains("missing_variable")).unwrap();
        assert_eq!(unknown_var_error.severity, "warning");
        assert_eq!(unknown_var_error.line, 3);
        
        // Should detect unknown filter
        let unknown_filter_error = diagnostics.iter().find(|d| d.message.contains("nonexistent")).unwrap();
        assert_eq!(unknown_filter_error.severity, "error");
        assert_eq!(unknown_filter_error.line, 4);
    }

    #[test]
    fn test_get_hover_information() {
        // 🔴 RED: This test should fail initially
        let mut engine = RuneEngine::new(".");
        let mut context = RuneScroll::new();
        context.set_string("user_name", "Alice");
        context.set_number("user_level", 42);
        
        let template = "{{user_name}} is level {{user_level}}";
        let position = 4; // Position over "user_name"
        
        
        let hover_info = engine.get_hover_info_at_position(template, position, &context).unwrap();
        
        assert_eq!(hover_info.variable_name, "user_name");
        assert_eq!(hover_info.variable_type, "String");
        assert_eq!(hover_info.current_value, "Alice");
        assert!(hover_info.description.contains("Template variable"));
    }

    #[test]
    fn test_get_definition_location() {
        // 🔴 RED: This test should fail initially
        let mut engine = RuneEngine::new(".");
        
        let template = r#"
{{macro user_card(name)}}
<div>{{name}}</div>
{{/macro}}

{{user_card("Alice")}}
"#;

        let position = 65; // Position over "user_card" in the call
        
        let definition = engine.get_definition_at_position(template, position).unwrap();
        
        assert_eq!(definition.definition_type, "macro");
        assert_eq!(definition.name, "user_card");
        assert_eq!(definition.line, 2);
        assert_eq!(definition.column, 9);
    }
}