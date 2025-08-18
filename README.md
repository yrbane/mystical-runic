# 🔮 Mystical-Runic - Ancient Symbols for Modern Web Magic

*"In the beginning was the Word, and the Word was `{{mustache}}`..."*

[![Crates.io](https://img.shields.io/crates/v/mystical-runic.svg)](https://crates.io/crates/mystical-runic)
[![Documentation](https://docs.rs/mystical-runic/badge.svg)](https://docs.rs/mystical-runic)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://github.com/yrbane/mystical-runic/workflows/CI/badge.svg)](https://github.com/yrbane/mystical-runic/actions)

Welcome, brave developer, to the mystical realm of **Mystical-Runic** - where ancient Nordic symbols meet modern HTML templating in a beautiful dance of curly braces and digital sorcery!

## ✨ Features

🔒 **Security First**: XSS-safe by default with comprehensive HTML escaping  
⚡ **Performance**: Template caching and efficient parsing  
🎯 **Simple API**: Clean, intuitive interface for Rust developers  
🧪 **Well Tested**: 100% test coverage with extensive security tests  
📚 **Rich Syntax**: Variables, conditionals, loops, includes, and comments  
🌐 **Unicode Safe**: Full Unicode support with proper encoding  

## ⚡ The Sacred Incantations

- **Whisper Variables**: `{{name}}` - Speak a name and it shall manifest (safely escaped from evil XSS spirits)
- **Summon Raw Power**: `{{& html}}` - Unleash unescaped HTML with great responsibility and greater danger
- **Divine Conditionals**: `{{if chosen_one}}...{{/if}}` - The HTML appears only for the worthy
- **Mystical Loops**: `{{for spell in grimoire}}...{{/for}}` - Repeat incantations until magic happens
- **Ancient Includes**: `{{include "scrolls/wisdom.html"}}` - Import wisdom from other sacred texts
- **Silent Whispers**: `{{! This is but a comment, invisible to mortals }}` - Notes for future wizards
- **Object Divination**: `{{user.power_level}}` - Peer into the properties of mystical entities
- **Deep Path Traversal**: `{{user.profile.stats.level}}` - Navigate through nested object realms with unlimited depth

## 🚀 Quick Start

### Installation

```toml
[dependencies]
mystical-runic = "0.1.3"
```

### Basic Usage

```rust
use mystical_runic::{RuneEngine, RuneScroll, RuneSymbol};

// Summon the ancient engine from the template realm
let mut engine = RuneEngine::new("templates");
let mut scroll = RuneScroll::new();

// Inscribe your desires upon the scroll
scroll.set_string("hero", "Rust Developer");
scroll.set_string("quest", "Debug Production Issues");
scroll.set_number("level", 99);
scroll.set_bool("has_coffee", true);

// Speak the incantation and witness the transformation
let result = engine.render_string(
    "Behold! {{hero}} of level {{level}} embarks upon {{quest}}! {{if has_coffee}}☕{{/if}}", 
    &scroll
).unwrap();

assert_eq!(result, "Behold! Rust Developer of level 99 embarks upon Debug Production Issues! ☕");
```

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

## 🔮 Examples

Check out the [`examples/`](examples/) directory for more magical demonstrations:

- [`spell_casting.rs`](examples/spell_casting.rs) - Fantasy RPG character sheet generator
- More examples coming soon!

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

### v0.1.1 (Latest Release)

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