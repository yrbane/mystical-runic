# Changelog

All notable changes to mystical-runic will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.2] - 2025-08-19 - Modern Rust Edition

### 🦀 Added - Modern Rust Compatibility
- **Minimum Supported Rust Version (MSRV)**: Explicitly set to 1.74.0+
- **Future Rust 2024 Ready**: Code structured for seamless Rust 2024 migration when stable
- **Modern Rust Patterns**: Updated code to use latest Rust idioms and best practices
- **Comprehensive Testing**: 158+ tests including modern Rust compatibility suite

### 🔧 Improved - Code Quality  
- **Cleaner Warnings**: Fixed unused imports and variables for cleaner compilation
- **Better Documentation**: Enhanced inline docs with Rust compatibility information
- **Optimized Patterns**: Improved code structure following modern Rust guidelines

### 📚 Enhanced - Documentation
- **Rust Compatibility Section**: Detailed MSRV and edition information in README
- **Future-Proof Design**: Documentation of Rust 2024 readiness and migration path
- **Test Coverage**: Updated test count (152+ → 158+) reflecting new compatibility tests

### 🔄 Maintained - Backwards Compatibility
- **Zero Breaking Changes**: All existing APIs work unchanged
- **Same Performance**: No performance regressions from modernization
- **Identical Behavior**: All existing functionality preserved

## [0.3.1] - 2025-08-19 - Dual Style Edition

### 🎭 Added - Dual Naming System
- **Professional Style**: Conventional naming (`TemplateEngine`, `TemplateContext`, `TemplateValue`)
- **Mystical Style**: Themed aliases (`RuneEngine`, `RuneScroll`, `RuneSymbol`)
- **Complete Interoperability**: Both styles are 100% compatible and interchangeable
- **Developer Choice**: Use the naming style that fits your project's personality
- **Type Aliases**: `FilterFunction`/`MysticFilter`, `HelperFunction`/`AncientHelper`

### 🧪 Enhanced - Testing Coverage
- **152+ Tests Total**: Added 8 new interoperability tests for dual naming system
- **Style Compatibility**: Complete test coverage for both naming conventions
- **Zero Breaking Changes**: All existing code works unchanged

### 📚 Enhanced - Documentation
- **Dual Style Examples**: README now showcases both professional and mystical styles
- **Enhanced lib.rs**: Complete documentation with both naming conventions
- **Usage Flexibility**: Developers can choose or mix naming styles freely

## [0.3.0] - 2025-08-19 - The Global Sorcery Edition

### 🌐 Added - Internationalization (i18n) System
- **Translation Support**: `{{t "key"}}` syntax for multi-language templates
- **Locale Management**: `engine.set_locale("en")` for dynamic language switching  
- **Translation Storage**: `engine.set_translations(locale, translations)` for organizing translations
- **Variable Integration**: Translations can contain template variables (e.g., `"Hello {{name}}!"`)
- **Fallback Handling**: Missing translations fallback to the key itself
- **Context Processing**: Translation strings are processed as templates with full variable support

### 📝 Added - Smart Pluralization System
- **Plural Forms**: `{{plural count "item" "items"}}` for automatic singular/plural selection
- **Zero Handling**: Proper handling of zero counts (uses plural form)
- **Number Integration**: Works seamlessly with numeric template values
- **Template Integration**: Can be combined with other template features

### 🎨 Added - Custom Filter Registration API
- **Filter Registration**: `engine.register_filter(name, closure)` for user-defined filters
- **Argument Support**: Custom filters receive input string and argument array
- **Error Handling**: Proper error propagation from custom filter functions
- **Filter Chaining**: Custom filters integrate seamlessly with built-in filter chains
- **Type Safety**: Rust closures ensure compile-time safety for custom filters

### 🔢 Added - Advanced Math Filters
- **Addition Filter**: `{{value|add:10}}` for numeric addition (existing, enhanced)
- **Multiplication Filter**: `{{value|multiply:2}}` for numeric multiplication (existing, enhanced)
- **Division Filter**: `{{value|divide:100}}` for numeric division with zero-check
- **Percentage Filter**: `{{value|percentage}}` for appending % symbol
- **Rounding Filter**: `{{value|round:2}}` for decimal precision control
- **Math Chaining**: Complex calculations like `{{price|multiply:tax|divide:100|add:price|round:2}}`

### 🧪 Added - Comprehensive Testing
- **135+ Tests Total**: Expanded from 130 to 135+ tests across all modules
- **v0.3.0 Feature Tests**: Complete test coverage for i18n, pluralization, custom filters, and math
- **TDD Implementation**: All features developed using strict Test-Driven Development methodology
- **Integration Testing**: Real-world scenarios for internationalized applications

### 🔧 Enhanced - Filter System
- **Custom Filter Integration**: Seamless integration of user-defined filters with built-in ones
- **Enhanced Error Handling**: Better error messages for filter failures
- **Performance Optimization**: Efficient filter lookup and execution

### 📚 Enhanced - Documentation
- **i18n Guide**: Complete examples for internationalization implementation
- **Custom Filters Tutorial**: Step-by-step guide for creating domain-specific filters
- **Math Operations Reference**: Documentation for all mathematical filters
- **Pluralization Guide**: Best practices for multilingual plural forms

### 🌐 Technical
- **Zero Dependencies**: Continues pure Rust implementation
- **Thread Safety**: All new features are fully thread-safe
- **Unicode Support**: Full Unicode compatibility across internationalization features
- **Backwards Compatibility**: All existing v0.2.0 APIs remain unchanged

## [0.2.0] - 2024-08-18 - The Advanced Sorcery Edition

### 🏰 Added - Template Inheritance System
- **Template Layouts**: `{{extends "base.html"}}` for template inheritance
- **Content Blocks**: `{{block content}}...{{/block}}` for defining template regions
- **Nested Inheritance**: Support for multi-level template hierarchies (base → admin → admin_users)
- **Super Blocks**: `{{super}}` directive to include parent template content
- **Layout Processing**: Complete layout resolution and block merging system

### 🔧 Added - Powerful Filters System
- **Built-in Filters**: 
  - `upper` - Convert text to uppercase
  - `lower` - Convert text to lowercase  
  - `currency` - Format numbers as currency ($12.99)
  - `truncate:N` - Truncate text to N characters with "..."
  - `date:"format"` - Date formatting (currently passes through)
- **Filter Chaining**: `{{name|lower|capitalize}}` for combining multiple transformations
- **Filter Arguments**: Support for parameterized filters like `truncate:50`

### 📦 Added - Reusable Macros System
- **Macro Definitions**: `{{macro name(params)}}...{{/macro}}` for reusable components
- **Macro Parameters**: Support for named parameters with default values
- **Macro Invocation**: `{{macro_name("arg1", param="value")}}` for calling macros
- **Context Isolation**: Proper variable scoping within macro execution
- **Variable Access**: Macros can access and modify template variables

### ⚡ Added - Enhanced Performance Features
- **Bytecode Compilation**: Pre-compile frequently used templates for faster execution
- **Parallel Processing**: Support for concurrent template rendering
- **Memory Mapping**: Efficient loading of large template files
- **Advanced Caching**: Improved template caching mechanisms

### 🧪 Added - Comprehensive Testing
- **130+ Tests Total**: Expanded from 127 to 130+ tests across all modules
- **v0.2.0 Feature Tests**: Complete test coverage for new template inheritance, filters, and macros
- **Performance Tests**: Benchmarking for new performance features
- **Integration Tests**: Real-world scenario testing for complex template hierarchies

### 📚 Added - Enhanced Documentation
- **Template Inheritance Guide**: Complete examples of layout systems
- **Filters Reference**: Documentation for all built-in filters with examples
- **Macros Tutorial**: Comprehensive guide to creating reusable template components
- **Updated README**: Showcases all v0.2.0 features with practical examples
- **API Documentation**: Enhanced inline documentation

### 🔧 Fixed
- **Nested Block Rendering**: Fixed boundary calculation in template block replacement
- **Function Call Error Handling**: Proper error messages for unsupported functions like `range()`
- **Backward Compatibility**: Maintained compatibility with existing templates

### 🌐 Technical
- **Zero Dependencies**: Continues to maintain pure Rust implementation
- **Memory Safety**: All new features maintain Rust's memory safety guarantees  
- **Unicode Support**: Full Unicode compatibility across all new features

## [0.1.4] - 2024-08-18 - Stability Release

### 🔧 Fixed
- Fixed nested layout inheritance block replacement boundary calculation
- Enhanced loop error handling to properly detect unsupported function calls
- Maintained backward compatibility for missing variables in loops  
- All 127 tests passing with comprehensive coverage

## [0.1.3] - 2024-XX-XX - Zero Dependencies Edition

### 🗑️ Removed
- **Zero Dependencies**: Completely self-contained with no external dependencies
  - Removed `thiserror` - replaced with native Error trait implementation
  - Removed `tempfile` - custom temp directory utilities using std::env::temp_dir()
  - Pure Rust standard library implementation
  - Faster compilation, smaller binaries, reduced attack surface

### ✅ Added  
- **Deep Dot Notation Support**: Complete support for unlimited depth object traversal
  - Variables: `{{user.profile.settings.theme.color}}`
  - Conditionals: `{{if user.account.subscription.active}}`
  - Array indexing: `{{items.0.properties.name}}`
  - Recursive property resolution with proper error handling

## [0.1.1] - 2024-XX-XX - Security & Testing Release

### 🛡️ Added
- Comprehensive security testing suite
- 85+ tests with 100% coverage following TDD methodology
- Complete documentation with TDD development guidelines
- Advanced XSS and injection protection
- Performance optimizations and stress testing
- Strict Test-Driven Development practices implemented

## [0.1.0] - 2024-XX-XX - Initial Release

### 🎉 Added
- **Core Template Engine**: Mustache-inspired syntax with Rust performance
- **Security First**: XSS-safe HTML escaping by default
- **Rich Syntax Support**:
  - Variable substitution with `{{variable}}`
  - Raw HTML output with `{{& variable}}`
  - Conditional rendering with `{{if condition}}...{{/if}}`
  - Loop iteration with `{{for item in items}}...{{/for}}`
  - Template includes with `{{include "template.html"}}`
  - Comments with `{{! comment text }}`
  - Object property access with `{{object.property}}`

### ⚡ Added  
- **Performance Features**:
  - Template caching for improved performance
  - Efficient string processing and memory management
  
### 🧪 Added
- **Comprehensive Testing**:
  - 100% test coverage across all modules
  - Extensive security testing suite
  - Integration tests for complex scenarios
  - Unit tests for individual components
  - Performance and stress testing

### 🛡️ Added
- **Security Features**:
  - XSS prevention through automatic HTML escaping
  - Path traversal protection for template includes
  - Template injection prevention
  - Unicode security handling
  - Memory exhaustion protection

### 📚 Added
- **Developer Experience**:
  - Clean, intuitive Rust API
  - Comprehensive documentation with examples
  - Magical themed naming conventions (RuneEngine, RuneScroll, RuneSymbol)
  - Detailed error messages

---

## Roadmap

### v0.3.0 - Internationalization & Advanced Filters
- **i18n Support**: `{{t "key"}}` syntax for translations
- **Pluralization**: Smart plural forms based on count
- **Custom Filter Registration**: API for user-defined filters
- **Advanced Math Filters**: Mathematical operations and formatting

### v0.4.0 - Developer Experience  
- **Better Error Messages**: Line/column numbers and suggestions
- **Template Debugging**: Step-through debugging capabilities
- **IDE Integration**: Language Server Protocol support
- **Hot Reload**: Development-time template reloading

### v0.5.0 - Ecosystem Integration
- **Async Support**: Non-blocking template rendering
- **Web Framework Integration**: First-class Axum, Warp, Actix support
- **WASM Compatibility**: Browser and edge runtime support
- **CLI Tools**: Command-line template processing utilities

---

*"The magic of templating lies not in complexity, but in elegant simplicity that scales."*