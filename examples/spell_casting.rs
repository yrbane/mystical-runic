//! 🔮 Example: Advanced Spell Casting with Runic
//! 
//! This example demonstrates the mystical powers of the Runic templating engine
//! by creating a magical character sheet for a fantasy RPG.

use mystical_runic::{RuneEngine, RuneScroll, RuneSymbol};
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧙‍♂️ Welcome to the Runic Magic Demonstration!");
    println!("═══════════════════════════════════════════════\n");

    // 🏰 Summon the ancient engine
    let mut engine = RuneEngine::new(".");
    
    // 📜 Create a mystical scroll for our variables
    let mut scroll = RuneScroll::new();
    
    // 🧙‍♂️ Create a magical character
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
        create_spell("Magic Missile", 25, "✨"),
    ];
    scroll.set("spells", RuneSymbol::Array(spells));
    
    // 🎭 Set additional magical variables
    scroll.set_string("title", "🏰 Character Sheet");
    scroll.set_string("guild", "Fellowship of the Ring");
    scroll.set_bool("is_legendary", true);
    scroll.set_number("experience", 1500000);
    
    // ✨ The magical template spell
    let character_sheet = r#"
╔══════════════════════════════════════════════════════════════╗
║                      {{title}}                       ║
╚══════════════════════════════════════════════════════════════╝

🧙‍♂️ Name: {{character.name}}
🎓 Class: {{character.class}} (Level {{character.level}})
🏛️ Guild: {{guild}}
💫 Experience: {{experience}} XP
🔮 Mana: {{character.mana}}

{{if is_legendary}}
🌟 ★ LEGENDARY CHARACTER ★ 🌟
{{/if}}

{{if character.has_staff}}
🪄 Equipment: Magical Staff of Power
{{/if}}

⚡ KNOWN SPELLS:
{{for spell in spells}}
  {{spell.icon}} {{spell.name}} - Power: {{spell.damage}}
{{/for}}

{{! This is a secret comment only wizards can see }}
═══════════════════════════════════════════════════════════════
🎮 Character ready for adventure! 
"#;

    // 🔥 Cast the spell and manifest the character sheet!
    let result = engine.render_string(character_sheet, &scroll)?;
    
    println!("{}", result);
    
    // 🎯 Demonstrate error handling
    println!("\n🔥 Testing magical error handling...");
    match engine.render_string("{{unclosed_bracket", &scroll) {
        Ok(_) => println!("❌ This shouldn't happen!"),
        Err(e) => println!("✅ Spell safely failed: {}", e),
    }
    
    // 🌟 Show raw HTML magic (dangerous!)
    scroll.set_string("dangerous_html", "<strong>Bold Text</strong>");
    let html_result = engine.render_string(
        "Safe: {{dangerous_html}}\nDangerous: {{& dangerous_html}}", 
        &scroll
    )?;
    
    println!("\n🛡️ XSS Protection Demo:");
    println!("{}", html_result);
    
    println!("\n🎉 Magic demonstration complete!");
    println!("✨ The ancient runic symbols have served you well! ✨");
    
    Ok(())
}

/// Helper function to create a spell object
fn create_spell(name: &str, damage: i64, icon: &str) -> RuneSymbol {
    let mut spell = HashMap::new();
    spell.insert("name".to_string(), RuneSymbol::String(name.to_string()));
    spell.insert("damage".to_string(), RuneSymbol::Number(damage));
    spell.insert("icon".to_string(), RuneSymbol::String(icon.to_string()));
    RuneSymbol::Object(spell)
}