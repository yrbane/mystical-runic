# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Mystical-Runic is a zero-dependency templating engine for Rust that provides Mustache-inspired syntax with advanced features like deep dot notation, filters, macros, template inheritance, nested loops, recursive includes, enterprise-grade security, comprehensive developer experience tools, and ecosystem integration (async, web frameworks, WASM, CLI tools). The project follows strict Test-Driven Development (TDD) practices with 204+ comprehensive tests.

## Key Commands

### Development and Testing
- `cargo test` - Run all tests (comprehensive test suite with 204+ tests)
- `cargo test basic_v040_tests` - Run v0.4.0 developer experience tests
- `cargo test integration_tests` - Run integration tests
- `cargo test unit_tests` - Run unit tests  
- `cargo test security_tests` - Run security-focused tests (includes path traversal protection)
- `cargo test performance_tests` - Run performance tests
- `cargo test v0_2_0_features_tests` - Run tests for v0.2.0 features
- `cargo test v0_3_0_features_tests` - Run tests for v0.3.0+ features (i18n, pluralization)
- `cargo test v0_5_0_ecosystem_integration_tests` - Run tests for v0.5.0 ecosystem integration
- `cargo test --features "async,web-frameworks,wasm,cli" v0_5_0_ecosystem_integration_tests` - Run v0.5.0 with all features
- `cargo test -- --nocapture` - Run tests with output visible
- `cargo check` - Fast compilation check without building binaries
- `cargo build` - Build the project
- `cargo run --bin performance_demo` - Run performance benchmarks

### Real-World Demo Application
- `cd examples/real_world_demo && ./run_demo.sh` - Run complete feature demonstration
- `cd examples/real_world_demo && cargo run` - Manual demo execution
- Generates `output_demo.html` - Complete working website (10KB+) showcasing all features

### Single Test Execution
- `cargo test test_basic_rendering -- --nocapture` - Run specific test with output
- `RUST_BACKTRACE=1 cargo test test_name -- --nocapture` - Run with full backtrace

## Architecture Overview

### Core Components

The templating engine is built around four main components:

1. **TemplateEngine/RuneEngine** (`src/engine.rs`) - Main engine with template processing, caching, macro system, and performance features
2. **TemplateContext/RuneScroll** (`src/context.rs`) - Variable storage and context management
3. **TemplateValue/RuneSymbol** (`src/value.rs`) - Type system supporting strings, numbers, booleans, arrays, and nested objects
4. **Error handling** (`src/error.rs`) - Comprehensive error types for parsing, rendering, and template issues

### Advanced Features

#### v0.4.0 NEW Developer Experience Features
- **Enhanced Error Messages**: Precise line/column error reporting with `ParseWithLocation`
- **Template Debugging**: Complete debugging system with variable tracking via `render_string_with_debug()`
- **Hot Reload**: Development-time automatic template reloading with `enable_hot_reload()`
- **Debug Mode**: Variable access tracking and execution step analysis
- **Performance Metrics**: Built-in performance tracking with execution time measurement
- **Intelligent Suggestions**: Smart template and variable name suggestions for common typos

#### v0.3.4 Advanced Security Features
- **Nested Loops**: Complete support for nested `{{for}}` directives with stack-based parsing
- **Recursive Includes**: Templates can include other templates recursively (unlimited depth)
- **Path Traversal Protection**: Enterprise-grade security preventing `../` and absolute path attacks
- **Security Error Type**: New `TemplateError::Security` for clear security violation reporting

#### Core Features  
- **Deep Dot Notation**: Unlimited depth object traversal (e.g., `{{game.character.stats.level}}`)
- **Template Inheritance**: Layout system with `{{extend}}` and `{{block}}` directives
- **Macro System**: Reusable template components with parameters and context resolution
- **Filter System**: Built-in filters like `upper`, `lower`, `truncate`, `currency`, `markdown`, plus custom filter registration
- **Internationalization**: Multi-language support with `{{t "key"}}` and variable interpolation
- **Smart Pluralization**: Automatic plural forms with `{{plural count "item" "items"}}`
- **Math Filters**: Advanced mathematical operations with chaining (`add`, `multiply`, `divide`, `round`, `percentage`)
- **XSS Security**: HTML escaping by default with raw output option `{{& safe_html}}`
- **Performance**: Template caching, bytecode compilation, parallel processing, memory mapping
- **Dual Naming**: Professional (`TemplateEngine`) or mystical (`RuneEngine`) API styles

### Module Structure

- `src/lib.rs` - Main library exports with both technical and "mystical" aliases
- `src/bytecode.rs` - Bytecode compilation and execution for performance
- `src/layouts.rs` - Template inheritance and layout processing
- `src/utils.rs` - Utility functions including HTML escaping

## Template Syntax

### Variables
- `{{name}}` - Escaped variable output
- `{{& html}}` - Raw/unescaped HTML output
- `{{user.profile.name}}` - Deep object property access

### Control Flow
- `{{if condition}}...{{/if}}` - Conditionals with comparison operators
- `{{for item in items}}...{{/for}}` - Loops over arrays
- `{{for category in categories}}{{for product in category.items}}...{{/for}}{{/for}}` - **NEW v0.3.4**: Nested loops

### Advanced Features
- `{{include "template.html"}}` - Template includes (recursive support in v0.3.4)
- `{{name|upper|truncate:10}}` - Filter chains
- `{{price|multiply:1.2|add:tax|currency}}` - **NEW**: Advanced math filter chaining  
- `{{macro button(text, class)}}...{{/macro}}` - Macro definitions
- `{{button("Click", "btn-primary")}}` - Macro calls
- `{{t "welcome" name=user.name}}` - **NEW**: Internationalization with variables
- `{{plural count "item" "items"}}` - **NEW**: Smart pluralization

## Testing Philosophy

Only SOLID code !

This project follows strict **Test-Driven Development (TDD)**:

1. **🔴 RED**: Write failing test first
2. **🟢 GREEN**: Write minimal code to pass test
3. **🔵 REFACTOR**: Improve code while keeping tests green

When adding new features:
1. Always write tests first before implementation
2. Use the existing test structure in `tests/` directory
3. Ensure 100% test coverage for new features
4. Run full test suite before committing

## Performance Features

The engine includes several performance optimizations:
- Template caching to avoid repeated parsing
- Bytecode compilation for frequently-used templates
- Parallel rendering capabilities for multiple templates
- Memory mapping support for large templates
- Efficient nested property traversal

## Security Considerations

- **XSS Protection**: All variable output is HTML-escaped by default
- **Path Traversal Protection**: NEW v0.3.4 - Enterprise-grade protection against:
  - `../` and `..\` path traversal attempts
  - Absolute paths like `/etc/passwd` and `C:\Windows\System32`
  - Drive letter paths on Windows
  - Multi-layer validation with canonicalization
- **Template Sandboxing**: Templates restricted to designated template directory
- **Security Error Type**: `TemplateError::Security` for clear security violation reporting
- **Template Injection Prevention**: Comprehensive input validation
- **Comprehensive Security Tests**: Full test suite validates all protection mechanisms