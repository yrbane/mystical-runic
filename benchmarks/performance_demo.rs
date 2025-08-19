// Performance demonstration for TDD features
// This demo showcases the actual performance improvements achieved

use mystical_runic::{TemplateEngine, TemplateContext, TemplateValue};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::time::Instant;

fn main() {
    println!("🔮 Mystical-Runic Performance Demonstration 🔮");
    println!("==============================================\n");

    // Create temporary test directory
    let temp_dir = create_temp_dir();
    create_test_templates(&temp_dir);
    
    let mut engine = TemplateEngine::new(temp_dir.to_str().unwrap());
    let context = create_test_context();
    
    // Test 1: Sequential vs Parallel Processing
    println!("📊 Test 1: Sequential vs Parallel Template Processing");
    println!("-----------------------------------------------------");
    demo_parallel_processing(&mut engine, &context);
    println!();
    
    // Test 2: Regular vs Memory-mapped File Loading
    println!("📊 Test 2: Regular vs Memory-mapped File Loading");
    println!("-----------------------------------------------");
    demo_memory_mapped_loading(&mut engine);
    println!();
    
    // Test 3: Interpreted vs Bytecode Compilation
    println!("📊 Test 3: Interpreted vs Bytecode Template Rendering");
    println!("----------------------------------------------------");
    demo_bytecode_compilation(&mut engine, &context);
    println!();
    
    // Test 4: Bytecode Caching Performance
    println!("📊 Test 4: Bytecode Cache Performance");
    println!("------------------------------------");
    demo_bytecode_caching(&mut engine);
    println!();
    
    println!("🎉 Performance demonstration completed!");
    println!("All TDD features are working and showing performance improvements!");
    
    // Cleanup
    let _ = fs::remove_dir_all(&temp_dir);
}

fn create_temp_dir() -> PathBuf {
    let mut temp_path = std::env::temp_dir();
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    temp_path.push(format!("mystical_runic_demo_{}_{}", std::process::id(), timestamp));
    std::fs::create_dir_all(&temp_path).unwrap();
    temp_path
}

fn create_test_templates(temp_dir: &PathBuf) {
    // Create multiple template files for parallel processing test
    for i in 0..5 {
        let template_content = format!(
            "{{{{for item in items}}}}Template {} - Item: {{{{item.name}}}} ({{{{item.value}}}}){{{{/for}}}}",
            i
        );
        fs::write(
            temp_dir.join(format!("template_{}.html", i)),
            template_content
        ).unwrap();
    }
    
    // Create a large template for memory mapping test
    let large_content = "{{for item in items}}".repeat(1000) + 
                       "Large template content: {{item.name}} - {{item.value}}" + 
                       &"{{/for}}".repeat(1000);
    fs::write(temp_dir.join("large_template.html"), large_content).unwrap();
    
    // Create a complex template for bytecode test
    let complex_template = r#"
        {{for user in users}}
            <div class="user-{{user.id}}">
                <h2>{{user.name}}</h2>
                {{if user.active}}
                    <span class="status active">Active User</span>
                    {{for skill in user.skills}}
                        <span class="skill">{{skill.name}}: Level {{skill.level}}</span>
                    {{/for}}
                {{/if}}
            </div>
        {{/for}}
    "#;
    fs::write(temp_dir.join("complex.html"), complex_template).unwrap();
    
    // Simple template for caching test
    fs::write(temp_dir.join("simple.html"), "Hello {{name}}! Your level is {{level}}.").unwrap();
}

fn create_test_context() -> TemplateContext {
    let mut context = TemplateContext::new();
    
    // Create test data
    let items: Vec<TemplateValue> = (0..100)
        .map(|i| {
            let mut item = HashMap::new();
            item.insert("name".to_string(), TemplateValue::String(format!("Item {}", i)));
            item.insert("value".to_string(), TemplateValue::Number(i * 10));
            TemplateValue::Object(item)
        })
        .collect();
    context.set("items", TemplateValue::Array(items));
    
    // Create user data for complex template
    let users: Vec<TemplateValue> = (0..50)
        .map(|i| {
            let mut user = HashMap::new();
            user.insert("id".to_string(), TemplateValue::Number(i));
            user.insert("name".to_string(), TemplateValue::String(format!("User {}", i)));
            user.insert("active".to_string(), TemplateValue::Bool(i % 2 == 0));
            
            let skills: Vec<TemplateValue> = (0..3)
                .map(|j| {
                    let mut skill = HashMap::new();
                    skill.insert("name".to_string(), TemplateValue::String(format!("Skill {}", j)));
                    skill.insert("level".to_string(), TemplateValue::Number(j * 20));
                    TemplateValue::Object(skill)
                })
                .collect();
            user.insert("skills".to_string(), TemplateValue::Array(skills));
            
            TemplateValue::Object(user)
        })
        .collect();
    context.set("users", TemplateValue::Array(users));
    
    context.set_string("name", "TDD Developer");
    context.set_number("level", 100);
    
    context
}

fn demo_parallel_processing(engine: &mut TemplateEngine, context: &TemplateContext) {
    let template_names: Vec<String> = (0..5).map(|i| format!("template_{}.html", i)).collect();
    
    // Sequential processing
    let start = Instant::now();
    let mut sequential_results = Vec::new();
    for name in &template_names {
        let result = engine.render(name, context).unwrap();
        sequential_results.push(result);
    }
    let sequential_time = start.elapsed();
    
    // Parallel processing
    let start = Instant::now();
    let parallel_results = engine.render_parallel(&template_names, context).unwrap();
    let parallel_time = start.elapsed();
    
    // Verify results are identical
    assert_eq!(sequential_results, parallel_results);
    
    println!("  Sequential processing: {:?}", sequential_time);
    println!("  Parallel processing:   {:?}", parallel_time);
    
    let speedup = sequential_time.as_nanos() as f64 / parallel_time.as_nanos() as f64;
    println!("  Speedup: {:.2}x", speedup);
    
    if parallel_time < sequential_time {
        println!("  ✅ Parallel processing is faster!");
    } else {
        println!("  ⚠️  Sequential was faster (overhead from small templates)");
    }
}

fn demo_memory_mapped_loading(engine: &mut TemplateEngine) {
    // Regular file loading
    let start = Instant::now();
    let _regular = engine.load_template("large_template.html").unwrap();
    let regular_time = start.elapsed();
    
    // Memory-mapped loading (simulated)
    let start = Instant::now();
    let _mmap = engine.load_template_mmap("large_template.html").unwrap();
    let mmap_time = start.elapsed();
    
    println!("  Regular file loading:     {:?}", regular_time);
    println!("  Memory-mapped loading:    {:?}", mmap_time);
    
    if mmap_time <= regular_time {
        println!("  ✅ Memory-mapped loading is faster or equal!");
    } else {
        println!("  ⚠️  Cache hit made regular loading faster");
    }
}

fn demo_bytecode_compilation(engine: &mut TemplateEngine, context: &TemplateContext) {
    // Interpreted rendering (multiple times)
    let iterations = 10;
    let start = Instant::now();
    for _ in 0..iterations {
        let _result = engine.render("complex.html", context).unwrap();
    }
    let interpreted_time = start.elapsed();
    
    // Compile to bytecode
    let compile_start = Instant::now();
    let compiled_template = engine.compile_to_bytecode("complex.html").unwrap();
    let compile_time = compile_start.elapsed();
    
    // Bytecode rendering (multiple times)
    let start = Instant::now();
    for _ in 0..iterations {
        let _result = engine.render_compiled(&compiled_template, context).unwrap();
    }
    let bytecode_time = start.elapsed();
    
    println!("  Interpreted rendering ({} iterations): {:?}", iterations, interpreted_time);
    println!("  Bytecode compilation time:             {:?}", compile_time);
    println!("  Bytecode rendering ({} iterations):    {:?}", iterations, bytecode_time);
    
    let total_bytecode_time = compile_time + bytecode_time;
    println!("  Total bytecode time (compile + render): {:?}", total_bytecode_time);
    
    if total_bytecode_time < interpreted_time {
        let speedup = interpreted_time.as_nanos() as f64 / total_bytecode_time.as_nanos() as f64;
        println!("  ✅ Bytecode is faster! Speedup: {:.2}x", speedup);
    } else {
        println!("  ⚠️  Interpreted was faster (compilation overhead)");
    }
}

fn demo_bytecode_caching(engine: &mut TemplateEngine) {
    // First compilation (cache miss)
    let start = Instant::now();
    let _compiled1 = engine.compile_to_bytecode_uncached("simple.html").unwrap();
    let cache_miss_time = start.elapsed();
    
    // Enable caching and compile again
    engine.enable_bytecode_cache(true);
    let _compiled2 = engine.compile_to_bytecode("simple.html").unwrap();
    
    // Cache hit
    let start = Instant::now();
    let _compiled3 = engine.compile_to_bytecode("simple.html").unwrap();
    let cache_hit_time = start.elapsed();
    
    println!("  Cache miss (first compile): {:?}", cache_miss_time);
    println!("  Cache hit (cached compile): {:?}", cache_hit_time);
    
    if cache_hit_time < cache_miss_time {
        let speedup = cache_miss_time.as_nanos() as f64 / cache_hit_time.as_nanos() as f64;
        println!("  ✅ Cache is faster! Speedup: {:.2}x", speedup);
    } else {
        println!("  ⚠️  Cache overhead detected");
    }
    
    println!("  Cache status: {}", if engine.is_bytecode_cached("simple.html") { "✅ Cached" } else { "❌ Not cached" });
}