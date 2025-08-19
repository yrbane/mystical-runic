//! # 🔮 Runic - Ancient Symbols for Modern Web Magic
//!
//! *"In the beginning was the Word, and the Word was `{{mustache}}`..."*
//! 
//! **Rust Compatibility**: This crate requires Rust 1.74.0+ and targets Rust 2021 edition
//! with future Rust 2024 readiness.
//! 
//! Welcome, brave developer, to the mystical realm of **Runic** - where ancient Nordic symbols
//! meet modern HTML templating in a beautiful dance of curly braces and digital sorcery!
//!
//! **NEW in v0.4.1**: Full IDE Integration with Language Server Protocol support, auto-completion,
//! syntax highlighting, real-time diagnostics, hover information, and go-to-definition features!
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
//! ## 🧙‍♂️ Example Usage - Choose Your Style
//!
//! ### Professional Style
//! ```rust
//! use mystical_runic::{TemplateEngine, TemplateContext, TemplateValue};
//!
//! let mut engine = TemplateEngine::new("templates");
//! let mut context = TemplateContext::new();
//! 
//! context.set("user", TemplateValue::String("Developer".to_string()));
//! context.set("task", TemplateValue::String("Build Features".to_string()));
//! 
//! let result = engine.render_string(
//!     "Hello {{user}}! Your mission: {{task}}", 
//!     &context
//! ).unwrap();
//! ```
//!
//! ### Mystical Style ✨
//! ```rust
//! use mystical_runic::{RuneEngine, RuneScroll, RuneSymbol};
//!
//! // Summon the ancient engine from the template realm
//! let mut engine = RuneEngine::new("sacred_scrolls");
//! let mut scroll = RuneScroll::new();
//! 
//! // Inscribe your desires upon the scroll
//! scroll.set("hero", RuneSymbol::String("Rust Developer".to_string()));
//! scroll.set("quest", RuneSymbol::String("Debug Production Issues".to_string()));
//! 
//! // Speak the incantation and witness the transformation
//! let result = engine.render_string(
//!     "Behold! {{hero}} embarks upon {{quest}}! 🗡️", 
//!     &scroll
//! ).unwrap();
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
mod debug;
mod suggestions;
mod lsp;

// 🏢 Conventional names for professional development environments
pub use error::{TemplateError, TemplateResult};
pub use engine::TemplateEngine;
pub use context::TemplateContext;
pub use value::TemplateValue;
pub use engine::FilterFunction;
pub use engine::HelperFunction;
pub use debug::{DebugInfo, DebugRenderResult, ExecutionStep, PerformanceMetrics};
pub use lsp::{LspParseResult, TemplateBlock, CompletionItem, SyntaxToken, Diagnostic, HoverInfo, DefinitionInfo};

// 🔮 Mystical aliases for the enlightened practitioners of ancient coding arts
pub use error::{TemplateError as RuneError, TemplateResult as RuneResult};
pub use engine::TemplateEngine as RuneEngine;
pub use context::TemplateContext as RuneScroll;
pub use value::TemplateValue as RuneSymbol;
pub use engine::FilterFunction as MysticFilter;
pub use engine::HelperFunction as AncientHelper;
pub use debug::{DebugInfo as RuneTrace, DebugRenderResult as RuneDivination, ExecutionStep as RuneStep, PerformanceMetrics as RuneMetrics};
pub use lsp::{LspParseResult as RunicLore, TemplateBlock as RunicBlock, CompletionItem as RunicCompletion, SyntaxToken as RunicToken, Diagnostic as RunicDiagnostic, HoverInfo as RunicWisdom, DefinitionInfo as RunicOrigin};