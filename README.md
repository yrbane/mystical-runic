# 🔮 Runic - Ancient Symbols for Modern Web Magic

*"With great power comes great responsibility... and curly braces."*

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Crates.io](https://img.shields.io/crates/v/runic.svg)](https://crates.io/crates/runic)
[![Documentation](https://docs.rs/runic/badge.svg)](https://docs.rs/runic)

## 🧙‍♂️ What is Runic?

**Runic** is a mystical templating engine that transforms ancient Nordic symbols (okay, just `{{}}`) into beautiful HTML. Born from the sacred fires of Rust and tempered in the forge of TDD, it brings magic to your web applications without the complexity of summoning actual demons.

## ⚡ Sacred Features

- 🔮 **Variable Conjuring**: `{{hero}}` - Manifest variables from the ethereal realm
- 🛡️ **XSS Protection**: Automatically blessed with anti-evil protection (HTML escaping)
- 🌟 **Raw Power**: `{{& dangerous_html}}` - For when you need to summon unescaped HTML
- 🎭 **Conditional Magic**: `{{if chosen_one}}You're a wizard, Harry!{{/if}}`
- 🔄 **Mystical Loops**: `{{for spell in spellbook}}Cast {{spell}}{{/for}}`
- 📜 **Ancient Scrolls**: `{{include "wisdom/ancient.html"}}` - Import other templates
- 💭 **Silent Whispers**: `{{! Only wizards can see this comment }}`
- 🎯 **Object Divination**: `{{user.power_level}}` - Access object properties

## 🚀 Quick Spell Casting

```toml
[dependencies]
runic = "0.1.0"
```

```rust
use runic::{RuneEngine, RuneScroll, RuneSymbol};

// Summon the ancient engine
let mut engine = RuneEngine::new("templates");
let mut scroll = RuneScroll::new();

// Inscribe your magical variables
scroll.set_string("hero", "Rust Developer");
scroll.set_string("villain", "Memory Leak");
scroll.set_bool("has_sword", true);

// Cast the spell!
let spell = r#"
{{if has_sword}}
⚔️ {{hero}} draws their blade against {{villain}}!
{{/if}}
"#;

let result = engine.render_string(spell, &scroll)?;
println!("{}", result);
```

## 📚 Ancient Wisdom (Documentation)

### Variable Manifestation
```html
<!-- Safely escaped (protected against dark magic) -->
<h1>Welcome, {{user_name}}!</h1>

<!-- Raw power (use with caution) -->
<div>{{& trusted_html_content}}</div>
```

### Conditional Sorcery
```html
{{if user.is_admin}}
  <button class="delete-universe">🔴 Don't Press This</button>
{{/if}}

{{if coffee_level}}
  <p>☕ Developer is functional</p>
{{/if}}
```

### Mystical Iterations
```html
<ul class="spell-list">
{{for spell in spellbook}}
  <li>{{spell.name}} - Power: {{spell.level}}</li>
{{/for}}
</ul>
```

### Ancient Scroll Importing
```html
<!-- layouts/base.html -->
{{include "components/header.html"}}
<main>{{content}}</main>
{{include "components/footer.html"}}
```

### Sacred Comments
```html
{{! This comment is invisible to mortals but helpful for future wizards }}
<div class="magic-container">
  {{! TODO: Add more sparkles }}
  ✨ Magic happens here ✨
</div>
```

## 🏰 Template Architecture

```
templates/
├── layouts/           # Base scrolls
│   └── main.html     
├── components/        # Reusable incantations
│   ├── header.html
│   └── footer.html
└── pages/            # Sacred pages
    ├── home.html
    └── about.html
```

## 🔥 Advanced Wizardry

### Object Property Divination
```rust
let mut user = std::collections::HashMap::new();
user.insert("name", RuneSymbol::String("Gandalf".to_string()));
user.insert("level", RuneSymbol::Number(99));

scroll.set("user", RuneSymbol::Object(user));
```

```html
<h1>{{user.name}} the Level {{user.level}} Wizard</h1>
```

### Error Handling (When Magic Goes Wrong)
```rust
match engine.render("spell.html", &scroll) {
    Ok(html) => println!("✨ Magic successful!"),
    Err(rune_error) => eprintln!("🔥 Spell backfired: {}", rune_error),
}
```

## 🎭 Philosophy

Runic embraces the ancient developer truth: **templating is basically magic**. 

- You write mysterious symbols that transform into reality ✨
- Variables appear and disappear like spirits 👻  
- One missing bracket and your entire spell explodes 💥
- Senior developers guard template secrets like ancient druids 🧙‍♂️
- It works perfectly... until production 🔥

## 🛡️ Battle-Tested Features

- **Memory Safe**: No dragons (memory leaks) here
- **Lightning Fast**: Cached templates for speed of light rendering
- **XSS Protected**: Your templates are blessed against evil
- **Test Covered**: 22+ tests ensure the magic works
- **Zero Dependencies**: Pure Rust magic (except for error handling)

## 🌟 Why Choose Runic?

Because life's too short for boring template engines. Runic brings:

- **Joy**: Code that makes you smile
- **Power**: All the features you need
- **Safety**: Rust's guarantees + XSS protection
- **Simplicity**: Easy enough for apprentice developers
- **Humor**: Documentation that doesn't put you to sleep

## 🤝 Join the Magical Community

Contributions welcome! Whether you're a seasoned archmage or a young padawan, all pull requests receive a blessing from the Rust compiler.

## 📜 License

MIT - Because sharing magic makes the world a better place.

---

*"May your templates compile on the first try and your variables always resolve."*  
— Ancient Rust Proverb

**Made with ❤️ and ☕ by developers who believe coding should be fun.**