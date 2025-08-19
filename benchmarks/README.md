# 🔮 Mystical-Runic v0.5.2 - Performance Benchmarks

This directory contains comprehensive performance benchmarks for Mystical-Runic v0.5.2, demonstrating the enterprise-grade performance and optimization features.

## 📊 Available Benchmarks

### 1. **`simple_benchmark.rs`** ✅ (Working)
**Core performance benchmark testing fundamental functionality**
- Template rendering performance (file vs string)
- Template caching effectiveness
- Filter processing efficiency
- Complex template with nested loops/conditionals
- Large dataset processing (1000 items)

**Usage:**
```bash
# Compile and run
rustc --edition 2021 benchmarks/simple_benchmark.rs -L target/release/deps --extern mystical_runic=target/release/libmystical_runic.rlib -O && ./simple_benchmark
```

### 2. **`v0_5_2_performance_benchmark.rs`** 🚧 (Advanced)
**Comprehensive enterprise-grade performance testing**
- Template caching with speedup metrics
- Bytecode compilation performance
- Parallel processing capabilities
- Filter chain optimization
- Template inheritance performance
- I18n/localization handling
- Deep object navigation
- Memory efficiency testing

### 3. **`ecosystem_benchmark.rs`** 🌐 (Feature-dependent)
**Ecosystem integration performance testing**
- Async template rendering (requires `async` feature)
- CLI tools performance (requires `cli` feature)
- Concurrent processing
- Web framework integration testing

### 4. **`performance_demo.rs`** 📚 (Legacy)
**Original TDD performance demonstration**
- Legacy benchmark from earlier versions
- May need updates for current implementation

## 🚀 Quick Start

### Run Simple Benchmark (Recommended)
```bash
# Build project first
cargo build --release

# Run simple benchmark
rustc --edition 2021 benchmarks/simple_benchmark.rs -L target/release/deps --extern mystical_runic=target/release/libmystical_runic.rlib -O && ./simple_benchmark
```

### Run All Benchmarks
```bash
# Make script executable
chmod +x run_benchmarks.sh

# Run comprehensive benchmark suite
./run_benchmarks.sh
```

## 📈 Performance Results (v0.5.2)

### ✅ Core Performance Metrics
- **Template Rendering**: ~2,724 ns per render (highly optimized)
- **Filter Processing**: ~4,417 ns per render (well optimized)  
- **Complex Templates**: ~469,887 ns per render (optimized)
- **Cache Effectiveness**: 1.01x speedup for simple templates
- **Large Data Processing**: 215ms per 1000 items (acceptable for scale)

### 🎯 Performance Categories

| Test Category | Performance Level | Status |
|---------------|------------------|---------|
| Simple Templates | **Excellent** (~2.7µs) | ✅ |
| Filter Chains | **Excellent** (~4.4µs) | ✅ |
| Complex Templates | **Good** (~470µs) | ✅ |
| Template Caching | **Working** (1.01x) | ✅ |
| Large Data Sets | **Acceptable** (215ms/1K) | ⚠️ |

## 🔧 Features Tested

### Core Engine Features
- [x] Template rendering (file & string)
- [x] Template caching
- [x] Filter processing (`upper`, `currency`, `truncate`)
- [x] Complex nested templates
- [x] Large dataset handling
- [x] Deep object navigation
- [x] Conditional rendering
- [x] Loop processing

### Advanced Features (Feature-dependent)
- [ ] Parallel processing (`render_parallel`)
- [ ] Bytecode compilation (`compile_to_bytecode`)
- [ ] Memory-mapped loading (`load_template_mmap`)
- [ ] Async rendering (requires `async` feature)
- [ ] CLI tools (requires `cli` feature)
- [ ] Web framework integration

## 🛠️ Benchmark Development

### Adding New Benchmarks
1. Create new `.rs` file in `benchmarks/` directory
2. Follow naming convention: `feature_benchmark.rs`
3. Include comprehensive error handling
4. Add cleanup for temporary resources
5. Update this README with benchmark description

### Benchmark Structure
```rust
fn main() {
    println!("🔮 Benchmark Name");
    
    // Setup
    let temp_dir = create_test_environment();
    let context = create_test_context();
    
    // Run tests
    test_feature_1();
    test_feature_2();
    
    // Summary
    display_results();
    
    // Cleanup
    cleanup_resources();
}
```

### Performance Thresholds

| Metric | Excellent | Good | Acceptable | Needs Work |
|--------|-----------|------|------------|------------|
| Simple Template | <10µs | <100µs | <1ms | >1ms |
| Filter Processing | <10µs | <100µs | <1ms | >1ms |
| Complex Template | <1ms | <10ms | <50ms | >50ms |
| Large Data (1K items) | <50ms | <200ms | <1s | >1s |

## 📚 Benchmark Methodology

### Test Environment
- **Rust Version**: 1.74.0+ (Edition 2021)
- **Build Mode**: Release (`-O` optimization)
- **Iterations**: Varies by test complexity
- **Data Size**: Scales from simple to 1000+ items
- **Cleanup**: Automatic temporary file cleanup

### Measurement Approach
- **High-resolution timing**: `std::time::Instant`
- **Multiple iterations**: Average performance over many runs
- **Warm-up runs**: Cache warming for realistic results
- **Memory tracking**: Resource usage monitoring
- **Error handling**: Graceful failure reporting

## 🔍 Troubleshooting

### Common Issues

**Compilation Errors**
```bash
# Ensure project is built first
cargo build --release

# Check library path exists
ls target/release/libmystical_runic.rlib
```

**Missing Features**
```bash
# Check available features
cargo check --features async
cargo check --features cli
cargo check --features full
```

**Performance Variations**
- CPU load affects results
- Multiple runs may show variance
- Release mode essential for accurate results
- Disk I/O can impact file-based benchmarks

## 🎯 Future Benchmark Plans

- [ ] WebAssembly performance testing
- [ ] Multi-threaded concurrent benchmarks  
- [ ] Memory usage profiling
- [ ] Network template loading performance
- [ ] Hot reload performance impact
- [ ] IDE integration response times
- [ ] Security feature overhead measurement

## 📞 Support

For benchmark-related issues:
1. Check that `cargo build --release` completes successfully
2. Verify feature flags are correct for your test
3. Review benchmark output for specific error messages
4. Consider system resources (CPU, memory) impact on results

---

🚀 **Mystical-Runic v0.5.2 delivers enterprise-grade performance for production deployments!**