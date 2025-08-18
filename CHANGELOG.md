# Changelog

All notable changes to mystical-runic will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added in v0.1.3 (Latest - Zero Dependencies Edition)
- 🗑️ **Zero Dependencies** - Completely self-contained with no external dependencies
  - Removed `thiserror` - replaced with native Error trait implementation
  - Removed `tempfile` - custom temp directory utilities using std::env::temp_dir()
  - Pure Rust standard library implementation
  - Faster compilation, smaller binaries, reduced attack surface

### Added in v0.1.2 
- ✅ **Deep Dot Notation Support** - Complete support for unlimited depth object traversal
  - Variables: `{{user.profile.settings.theme.color}}`
  - Conditionals: `{{if user.account.subscription.active}}` 
  - Array indexing: `{{items.0.properties.name}}`
  - Recursive property resolution with proper error handling

### Planned for v0.2.0
- [ ] **Performance Improvements**
  - Parallel template processing for large files
  - Memory-mapped file loading for large templates
  - Template compilation to bytecode for faster execution

- [ ] **Advanced Features**
  - Template inheritance (extends/blocks)
  - Custom helper functions registration
  - Template macros and reusable components
  - Conditional operators (==, !=, <, >, <=, >=)
  - String manipulation filters (uppercase, lowercase, truncate)
  - Date/time formatting helpers

- [ ] **Developer Experience**
  - Better error messages with line/column numbers
  - Template debugging and profiling tools
  - IDE integration (Language Server Protocol)
  - Template syntax highlighting definitions
  - Live reload for development

- [ ] **Security Enhancements**
  - Content Security Policy (CSP) helper functions
  - Template sandboxing for untrusted templates
  - Resource usage limits (memory, execution time)
  - Audit logging for template operations

- [ ] **Ecosystem Integration**
  - Async template rendering support
  - Serde integration for automatic object serialization
  - Web framework integrations (Axum, Warp, Actix)
  - Static site generator features

## [0.1.0] - 2024-XX-XX

### Added
- 🎉 **Initial Release**: Core template engine with Mustache-inspired syntax
- 🔒 **Security First**: XSS-safe HTML escaping by default
- 📝 **Rich Syntax Support**:
  - Variable substitution with `{{variable}}`
  - Raw HTML output with `{{& variable}}`
  - Conditional rendering with `{{if condition}}...{{/if}}`
  - Loop iteration with `{{for item in items}}...{{/for}}`
  - Template includes with `{{include "template.html"}}`
  - Comments with `{{! comment text }}`
  - Object property access with `{{object.property}}`
- ⚡ **Performance Features**:
  - Template caching for improved performance
  - Efficient string processing and memory management
- 🧪 **Comprehensive Testing**:
  - 100% test coverage across all modules
  - Extensive security testing suite
  - Integration tests for complex scenarios
  - Unit tests for individual components
  - Performance and stress testing
- 📚 **Developer Experience**:
  - Clean, intuitive Rust API
  - Comprehensive documentation with examples
  - Magical themed naming conventions (RuneEngine, RuneScroll, RuneSymbol)
  - Detailed error messages
- 🛡️ **Security Features**:
  - XSS prevention through automatic HTML escaping
  - Path traversal protection for template includes
  - Template injection prevention
  - Unicode security handling
  - Memory exhaustion protection
- 🌐 **Standards Compliance**:
  - Full Unicode support
  - MIT license for broad adoption
  - Rust 2021 edition compatibility
  - Zero-dependency core (except for error handling)

### Technical Details
- **Supported Value Types**: String, Number (i64), Boolean, Array, Object (HashMap)
- **Template Caching**: Automatic caching of loaded templates for performance
- **Error Handling**: Comprehensive error types with descriptive messages
- **Memory Safety**: Rust's ownership system prevents memory leaks and crashes
- **Unicode Support**: Proper handling of international characters and emoji
- **Testing Coverage**: 
  - 22+ integration tests
  - 15+ unit tests  
  - 20+ security tests
  - Edge case and stress testing

### Architecture
- **Core Modules**:
  - `engine.rs`: Template processing and rendering engine
  - `context.rs`: Variable storage and retrieval system
  - `value.rs`: Type system for template values
  - `error.rs`: Error handling and reporting
  - `utils.rs`: HTML escaping and utility functions
- **Public API**: Both mystical names (Rune*) and conventional names (Template*)
- **Template Format**: Mustache-inspired with extensions for includes and comments

### Performance Characteristics
- **Template Loading**: O(1) for cached templates, O(n) for file I/O
- **Variable Substitution**: O(1) lookup in hash maps
- **Template Rendering**: Linear time complexity relative to template size
- **Memory Usage**: Minimal allocations through string reuse and caching

### Security Model
- **Default Secure**: All variables are HTML-escaped by default
- **Explicit Unsafe**: Raw HTML requires `{{& variable}}` syntax
- **File System Protection**: Templates limited to configured directory
- **Input Validation**: All user input properly sanitized
- **No Code Execution**: Templates cannot execute arbitrary code

---

## Future Roadmap (v0.3.0 and beyond)

### Advanced Template Features
- **Template Inheritance**: `{{extends "base.html"}}` and `{{block content}}` syntax
- **Partial Templates**: More flexible include system with parameters
- **Template Macros**: Reusable template components with parameters
- **Conditional Operators**: `{{if user.age > 18}}`, `{{if status == "active"}}`
- **String Filters**: `{{name | uppercase | truncate(10)}}`
- **Mathematical Operations**: `{{price * quantity}}`, `{{total + tax}}`

### Performance Optimizations
- **Template Compilation**: Pre-compile templates to bytecode
- **Streaming Rendering**: Render large templates incrementally
- **Parallel Processing**: Process multiple templates concurrently
- **Memory Mapping**: Use memory-mapped files for large templates
- **SIMD Optimizations**: Vectorized string processing where possible

### Developer Tools
- **Syntax Highlighting**: VS Code, Vim, Emacs extensions
- **Language Server**: IDE integration with autocomplete and error checking
- **Template Debugger**: Step through template rendering process
- **Performance Profiler**: Identify slow template operations
- **Hot Reload**: Automatic template reloading during development

### Ecosystem Integration
- **Web Frameworks**: First-class support for Axum, Warp, Actix-web
- **Static Site Generators**: Build-time template processing
- **Async Support**: Non-blocking template rendering
- **WASM Compatibility**: Run templates in WebAssembly environments
- **CLI Tools**: Command-line template processing utilities

### Advanced Security
- **Content Security Policy**: Automatic CSP header generation
- **Template Sandboxing**: Isolated execution environments
- **Resource Limits**: Prevent DoS through resource exhaustion
- **Audit Logging**: Security event logging and monitoring
- **OWASP Compliance**: Follow latest web security guidelines

### Documentation and Community
- **Interactive Playground**: Online template testing environment
- **Video Tutorials**: Comprehensive learning materials
- **Community Templates**: Shared template library
- **Migration Guides**: Easy migration from other template engines
- **Best Practices**: Security and performance guidelines

---

*"The best templating engine is one that gets out of your way and lets you focus on building amazing things."*