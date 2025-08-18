//! # 🔮 Runic - Ancient Symbols for Modern Web Magic
//!
//! *"In the beginning was the Word, and the Word was `{{mustache}}`..."*
//! 
//! Welcome, brave developer, to the mystical realm of **Runic** - where ancient Nordic symbols
//! meet modern HTML templating in a beautiful dance of curly braces and digital sorcery!
//!
//! ## ⚡ The Sacred Incantations
//!
//! - **Whisper Variables**: `{{name}}` - Speak a name and it shall manifest (safely escaped from evil XSS spirits)
//! - **Summon Raw Power**: `{{& html}}` - Unleash unescaped HTML with great responsibility and greater danger
//! - **Divine Conditionals**: `{{if chosen_one}}...{{/if}}` - The HTML appears only for the worthy
//! - **Mystical Loops**: `{{for spell in grimoire}}...{{/for}}` - Repeat incantations until magic happens
//! - **Ancient Includes**: `{{include "scrolls/wisdom.html"}}` - Import wisdom from other sacred texts
//! - **Silent Whispers**: `{{! This is but a comment, invisible to mortals }}` - Notes for future wizards
//! - **Object Divination**: `{{user.power_level}}` - Peer into the properties of mystical entities
//!
//! ## 🧙‍♂️ Example Spell Casting
//!
//! ```rust
//! use mystical_runic::{RuneEngine, RuneScroll, RuneSymbol};
//!
//! // Summon the ancient engine from the template realm
//! let mut engine = RuneEngine::new("sacred_scrolls");
//! let mut scroll = RuneScroll::new();
//! 
//! // Inscribe your desires upon the scroll
//! scroll.set_string("hero", "Rust Developer");
//! scroll.set_string("quest", "Debug Production Issues");
//! 
//! // Speak the incantation and witness the transformation
//! let result = engine.render_string(
//!     "Behold! {{hero}} embarks upon {{quest}}! 🗡️", 
//!     &scroll
//! ).unwrap();
//! 
//! assert_eq!(result, "Behold! Rust Developer embarks upon Debug Production Issues! 🗡️");
//! ```
//!
//! ## 🌟 Why "Runic"?
//!
//! Because templating is basically ancient magic:
//! - You write mysterious symbols (`{{}}`) that transform into reality
//! - Variables appear and disappear like spirits
//! - One wrong bracket and your entire spell explodes
//! - Senior developers guard the template secrets like ancient druids
//! - Documentation is written in a language only the initiated understand
//! - And just like real magic, it works perfectly until production 🔥
//!
//! *"May your templates be bug-free and your variables always defined."*  
//! — Ancient DevOps Proverb

mod error;
mod engine;
mod context;
mod value;
mod utils;
mod bytecode;
mod layouts;

// Ancient names for backwards compatibility with mundane mortals
pub use error::{TemplateError, TemplateResult};
pub use engine::TemplateEngine;
pub use context::TemplateContext;
pub use value::TemplateValue;

// Mystical names for the enlightened practitioners
pub use error::{TemplateError as RuneError, TemplateResult as RuneResult};
pub use engine::TemplateEngine as RuneEngine;
pub use context::TemplateContext as RuneScroll;
pub use value::TemplateValue as RuneSymbol;