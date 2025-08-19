//! # 💻 IDE Integration Showcase - v0.4.1
//!
//! This example demonstrates ALL the new IDE integration features in Mystical-Runic v0.4.1:
//! - Language Server Protocol (LSP) support
//! - Syntax highlighting with semantic tokens
//! - Auto-completion for variables, filters, and directives  
//! - Real-time error diagnostics with squiggles
//! - Hover information with type and value inspection
//! - Go-to-definition for macros and templates
//!
//! Run this example to see the IDE features in action!

use mystical_runic::{RuneEngine, RuneScroll, RuneSymbol};
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔮 MYSTICAL-RUNIC v0.4.1 - IDE Integration Showcase!");
    println!("═══════════════════════════════════════════════════════\n");
    
    let mut engine = RuneEngine::new(".");
    let context = create_sample_context();
    
    // Sample template with various features for IDE integration
    let template = r#"
{{! Welcome template with macro and various features }}
{{macro user_card(user, show_details)}}
<div class="user-card">
    <h3>{{user.name|upper}}</h3>
    <p>Email: {{user.email}}</p>
    {{if show_details}}
        <p>Level: {{user.stats.level}}</p>
        <p>Score: {{user.stats.score|currency}}</p>
    {{/if}}
</div>
{{/macro}}

<h1>{{site_title|upper}}</h1>

{{for user in users}}
    {{user_card(user, true)}}
{{/for}}

{{if total_users}}
    <p>Total: {{total_users}} {{plural total_users "user" "users"}}</p>
{{/if}}
"#;

    println!("📝 TEMPLATE CONTENT:");
    println!("{}", template);
    println!();

    // 🔍 1. Language Server Protocol - Template Analysis
    println!("🔍 1. LANGUAGE SERVER PROTOCOL - Template Analysis");
    println!("─────────────────────────────────────────────────────");
    
    let lsp_result = engine.parse_for_lsp(template, "showcase.html")?;
    
    println!("📊 Analysis Results:");
    println!("   Variables found: {:?}", lsp_result.variables);
    println!("   Template blocks: {} ({})", 
        lsp_result.blocks.len(),
        lsp_result.blocks.iter().map(|b| b.block_type.as_str()).collect::<Vec<_>>().join(", ")
    );
    println!("   Filters used: {:?}", lsp_result.filters);
    println!("   Macros defined: {:?}", lsp_result.macros);
    println!();
    
    // 💡 2. Auto-completion Features
    println!("💡 2. AUTO-COMPLETION - Intelligent Suggestions");
    println!("──────────────────────────────────────────────────");
    
    // Test variable completion
    let test_partial = "{{user_n}}"; // Incomplete variable
    let position = 7; // After "user_n"
    
    println!("🔤 Testing completion for '{}' at position {}", test_partial, position);
    
    let completions = engine.get_completions_at_position(test_partial, position, &context)?;
    println!("   Suggestions found: {}", completions.len());
    for completion in &completions {
        println!("     • {} ({}) - {}", 
            completion.label, 
            completion.completion_type,
            completion.detail
        );
    }
    println!();
    
    // Test filter completion
    let test_filter = "{{name|up}}"; // Incomplete filter
    let filter_pos = 8; // After "up"
    
    println!("🎨 Testing filter completion for '{}' at position {}", test_filter, filter_pos);
    let filter_completions = engine.get_completions_at_position(test_filter, filter_pos, &context)?;
    for completion in &filter_completions {
        println!("     • {} ({}) - {}", 
            completion.label, 
            completion.completion_type,
            completion.detail
        );
    }
    println!();
    
    // 🎨 3. Syntax Highlighting - Token Analysis
    println!("🎨 3. SYNTAX HIGHLIGHTING - Semantic Token Analysis");
    println!("────────────────────────────────────────────────────");
    
    let tokens = engine.tokenize_for_syntax_highlighting(template)?;
    println!("📝 Found {} syntax tokens", tokens.len());
    
    // Show first 10 tokens as examples
    println!("   First 10 tokens:");
    for (i, token) in tokens.iter().take(10).enumerate() {
        println!("     {}. '{}' -> {} (pos: {})", 
            i + 1, 
            token.content.replace('\n', "\\n"),
            token.token_type,
            token.start_position
        );
    }
    println!();
    
    // Get theme information
    let theme_info = engine.get_syntax_theme_info()?;
    println!("🌈 Available syntax theme colors:");
    for (token_type, color) in &theme_info {
        println!("     • {}: {}", token_type, color);
    }
    println!();
    
    // 🚨 4. Real-time Diagnostics - Error Detection
    println!("🚨 4. REAL-TIME DIAGNOSTICS - Error Detection");
    println!("──────────────────────────────────────────────────");
    
    // Test with template containing errors
    let invalid_template = r#"
{{title}}
{{if unclosed_condition}}
{{missing_variable}}
{{name|nonexistent_filter}}
{{unknown_macro()}}
"#;
    
    println!("🔍 Analyzing template with intentional errors:");
    let diagnostics = engine.get_diagnostics_for_editor(invalid_template, &context)?;
    println!("   Found {} diagnostic issues:", diagnostics.len());
    
    for diagnostic in &diagnostics {
        let severity_icon = match diagnostic.severity.as_str() {
            "error" => "❌",
            "warning" => "⚠️", 
            _ => "ℹ️"
        };
        println!("     {} {} (line {}, column {}): {}", 
            severity_icon,
            diagnostic.severity.to_uppercase(),
            diagnostic.line,
            diagnostic.column,
            diagnostic.message
        );
    }
    println!();
    
    // ℹ️ 5. Hover Information - Variable Inspection  
    println!("ℹ️ 5. HOVER INFORMATION - Variable Inspection");
    println!("─────────────────────────────────────────────────");
    
    let hover_template = "{{site_title}} has {{total_users}} users";
    let hover_positions = vec![
        (2, "site_title"),  // Position over 'site_title'
        (20, "total_users") // Position over 'total_users'
    ];
    
    for (pos, expected_var) in hover_positions {
        println!("🖱️ Hover at position {} (over '{}'):", pos, expected_var);
        match engine.get_hover_info_at_position(hover_template, pos, &context) {
            Ok(hover_info) => {
                println!("     Variable: {} ({})", hover_info.variable_name, hover_info.variable_type);
                println!("     Current Value: '{}'", hover_info.current_value);
                println!("     Description: {}", hover_info.description);
            }
            Err(e) => println!("     No hover information available: {}", e),
        }
        println!();
    }
    
    // 🔍 6. Go to Definition - Navigate to Definitions
    println!("🔍 6. GO TO DEFINITION - Navigate to Macro Definitions");
    println!("──────────────────────────────────────────────────────");
    
    // Find macro call position
    let macro_template = template;
    let macro_call_pos = macro_template.find("{{user_card(user, true)}}").unwrap() + 2;
    
    println!("🎯 Testing go-to-definition for macro call at position {}", macro_call_pos);
    match engine.get_definition_at_position(macro_template, macro_call_pos) {
        Ok(definition) => {
            println!("     Found definition!");
            println!("     Type: {}", definition.definition_type);
            println!("     Name: {}", definition.name);
            println!("     Location: line {}, column {}", definition.line, definition.column);
            if let Some(file) = &definition.file_path {
                println!("     File: {}", file);
            }
        }
        Err(e) => println!("     Definition not found: {}", e),
    }
    println!();
    
    // 🎯 7. Final Template Rendering
    println!("🎯 7. FINAL RENDERING - Put It All Together");
    println!("────────────────────────────────────────────────");
    
    let rendered = engine.render_string(template, &context)?;
    println!("📄 Rendered Output:");
    println!("{}", rendered);
    println!();
    
    println!("✅ IDE Integration Showcase Complete!");
    println!("🚀 All v0.4.1 IDE features demonstrated successfully!");
    
    Ok(())
}

fn create_sample_context() -> RuneScroll {
    let mut context = RuneScroll::new();
    
    // Site configuration
    context.set_string("site_title", "Mystical User Portal");
    context.set_number("total_users", 3);
    
    // Create sample users with nested data
    let users = vec![
        create_user("Alice", "alice@example.com", 42, 1250),
        create_user("Bob", "bob@example.com", 38, 890),
        create_user("Charlie", "charlie@example.com", 55, 2100),
    ];
    
    context.set("users", RuneSymbol::Array(users));
    
    context
}

fn create_user(name: &str, email: &str, level: i64, score: i64) -> RuneSymbol {
    let mut user = HashMap::new();
    user.insert("name".to_string(), RuneSymbol::String(name.to_string()));
    user.insert("email".to_string(), RuneSymbol::String(email.to_string()));
    
    // Nested stats object
    let mut stats = HashMap::new();
    stats.insert("level".to_string(), RuneSymbol::Number(level));
    stats.insert("score".to_string(), RuneSymbol::Number(score));
    
    user.insert("stats".to_string(), RuneSymbol::Object(stats));
    
    RuneSymbol::Object(user)
}