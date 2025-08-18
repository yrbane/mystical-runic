# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Mystical-Runic is a zero-dependency templating engine for Rust that provides Mustache-inspired syntax with advanced features like deep dot notation, filters, macros, and template inheritance. The project follows strict Test-Driven Development (TDD) practices.

## Key Commands

### Development and Testing
- `cargo test` - Run all tests (comprehensive test suite with 85+ tests)
- `cargo test integration_tests` - Run integration tests
- `cargo test unit_tests` - Run unit tests  
- `cargo test security_tests` - Run security-focused tests
- `cargo test performance_tests` - Run performance tests
- `cargo test v0_2_0_features_tests` - Run tests for v0.2.0 features
- `cargo test -- --nocapture` - Run tests with output visible
- `cargo check` - Fast compilation check without building binaries
- `cargo build` - Build the project
- `cargo run --bin performance_demo` - Run performance benchmarks

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

- **Deep Dot Notation**: Unlimited depth object traversal (e.g., `{{game.character.stats.level}}`)
- **Template Inheritance**: Layout system with `{{extend}}` and `{{block}}` directives
- **Macro System**: Reusable template components with parameters and context resolution
- **Filter System**: Built-in filters like `upper`, `lower`, `truncate`, `currency`, `markdown`, `highlight`
- **Security**: XSS-safe HTML escaping by default, path traversal protection
- **Performance**: Template caching, bytecode compilation, parallel processing, memory mapping

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

### Advanced Features
- `{{include "template.html"}}` - Template includes
- `{{name|upper|truncate:10}}` - Filter chains
- `{{macro button(text, class)}}...{{/macro}}` - Macro definitions
- `{{button("Click", "btn-primary")}}` - Macro calls

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

- All variable output is HTML-escaped by default
- Path traversal protection for template includes
- Template injection prevention
- Comprehensive security test suite validates XSS protection