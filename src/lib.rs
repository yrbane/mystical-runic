//! # 🔮 Mystical-Runic - Professional Template Engine for Rust
//!
//! [![Crates.io](https://img.shields.io/crates/v/mystical-runic.svg)](https://crates.io/crates/mystical-runic)
//! [![Documentation](https://docs.rs/mystical-runic/badge.svg)](https://docs.rs/mystical-runic)
//! [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
//! 
//! **Rust Compatibility**: Requires Rust 1.74.0+ | Edition 2021 | Future Rust 2024 Ready
//! 
//! **Mystical-Runic** is a high-performance, zero-dependency templating engine for Rust that combines
//! Mustache-inspired syntax with advanced features like template inheritance, macros, filters,
//! enterprise-grade security, comprehensive developer tools, and ecosystem integration.
//!
//! ## 🌟 Key Features
//!
//! - **Zero Dependencies Core**: No external dependencies for core functionality
//! - **Enterprise Security**: XSS protection, path traversal prevention, template injection security
//! - **High Performance**: Bytecode compilation, caching, parallel processing, memory mapping
//! - **Developer Experience**: Hot reload, debugging, IDE integration, intelligent error messages
//! - **Ecosystem Integration**: Async support, web framework integration, WASM compatibility, CLI tools
//! - **Comprehensive Testing**: 204+ tests with 100% coverage following strict TDD methodology
//! - **Professional & Mystical APIs**: Choose your preferred naming style
//!
//! ## 📋 Template Syntax Reference
//!
//! | Feature | Syntax | Description |
//! |---------|--------|-------------|
//! | **Variables** | `{{name}}` | HTML-escaped variable output |
//! | **Raw HTML** | `{{& html}}` | Unescaped HTML output (use carefully) |
//! | **Conditionals** | `{{if condition}}...{{/if}}` | Conditional rendering with comparison operators |
//! | **Loops** | `{{for item in items}}...{{/for}}` | Iterate over arrays and nested structures |
//! | **Deep Access** | `{{user.profile.name}}` | Unlimited depth object property access |
//! | **Includes** | `{{include "template.html"}}` | Template composition and reuse |
//! | **Comments** | `{{! comment }}` | Template comments (not rendered) |
//! | **Filters** | `{{value\|upper\|truncate:10}}` | Transform output with filter chains |
//! | **Macros** | `{{macro name(params)}}...{{/macro}}` | Reusable template components |
//! | **Inheritance** | `{{extend "layout.html"}}` | Template inheritance system |
//! | **Blocks** | `{{block content}}...{{/block}}` | Replaceable content blocks |
//! | **I18n** | `{{t "key" name=user}}` | Internationalization with variables |
//! | **Pluralization** | `{{plural count "item" "items"}}` | Smart plural forms |
//!
//! ## 📖 Quick Start Guide
//!
//! Add to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! mystical-runic = "0.5.0"
//!
//! # Optional features
//! mystical-runic = { version = "0.5.0", features = ["async", "web-frameworks", "wasm", "cli"] }
//! ```
//!
//! ## 🧙‍♂️ Usage Examples - Choose Your Style
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
//! ## 🚀 Performance & Security
//!
//! **Performance Features:**
//! - Template caching with smart invalidation
//! - Bytecode compilation for frequently-used templates  
//! - Parallel template processing capabilities
//! - Memory-mapped file loading for large templates
//! - Optimized nested property traversal
//!
//! **Security Features:**
//! - HTML escaping by default (XSS prevention)
//! - Path traversal protection (`../` and absolute path blocking)
//! - Template injection prevention
//! - Memory exhaustion protection
//! - Input validation and sanitization
//!
//! ## 🛠️ Developer Experience
//!
//! - **Hot Reload**: Automatic template recompilation during development
//! - **Debug Mode**: Variable tracking and execution analysis
//! - **IDE Integration**: LSP support with auto-completion and diagnostics
//! - **Smart Suggestions**: Intelligent template and variable suggestions
//! - **Precise Errors**: Line/column error reporting with context
//! - **Performance Metrics**: Built-in execution time measurement

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
mod async_engine;
mod web_frameworks;
mod wasm_support;
mod cli;
mod ecosystem;

// 🏢 Conventional names for professional development environments
pub use error::{TemplateError, TemplateResult};
pub use engine::TemplateEngine;
pub use context::TemplateContext;
pub use value::TemplateValue;
pub use engine::FilterFunction;
pub use engine::HelperFunction;
pub use debug::{DebugInfo, DebugRenderResult, ExecutionStep, PerformanceMetrics};
pub use lsp::{LspParseResult, TemplateBlock, CompletionItem, SyntaxToken, Diagnostic, HoverInfo, DefinitionInfo};

// 🚀 v0.5.0 Ecosystem Integration exports
#[cfg(feature = "async")]
pub use async_engine::AsyncTemplateEngine;

#[cfg(feature = "axum-integration")]
pub use web_frameworks::axum_integration::{AxumTemplateEngine, TemplateResponseError};

#[cfg(feature = "warp-integration")]  
pub use web_frameworks::warp_integration::WarpTemplateEngine;

#[cfg(feature = "actix-integration")]
pub use web_frameworks::actix_integration::ActixTemplateEngine;

#[cfg(feature = "wasm")]
pub use wasm_support::{WasmTemplateEngine, WasmRuneEngine};

#[cfg(feature = "cli")]
pub use cli::{Cli, Commands, CliConfig, TemplateWatcher, process_template, process_files, batch_process, load_config};

pub use ecosystem::{EcosystemCompatibility, EcosystemTemplateEngine};

// 🔮 Mystical aliases for the enlightened practitioners of ancient coding arts
pub use error::{TemplateError as RuneError, TemplateResult as RuneResult};
pub use engine::TemplateEngine as RuneEngine;
pub use context::TemplateContext as RuneScroll;
pub use value::TemplateValue as RuneSymbol;
pub use engine::FilterFunction as MysticFilter;
pub use engine::HelperFunction as AncientHelper;
pub use debug::{DebugInfo as RuneTrace, DebugRenderResult as RuneDivination, ExecutionStep as RuneStep, PerformanceMetrics as RuneMetrics};
pub use lsp::{LspParseResult as RunicLore, TemplateBlock as RunicBlock, CompletionItem as RunicCompletion, SyntaxToken as RunicToken, Diagnostic as RunicDiagnostic, HoverInfo as RunicWisdom, DefinitionInfo as RunicOrigin};

// 🔮 v0.5.0 Mystical ecosystem aliases
#[cfg(feature = "async")]
pub use async_engine::AsyncTemplateEngine as AsyncRuneEngine;

#[cfg(feature = "axum-integration")]
pub use web_frameworks::axum_integration::{AxumTemplateEngine as AxumRuneEngine, TemplateResponseError as RuneResponseError};

#[cfg(feature = "warp-integration")]
pub use web_frameworks::warp_integration::WarpTemplateEngine as WarpRuneEngine;

#[cfg(feature = "actix-integration")]
pub use web_frameworks::actix_integration::ActixTemplateEngine as ActixRuneEngine;

#[cfg(feature = "wasm")]
pub use wasm_support::{WasmTemplateEngine as WasmRuneEngineTrait, WasmRuneEngine as BrowserRuneEngine};

#[cfg(feature = "cli")]
pub use cli::{Cli as RunicCli, Commands as RunicCommands, CliConfig as RunicConfig, TemplateWatcher as RuneWatcher};

pub use ecosystem::{EcosystemCompatibility as RunicCompatibility, EcosystemTemplateEngine as EcosystemRuneEngine};