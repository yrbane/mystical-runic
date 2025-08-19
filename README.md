# 🔮 Mystical-Runic - Ancient Symbols for Modern Web Magic

*"In the beginning was the Word, and the Word was `{{mustache}}`..."*

[![Crates.io](https://img.shields.io/crates/v/mystical-runic.svg)](https://crates.io/crates/mystical-runic)
[![Documentation](https://docs.rs/mystical-runic/badge.svg)](https://docs.rs/mystical-runic)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://github.com/yrbane/mystical-runic/workflows/CI/badge.svg)](https://github.com/yrbane/mystical-runic/actions)

Welcome, brave developer, to the mystical realm of **Mystical-Runic** - where ancient Nordic symbols meet modern HTML templating in a beautiful dance of curly braces and digital sorcery!

## ✨ Features

🔒 **Security First**: XSS-safe by default with comprehensive HTML escaping  
⚡ **High Performance**: Template caching, bytecode compilation, parallel processing  
🎯 **Simple API**: Clean, intuitive interface for Rust developers  
🧪 **Well Tested**: 188+ tests with extensive security, performance, IDE integration, and developer experience tests  
🏗️ **Template Inheritance**: Advanced layout system with nested inheritance and `{{super}}`  
🔧 **Powerful Filters**: Built-in filters like `upper`, `lower`, `currency`, `truncate` with chaining support  
📦 **Reusable Macros**: Define and reuse template components with parameters  
🌊 **Deep Object Navigation**: Unlimited depth dot notation (e.g., `{{user.profile.stats.level}}`)  
🔄 **Nested Templates**: Full support for nested loops and recursive template includes  
🛡️ **Path Traversal Protection**: Enterprise-grade security preventing `../` attacks  
🌐 **Internationalization (i18n)**: Multi-language support with `{{t "key"}}` syntax and locale switching  
🔢 **Advanced Math Filters**: Mathematical operations with `add`, `multiply`, `divide`, `percentage`, `round`  
🎨 **Custom Filter API**: Register your own filters for domain-specific transformations  
🎭 **Dual Naming System**: Choose between professional (`TemplateEngine`) or mystical (`RuneEngine`) naming styles  
📝 **Smart Pluralization**: Automatic plural forms with `{{plural count "item" "items"}}`  
🔍 **Enhanced Error Messages**: Line/column numbers with helpful suggestions and context (v0.4.0)  
🐛 **Template Debugging**: Step-through debugging with variable tracking and execution insights (v0.4.0)  
🔥 **Hot Reload**: Development-time template reloading for faster iteration cycles (v0.4.0)  
💻 **IDE Integration**: Full Language Server Protocol support with auto-completion, syntax highlighting, error squiggles (v0.4.1)  
🌐 **Zero Dependencies**: Pure Rust implementation with no external dependencies  
🦀 **Modern Rust**: Rust 2021 edition with 1.74.0+ MSRV, future Rust 2024 ready  

## ⚡ The Sacred Incantations

### Core Magic
- **Whisper Variables**: `{{name}}` - Speak a name and it shall manifest (safely escaped from evil XSS spirits)
- **Summon Raw Power**: `{{& html}}` - Unleash unescaped HTML with great responsibility and greater danger
- **Divine Conditionals**: `{{if chosen_one}}...{{/if}}` - The HTML appears only for the worthy
- **Mystical Loops**: `{{for spell in grimoire}}...{{/for}}` - Repeat incantations until magic happens
- **Ancient Includes**: `{{include "scrolls/wisdom.html"}}` - Import wisdom from other sacred texts
- **Silent Whispers**: `{{! This is but a comment, invisible to mortals }}` - Notes for future wizards
- **Deep Path Traversal**: `{{user.profile.stats.level}}` - Navigate through nested object realms with unlimited depth

### Advanced Sorcery (v0.2.0)
- **Sacred Inheritance**: `{{extends "base.html"}}` - Inherit the power of ancestral templates
- **Mystical Blocks**: `{{block content}}...{{/block}}` - Define regions of power in your layouts
- **Ancestral Wisdom**: `{{super}}` - Channel the content of parent templates
- **Transformation Filters**: `{{name|upper|truncate:10}}` - Transform values with ancient filters
- **Reusable Spells (Macros)**: `{{macro spell(power)}}...{{/macro}}` - Create reusable incantations
- **Spell Invocation**: `{{spell("lightning")}}` - Call upon your defined macros

### Master Sorcery (v0.3.4) - The Advanced Features
- **Nested Loop Mastery**: `{{for category in shops}}{{for item in category.items}}...{{/for}}{{/for}}` - Complex nested iterations with stack-based parsing
- **Recursive Includes**: Templates can include other templates that include more templates - unlimited depth!
- **Path Traversal Wards**: Enterprise-grade protection against `../../../etc/passwd` and `C:\Windows\System32` attacks
- **Multilingual Magic**: `{{t "welcome" name=user}}` - Full i18n with variable interpolation
- **Smart Plurals**: `{{plural count "item" "items"}}` - Automatic singular/plural forms
- **Mathematical Alchemy**: `{{price|multiply:1.2|add:shipping|round:2|currency}}` - Complex calculations with filter chaining
- **Custom Enchantments**: Register your own filters with `engine.register_filter()`

### Global Sorcery (v0.3.0)
- **Universal Translation**: `{{t "welcome"}}` - Speak all tongues with i18n support
- **Locale Switching**: Switch between languages with `engine.set_locale("en")`
- **Quantity Wisdom**: `{{plural count "item" "items"}}` - Smart pluralization magic
- **Mathematical Alchemy**: `{{price|add:10|multiply:2|percentage}}` - Advanced math transformations
- **Custom Enchantments**: `engine.register_filter("reverse", |input, _| Ok(input.chars().rev().collect()))` - Forge your own filters

### Developer Experience (v0.4.0) - The Debugging Edition
- **Enhanced Error Diagnostics**: `ParseWithLocation` - Precise line/column error reporting with context
- **Intelligent Suggestions**: Template and variable name suggestions for typos
- **Debug Mode**: `engine.enable_debug_mode()` - Variable tracking and execution step analysis
- **Template Debugging**: `render_string_with_debug()` - Step-through debugging with performance metrics
- **Hot Reload**: `engine.enable_hot_reload()` - Automatic template reloading during development
- **Developer-Friendly Errors**: Stack traces for nested template errors with full context

### IDE Integration (v0.4.1) - The Editor Edition
- **Language Server Protocol**: `parse_for_lsp()` - Full LSP support for template editing
- **Syntax Highlighting**: `tokenize_for_syntax_highlighting()` - Semantic token analysis
- **Auto-completion**: `get_completions_at_position()` - Variables, filters, and directive completion
- **Real-time Diagnostics**: `get_diagnostics_for_editor()` - Error squiggles and warnings
- **Hover Information**: `get_hover_info_at_position()` - Variable type and value inspection
- **Go to Definition**: `get_definition_at_position()` - Navigate to macro definitions

## 🚀 Quick Start

### Installation

```toml
[dependencies]
mystical-runic = "0.4.1"
```

### Basic Usage - Choose Your Style! 🎭

#### 🏢 Professional Style (Conventional)
Perfect for corporate environments, team projects, and those who prefer explicit naming:

```rust
use mystical_runic::{TemplateEngine, TemplateContext, TemplateValue};

let mut engine = TemplateEngine::new("templates");
let mut context = TemplateContext::new();

// Set variables in your template context
context.set("hero", TemplateValue::String("Professional Developer".to_string()));
context.set("level", TemplateValue::Number(99));
context.set("has_coffee", TemplateValue::Bool(true));

let result = engine.render_string(
    "Hello {{hero}} of level {{level}}! {{if has_coffee}}☕ Ready to work!{{/if}}", 
    &context
).unwrap();
```

#### 🔮 Mystical Style (Themed)
For the adventurous, creative projects, and those who enjoy a touch of magic:

```rust
use mystical_runic::{RuneEngine, RuneScroll, RuneSymbol};

// Summon the ancient engine from the template realm
let mut engine = RuneEngine::new("sacred_scrolls");
let mut scroll = RuneScroll::new();

// Inscribe your desires upon the scroll
scroll.set("hero", RuneSymbol::String("Mystical Sorcerer".to_string()));
scroll.set("level", RuneSymbol::Number(99));
scroll.set("has_coffee", RuneSymbol::Bool(true));

// Speak the incantation and witness the transformation
let result = engine.render_string(
    "Behold! {{hero}} of level {{level}} embarks upon their quest! {{if has_coffee}}☕{{/if}}", 
    &scroll
).unwrap();
```

> **💡 Pro Tip**: Both styles are completely interchangeable! Use whichever feels right for your project. You can even mix them in the same codebase - they're just aliases for the same underlying types.

### Advanced Example: Character Sheet Generator

```rust
use mystical_runic::{RuneEngine, RuneScroll, RuneSymbol};
use std::collections::HashMap;

let mut engine = RuneEngine::new(".");
let mut scroll = RuneScroll::new();

// Create a magical character
let mut character = HashMap::new();
character.insert("name".to_string(), RuneSymbol::String("Gandalf the Grey".to_string()));
character.insert("class".to_string(), RuneSymbol::String("Wizard".to_string()));
character.insert("level".to_string(), RuneSymbol::Number(85));
character.insert("mana".to_string(), RuneSymbol::Number(9999));
character.insert("has_staff".to_string(), RuneSymbol::Bool(true));

scroll.set("character", RuneSymbol::Object(character));

// ⚔️ Create spell list
let spells = vec![
    create_spell("Fireball", 50, "🔥"),
    create_spell("Lightning Bolt", 75, "⚡"),
    create_spell("Ice Shard", 40, "❄️"),
];
scroll.set("spells", RuneSymbol::Array(spells));

let character_sheet = r#"
🧙‍♂️ Name: {{character.name}}
🎓 Class: {{character.class}} (Level {{character.level}})
🔮 Mana: {{character.mana}}

{{if character.has_staff}}
🪄 Equipment: Magical Staff of Power
{{/if}}

⚡ KNOWN SPELLS:
{{for spell in spells}}
  {{spell.icon}} {{spell.name}} - Power: {{spell.damage}}
{{/for}}
"#;

let result = engine.render_string(character_sheet, &scroll).unwrap();
println!("{}", result);

fn create_spell(name: &str, damage: i64, icon: &str) -> RuneSymbol {
    let mut spell = HashMap::new();
    spell.insert("name".to_string(), RuneSymbol::String(name.to_string()));
    spell.insert("damage".to_string(), RuneSymbol::Number(damage));
    spell.insert("icon".to_string(), RuneSymbol::String(icon.to_string()));
    RuneSymbol::Object(spell)
}
```

### Deep Dot Notation Example

```rust
use mystical_runic::{RuneEngine, RuneScroll, RuneSymbol};
use std::collections::HashMap;

let mut engine = RuneEngine::new(".");
let mut scroll = RuneScroll::new();

// Create deeply nested game data structure
let mut stats = HashMap::new();
stats.insert("level".to_string(), RuneSymbol::Number(42));
stats.insert("health".to_string(), RuneSymbol::Number(100));
stats.insert("mana".to_string(), RuneSymbol::Number(75));

let mut equipment = HashMap::new();
equipment.insert("weapon".to_string(), RuneSymbol::String("Mystical Sword".to_string()));
equipment.insert("armor".to_string(), RuneSymbol::String("Dragon Scale".to_string()));

let mut character = HashMap::new();
character.insert("name".to_string(), RuneSymbol::String("Aragorn".to_string()));
character.insert("class".to_string(), RuneSymbol::String("Ranger".to_string()));
character.insert("stats".to_string(), RuneSymbol::Object(stats));
character.insert("equipment".to_string(), RuneSymbol::Object(equipment));

let mut game_data = HashMap::new();
game_data.insert("character".to_string(), RuneSymbol::Object(character));

scroll.set("game", RuneSymbol::Object(game_data));

// Use deep dot notation to access nested values
let template = r#"
🎮 GAME CHARACTER SHEET 🎮

👤 Name: {{game.character.name}}
🎭 Class: {{game.character.class}}

📊 STATS:
❤️ Health: {{game.character.stats.health}}
💙 Mana: {{game.character.stats.mana}}
⭐ Level: {{game.character.stats.level}}

⚔️ EQUIPMENT:
{{if game.character.equipment.weapon}}
🗡️ Weapon: {{game.character.equipment.weapon}}
{{/if}}
{{if game.character.equipment.armor}}
🛡️ Armor: {{game.character.equipment.armor}}
{{/if}}

{{if game.character.stats.level}}
🏆 Status: {{if game.character.stats.health}}Combat Ready{{/if}}
{{/if}}
"#;

let result = engine.render_string(template, &scroll).unwrap();
println!("{}", result);

// 🔥 NEW v0.4.0: Debug Mode for Development
engine.enable_debug_mode();
let debug_result = engine.render_string_with_debug(template, &scroll).unwrap();

println!("Rendered: {}", debug_result.output);
println!("Variables accessed: {:?}", debug_result.debug_info.variables_accessed);
println!("Execution time: {}ms", debug_result.debug_info.performance_metrics.total_time_nanos / 1_000_000);
```

### IDE Integration Example (v0.4.1)

```rust
use mystical_runic::{RuneEngine, RuneScroll, RuneSymbol};

let mut engine = RuneEngine::new(".");
let mut context = RuneScroll::new();
context.set_string("user_name", "Alice");
context.set_number("user_level", 42);

let template = r#"
{{macro greeting(name, level)}}
<h1>Hello {{name}}! Level: {{level}}</h1>
{{/macro}}

{{greeting(user_name, user_level)}}
{{if user_level}}
    {{user_name|upper}} is ready!
{{/if}}
"#;

// 🔍 NEW v0.4.1: Language Server Protocol Support
let lsp_info = engine.parse_for_lsp(template, "greeting.html").unwrap();
println!("Variables found: {:?}", lsp_info.variables);
println!("Macros found: {:?}", lsp_info.macros);

// 💡 NEW v0.4.1: Auto-completion at cursor position  
let position = 95; // Position in "{{user_na|" 
let completions = engine.get_completions_at_position(template, position, &context).unwrap();
for completion in completions {
    println!("Suggestion: {} ({})", completion.label, completion.completion_type);
}

// 🎨 NEW v0.4.1: Syntax highlighting tokens
let tokens = engine.tokenize_for_syntax_highlighting(template).unwrap();
for token in tokens.iter().take(5) {
    println!("Token: '{}' ({})", token.content, token.token_type);
}

// 🚨 NEW v0.4.1: Real-time error diagnostics
let invalid_template = "{{name}} {{unknown_variable}} {{name|nonexistent_filter}}";
let diagnostics = engine.get_diagnostics_for_editor(invalid_template, &context).unwrap();
for diagnostic in diagnostics {
    println!("⚠️ {}: {} (line {})", diagnostic.severity, diagnostic.message, diagnostic.line);
}

// ℹ️ NEW v0.4.1: Hover information  
let hover_position = 50; // Position over a variable
if let Ok(hover_info) = engine.get_hover_info_at_position(template, hover_position, &context) {
    println!("Hover: {} ({}) = {}", hover_info.variable_name, hover_info.variable_type, hover_info.current_value);
}

// 🔍 NEW v0.4.1: Go to definition
let macro_position = 200; // Position over a macro call
if let Ok(definition) = engine.get_definition_at_position(template, macro_position) {
    println!("Definition: {} at line {} column {}", definition.name, definition.line, definition.column);
}
```

## 🎮 Complete Real-World Demo Application

Experience **ALL** features of Mystical-Runic v0.3.4 with our comprehensive demonstration application!

```bash
# Run the complete feature showcase
cd examples/real_world_demo
./run_demo.sh

# OR manually
cargo run
```

### 🌟 What the Demo Showcases

Our demo application is a **full-featured e-commerce/blog site** that demonstrates every single feature:

- **🔄 NEW v0.3.4**: Nested loops with complex data structures (categories → products)
- **🔄 NEW v0.3.4**: Recursive includes with 4 levels deep (profile → stats → preferences → comments)
- **🛡️ NEW v0.3.4**: Path traversal protection blocking `../../../etc/passwd` attacks
- **🏗️ Template inheritance** with `base.html` → `shop.html` 
- **🌐 Full i18n** with French/English translations
- **📝 Smart pluralization** throughout the interface
- **🔢 Advanced math filters** for pricing, taxes, discounts
- **🎨 Custom filters** for encryption, markdown, dates
- **📦 Reusable macros** for product cards
- **🌊 Deep dot notation** accessing `user.profile.settings.theme`
- **🔒 XSS protection** with real attack demonstrations
- **⚡ Performance features** with benchmarking

### 📊 Demo Results

```
🔮 MYSTICAL-RUNIC v0.3.4 - DÉMONSTRATION COMPLÈTE!
✅ Main template rendered successfully! (10KB+ HTML)
✅ Nested loops work perfectly!
✅ Nested includes work perfectly! 
✅ All path traversal attempts blocked!
✅ XSS protection working
✅ Rendered 100 times in 1-2ms
✅ HTML file generated: output_demo.html

🎉 All 173+ tests pass! Production ready!
```

### 🎯 Generated Output

The demo creates `output_demo.html` - a complete working website showcasing:
- Professional styling with responsive design
- Real e-commerce functionality simulation  
- Complex nested data visualization
- Multi-language interface
- Security feature demonstrations
- Performance metrics display

**📁 Location**: `examples/real_world_demo/` - Complete application with 7+ templates, realistic data, and full documentation.

## 🏰 Template Inheritance (v0.2.0)

Create sophisticated layouts with template inheritance:

```html
<!-- base.html -->
<!DOCTYPE html>
<html>
<head>
    <title>{{block title}}My Site{{/block}}</title>
</head>
<body>
    <header>{{block header}}Default Header{{/block}}</header>
    <main>{{block content}}{{/block}}</main>
    <footer>{{block footer}}© 2024 My Site{{/block}}</footer>
</body>
</html>
```

```html
<!-- admin.html -->
{{extends "base.html"}}

{{block title}}Admin Panel - {{block page_title}}{{/block}}{{/block}}

{{block content}}
<div class="admin-layout">
    <nav>{{block sidebar}}Default Sidebar{{/block}}</nav>
    <main>{{block admin_content}}{{/block}}</main>
</div>
{{/block}}
```

```html
<!-- admin_users.html -->
{{extends "admin.html"}}

{{block page_title}}User Management{{/block}}

{{block admin_content}}
<h1>Users</h1>
{{for user in users}}
    <div class="user-card">{{user.name}} - {{user.role}}</div>
{{/for}}
{{/block}}
```

## 🔧 Powerful Filters (v0.2.0)

Transform your data with built-in filters:

```html
<h1>{{title|upper}}</h1>                    <!-- HELLO WORLD -->
<p>{{description|lower}}</p>                <!-- hello world -->
<span>${{price|currency}}</span>            <!-- $12.99 -->
<div>{{content|truncate:50}}</div>          <!-- Truncated text... -->
<time>{{date|date:"Y-m-d"}}</time>          <!-- 2024-01-15 -->

<!-- Chain multiple filters -->
<p>{{name|lower|capitalize}}</p>            <!-- John Doe -->
<span>{{text|strip|truncate:20|upper}}</span>   <!-- TRIMMED TEXT... -->
```

## 📦 Reusable Macros (v0.2.0)

Create reusable template components:

```html
<!-- Define macros -->
{{macro button(text, type="button", class="btn")}}
<button type="{{type}}" class="{{class}}">{{text}}</button>
{{/macro}}

{{macro card(title, content, class="card")}}
<div class="{{class}}">
    <h3 class="card-title">{{title}}</h3>
    <div class="card-body">{{content}}</div>
</div>
{{/macro}}

{{macro user_card(user)}}
<div class="user-card">
    <h4>{{user.name}}</h4>
    <p>{{user.email}}</p>
    {{if user.active}}
        <span class="status active">Online</span>
    {{/if}}
</div>
{{/macro}}

<!-- Use macros -->
{{button("Save", type="submit", class="btn btn-primary")}}
{{card("Welcome", "This is a welcome message", class="card highlight")}}

{{for user in users}}
    {{user_card(user)}}
{{/for}}
```

## 📖 Template Syntax Guide

### Variables

```html
<!-- Safe HTML escaping (default) -->
<p>{{user_input}}</p>

<!-- Raw HTML output (use with caution) -->
<div>{{& trusted_html}}</div>

<!-- Object properties -->
<span>{{user.name}} ({{user.email}})</span>

<!-- Deep nested properties -->
<div>Level: {{player.character.stats.level}}</div>
<p>{{config.database.connection.host}}:{{config.database.connection.port}}</p>
```

### Conditionals

```html
{{if user.is_admin}}
  <button class="admin-panel">Admin Controls</button>
{{/if}}

{{if items}}
  <ul class="item-list">
    <!-- items exist -->
  </ul>
{{/if}}

<!-- Deep conditionals -->
{{if user.settings.notifications.email.enabled}}
  <p>Email notifications are on</p>
{{/if}}

{{if config.features.advanced.enabled}}
  <div class="advanced-features">Advanced mode active</div>
{{/if}}
```

**Truthiness Rules:**
- Strings: non-empty = true, empty = false
- Numbers: non-zero = true, zero = false
- Booleans: as expected
- Arrays: non-empty = true, empty = false
- Objects: non-empty = true, empty = false

### Loops

```html
{{for product in products}}
  <div class="product">
    <h3>{{product.name}}</h3>
    <p>Price: ${{product.price}}</p>
    {{if product.on_sale}}
      <span class="sale-badge">ON SALE!</span>
    {{/if}}
  </div>
{{/for}}
```

### Template Includes

```html
<!-- main.html -->
<!DOCTYPE html>
<html>
<head>
  {{include "partials/head.html"}}
</head>
<body>
  {{include "partials/header.html"}}
  <main>{{content}}</main>
  {{include "partials/footer.html"}}
</body>
</html>
```

### Comments

```html
{{! This comment will not appear in the output }}
<div>
  {{! 
    Multi-line comments
    are also supported
  }}
  <p>Visible content</p>
</div>
```

## 🔒 Security Features

Mystical-Runic takes security seriously and provides multiple layers of protection:

### XSS Prevention

```rust
let mut context = RuneScroll::new();
context.set_string("user_input", "<script>alert('xss')</script>");

let result = engine.render_string("{{user_input}}", &context).unwrap();
// Output: &lt;script&gt;alert(&#x27;xss&#x27;)&lt;/script&gt;
```

### Path Traversal Protection

```rust
// These will safely fail:
engine.render("../../../etc/passwd", &context);  // ❌ Blocked
engine.render("..\\windows\\system32", &context);  // ❌ Blocked
```

### Template Injection Prevention

```rust
context.set_string("malicious", "{{admin_password}}");
let result = engine.render_string("{{malicious}}", &context).unwrap();
// Output: {{admin_password}} (literal text, not executed)
```

## 🎨 API Reference

### RuneEngine (TemplateEngine)

```rust
let mut engine = RuneEngine::new("path/to/templates");

// Render from file
let result = engine.render("template.html", &context)?;

// Render from string
let result = engine.render_string("Hello {{name}}!", &context)?;

// Load template (with caching)
let template_content = engine.load_template("header.html")?;
```

### RuneScroll (TemplateContext)

```rust
let mut scroll = RuneScroll::new();

// Set different value types
scroll.set_string("name", "value");
scroll.set_number("count", 42);
scroll.set_bool("active", true);

// Set complex values
scroll.set("array", RuneSymbol::Array(vec![...]));
scroll.set("object", RuneSymbol::Object(hashmap));

// Retrieve values
let value = scroll.get_string("name");
```

### RuneSymbol (TemplateValue)

```rust
// Create different value types
let string_val = RuneSymbol::String("text".to_string());
let number_val = RuneSymbol::Number(42);
let bool_val = RuneSymbol::Bool(true);
let array_val = RuneSymbol::Array(vec![...]);
let object_val = RuneSymbol::Object(hashmap);
```

## 🧪 Testing

Run the comprehensive test suite:

```bash
# Run all tests
cargo test

# Run specific test categories
cargo test integration_tests
cargo test unit_tests
cargo test security_tests

# Run with output
cargo test -- --nocapture
```

### Internationalization Example (v0.3.0)
```rust
use mystical_runic::{RuneEngine, RuneScroll, RuneSymbol};
use std::collections::HashMap;

let mut engine = RuneEngine::new("./templates");
let mut context = RuneScroll::new();
context.set("name", RuneSymbol::String("Alice".to_string()));

// Set up English translations
let mut en_translations = HashMap::new();
en_translations.insert("welcome".to_string(), "Welcome {{name}}!".to_string());
engine.set_translations("en", en_translations);

// Set up French translations
let mut fr_translations = HashMap::new();
fr_translations.insert("welcome".to_string(), "Bienvenue {{name}}!".to_string());
engine.set_translations("fr", fr_translations);

// Use English
engine.set_locale("en");
let welcome_en = engine.render_string("{{t \"welcome\"}}", &context).unwrap();
// Output: "Welcome Alice!"

// Switch to French
engine.set_locale("fr");
let welcome_fr = engine.render_string("{{t \"welcome\"}}", &context).unwrap();
// Output: "Bienvenue Alice!"
```

### Custom Filters Example (v0.3.0)
```rust
use mystical_runic::{RuneEngine, RuneScroll, RuneSymbol};

let mut engine = RuneEngine::new("./templates");
let mut context = RuneScroll::new();
context.set("text", RuneSymbol::String("hello world".to_string()));

// Register custom filters
engine.register_filter("reverse", |input: &str, _args: &[&str]| {
    Ok(input.chars().rev().collect())
});

engine.register_filter("repeat", |input: &str, args: &[&str]| {
    let times = args.get(0).map_or("1", |v| v).parse::<usize>().unwrap_or(1);
    Ok(input.repeat(times))
});

let result = engine.render_string("{{text|reverse|upper|repeat:2}}", &context).unwrap();
// Output: "DLROW OLLEHDLROW OLLEH"
```

### Math Filters Example (v0.3.0)  
```rust
use mystical_runic::{RuneEngine, RuneScroll, RuneSymbol};

let mut engine = RuneEngine::new("./templates");
let mut context = RuneScroll::new();
context.set("price", RuneSymbol::Number(100));
context.set("tax_rate", RuneSymbol::Number(8));

// Complex calculations with chaining
let template = "Price: ${{price}}, Total: {{price|multiply:tax_rate|divide:100|add:price|round:2}}";
let result = engine.render_string(template, &context).unwrap();
// Output: "Price: $100, Total: 108.00"
```

### Pluralization Example (v0.3.0)
```rust
use mystical_runic::{RuneEngine, RuneScroll, RuneSymbol};

let mut engine = RuneEngine::new("./templates");
let mut context = RuneScroll::new();

let template = "You have {{count}} {{plural count \"apple\" \"apples\"}}";

context.set("count", RuneSymbol::Number(1));
let result = engine.render_string(template, &context).unwrap();
// Output: "You have 1 apple"

context.set("count", RuneSymbol::Number(5)); 
let result = engine.render_string(template, &context).unwrap();
// Output: "You have 5 apples"
```

## 🔮 Examples

Check out the [`examples/`](examples/) directory for magical demonstrations:

- **[`real_world_demo/`](examples/real_world_demo/)** - **⭐ COMPLETE DEMO APPLICATION** - Full-featured e-commerce/blog showcasing ALL v0.3.4 features
  - 🎮 Run with: `cd examples/real_world_demo && ./run_demo.sh`
  - 📄 Generates complete HTML website (10KB+)
  - 🔄 NEW: Nested loops, recursive includes, path traversal protection
  - 🌐 Multi-language (FR/EN), math filters, custom filters, macros, inheritance
- [`spell_casting.rs`](examples/spell_casting.rs) - Fantasy RPG character sheet generator
- More examples coming with each release!

## 🦀 Rust Compatibility

**Minimum Supported Rust Version (MSRV)**: 1.74.0+  
**Edition**: Rust 2021 (with future Rust 2024 readiness)  
**Tested on**: Rust 1.74.0 through latest stable

### Future-Proof Design
- Modern Rust patterns and idioms  
- Prepared for Rust 2024 edition migration
- Zero unsafe code  
- Comprehensive test coverage (173+ tests)  
- Zero compilation warnings

### Development Setup

```bash
git clone https://github.com/yrbane/mystical-runic.git
cd mystical-runic
cargo build
cargo test
```

## 🧪 Test-Driven Development (TDD) Methodology

Mystical-Runic follows strict **Test-Driven Development** practices. When contributing, please observe the sacred TDD ritual:

### 🔴 Red → 🟢 Green → 🔵 Refactor Cycle

1. **🔴 RED - Write a Failing Test First**
   ```bash
   # Write your test before any implementation
   cargo test your_new_feature_test
   # ❌ Should fail - good!
   ```

2. **🟢 GREEN - Write Minimal Code to Pass**
   ```bash
   # Write just enough code to make the test pass
   cargo test your_new_feature_test
   # ✅ Should pass - excellent!
   ```

3. **🔵 REFACTOR - Improve Without Breaking**
   ```bash
   # Clean up code while keeping tests green
   cargo test  # All tests should still pass
   ```



---

*"Tests are the safety net that lets you refactor fearlessly."* — TDD Proverb

## 📜 Changelog

### v0.4.1 (Latest Release) - The IDE Integration Edition

- 💻 **NEW: Language Server Protocol Support**: Complete LSP implementation for template editing with `parse_for_lsp()`
- 🎨 **NEW: Syntax Highlighting**: Semantic token analysis with `tokenize_for_syntax_highlighting()` for editor integration
- 💡 **NEW: Auto-completion**: Intelligent completion for variables, filters, and directives with `get_completions_at_position()`
- 🚨 **NEW: Real-time Diagnostics**: Error squiggles and warnings with `get_diagnostics_for_editor()` for live error detection
- ℹ️ **NEW: Hover Information**: Variable type and value inspection with `get_hover_info_at_position()`
- 🔍 **NEW: Go to Definition**: Navigate to macro definitions with `get_definition_at_position()`
- 🧹 **Code Quality**: Zero compiler warnings, clean production-ready codebase
- 🧪 **198+ Tests**: Comprehensive test suite including all v0.4.1 IDE integration features
- 🚀 **Production Ready**: All tests passing, full IDE support for enhanced developer experience

### v0.4.0 - The Developer Experience Edition

- 🔍 **NEW: Enhanced Error Messages**: Precise line/column error reporting with helpful context and suggestions
- 🐛 **NEW: Template Debugging**: Complete debugging system with variable tracking and execution step analysis  
- 🔥 **NEW: Hot Reload**: Automatic template reloading during development for faster iteration cycles
- 📊 **NEW: Performance Metrics**: Built-in performance tracking with execution time analysis
- 🎯 **NEW: Intelligent Suggestions**: Smart suggestions for template and variable name typos
- 🧪 **198+ Tests**: Comprehensive test suite including all v0.4.0 developer experience features and v0.4.1 IDE integration
- 🚀 **Production Ready**: All tests passing, enhanced developer productivity tools

### v0.3.4 - The Advanced Features Edition

- 🔄 **NEW: Nested Loops**: Complete support for nested loops with stack-based parsing (`{{for category in shops}}{{for item in category.items}}`)
- 🔄 **NEW: Recursive Includes**: Unlimited depth recursive template includes (templates including templates)
- 🛡️ **NEW: Path Traversal Protection**: Enterprise-grade security preventing `../../../etc/passwd` and `C:\Windows\System32` attacks
- 🎮 **Complete Real-World Demo**: Full-featured e-commerce/blog application showcasing ALL features (`examples/real_world_demo/`)
- 📚 **Enhanced Documentation**: Complete README overhaul with comprehensive examples and demo application
- 🧪 **173+ Tests**: Expanded test suite including all v0.3.4 features with comprehensive security testing
- 🚀 **Production Ready**: All tests passing, zero warnings, full feature demonstration

### v0.3.3 - The Warning-Free Edition

- 🧹 **Zero Warnings**: Complete cleanup of all compiler warnings for production readiness
- 🔧 **Code Quality**: Enhanced code quality and maintainability improvements
- ✅ **Stability**: All existing functionality maintained with improved reliability

### v0.3.2 - The Enhancement Edition  

- 🌐 **Enhanced i18n**: Improved internationalization with better variable interpolation
- 📝 **Smart Pluralization**: Advanced plural form handling with locale-aware rules
- 🔢 **Math Filter Improvements**: Enhanced mathematical operations with better precision
- 🎨 **Custom Filter API**: Improved API for registering custom filters

### v0.3.1 - The Stability Edition

- 🔧 **Bug Fixes**: Critical fixes for edge cases in template processing
- ⚡ **Performance**: Optimized parsing and rendering pipeline
- 🧪 **Testing**: Enhanced test coverage for reliability improvements

### v0.3.0 - The Global Sorcery Edition

- 🌐 **Internationalization (i18n)**: Full multi-language support with `{{t "key"}}` syntax
- 📝 **Smart Pluralization**: Automatic plural forms with `{{plural count "item" "items"}}`
- 🔢 **Advanced Math Filters**: Mathematical operations (`add`, `multiply`, `divide`, `percentage`, `round`)
- 🎨 **Custom Filter API**: Register your own filters with `engine.register_filter()`
- 🎭 **Dual Naming System**: Choose between professional (`TemplateEngine`) or mystical (`RuneEngine`) styles
- 🧪 **150+ Tests**: Comprehensive test suite covering all new features

### v0.2.0 - The Advanced Sorcery Edition

- 🏰 **Template Inheritance**: Advanced layout system with nested inheritance support
- 🔧 **Powerful Filters**: Built-in filters (`upper`, `lower`, `currency`, `truncate`, `date`) with chaining
- 📦 **Reusable Macros**: Define and invoke template components with parameters
- 🌊 **Enhanced Deep Navigation**: Unlimited depth dot notation (`{{game.player.stats.level}}`)
- ⚡ **Performance Boost**: Bytecode compilation, parallel processing, memory mapping
- 🧪 **127+ Tests**: Comprehensive test coverage including v0.2.0 features
- 🔧 **Bug Fixes**: Fixed nested layout inheritance and function call error handling
- 🌐 **Zero Dependencies**: Pure Rust implementation

### v0.1.4 (Stability Release)

- 🔧 Fixed nested layout inheritance block replacement boundary calculation
- 🛡️ Enhanced loop error handling for unsupported function calls
- 🔄 Maintained backward compatibility for missing variables in loops
- ✅ All 127 tests passing with comprehensive coverage

### v0.1.1 (Security & Testing Release)

- 🛡️ Comprehensive security testing suite
- 🧪 85+ tests with 100% coverage following TDD methodology
- 📚 Complete documentation with TDD development guidelines
- 🔒 Advanced XSS and injection protection
- ⚡ Performance optimizations and stress testing
- 🔴🟢🔵 Strict Test-Driven Development practices implemented

### v0.1.0 (Initial Release)

- ✨ Core template engine with Mustache-inspired syntax
- 🔒 XSS-safe HTML escaping by default
- ⚡ Template caching for performance
- 🎯 Support for variables, conditionals, loops, includes, and comments
- 🧪 Comprehensive test suite with high coverage
- 📚 Complete documentation and examples

## 🗺️ Roadmap

### ✅ v0.3.0-v0.4.1 - COMPLETED
- ✅ **i18n Support**: `{{t "key"}}` syntax for translations
- ✅ **Pluralization**: Smart plural forms based on count  
- ✅ **Custom Filter Registration**: API for user-defined filters
- ✅ **Advanced Math Filters**: Mathematical operations and formatting
- ✅ **Nested Loops**: Stack-based parsing for complex nested structures
- ✅ **Recursive Includes**: Deep template inclusion hierarchies
- ✅ **Path Traversal Protection**: Enterprise-grade security features
- ✅ **Complete Real-World Demo**: Full-featured showcase application
- ✅ **Enhanced Error Messages**: Line/column numbers and intelligent suggestions
- ✅ **Template Debugging**: Step-through debugging with variable tracking
- ✅ **Hot Reload**: Development-time template reloading
- ✅ **Language Server Protocol**: Complete LSP support for template editing
- ✅ **Syntax Highlighting**: Semantic token analysis for editor integration
- ✅ **Auto-completion**: Intelligent variable, filter, and directive completion
- ✅ **Real-time Diagnostics**: Error squiggles and warnings in editors
- ✅ **Hover Information**: Variable type and value inspection
- ✅ **Go to Definition**: Navigate to macro and template definitions

### 🌐 v0.5.0 - Ecosystem Integration
- **Async Support**: Non-blocking template rendering
- **Web Framework Integration**: First-class Axum, Warp, Actix support
- **WASM Compatibility**: Browser and edge runtime support
- **CLI Tools**: Command-line template processing utilities

## 🌟 Why "Mystical-Runic"?

Because templating is basically ancient magic:
- You write mysterious symbols (`{{}}`) that transform into reality
- Variables appear and disappear like spirits
- One wrong bracket and your entire spell explodes
- Senior developers guard the template secrets like ancient druids
- Documentation is written in a language only the initiated understand
- And just like real magic, it works perfectly until production 🔥

## 🔗 Links

- **Documentation**: [docs.rs/mystical-runic](https://docs.rs/mystical-runic)
- **Crates.io**: [crates.io/crates/mystical-runic](https://crates.io/crates/mystical-runic)
- **Repository**: [github.com/yrbane/mystical-runic](https://github.com/yrbane/mystical-runic)
- **Issues**: [github.com/yrbane/mystical-runic/issues](https://github.com/yrbane/mystical-runic/issues)

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

*"May your templates be bug-free and your variables always defined."*  
— Ancient DevOps Proverb

🔮✨ Happy templating! ✨🔮