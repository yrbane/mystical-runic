// Performance tests following TDD methodology
// 🔴 RED Phase: Write failing tests first

use mystical_runic::{TemplateEngine, TemplateContext, TemplateValue};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::time::{Duration, Instant};

// Utility to create temporary directories for testing
fn create_temp_dir() -> PathBuf {
    let mut temp_path = std::env::temp_dir();
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    temp_path.push(format!("mystical_runic_perf_test_{}_{}", std::process::id(), timestamp));
    let _ = std::fs::create_dir_all(&temp_path);
    temp_path
}

// 🔴 RED: Test for parallel template processing
#[test]
fn test_parallel_template_processing() {
    let templates_path = create_temp_dir();
    
    // Create multiple large template files
    for i in 0..10 {
        let large_template = format!(
            "{{{{for item in items}}}}Large template {} content: {{{{item.name}}}} - {{{{item.value}}}}{{{{/for}}}}",
            i
        );
        fs::write(
            templates_path.join(format!("template_{}.html", i)),
            large_template
        ).unwrap();
    }
    
    let mut engine = TemplateEngine::new(templates_path.to_str().unwrap());
    let mut context = TemplateContext::new();
    
    // Create large dataset
    let items: Vec<TemplateValue> = (0..1000)
        .map(|i| {
            let mut item = HashMap::new();
            item.insert("name".to_string(), TemplateValue::String(format!("Item {}", i)));
            item.insert("value".to_string(), TemplateValue::Number(i));
            TemplateValue::Object(item)
        })
        .collect();
    context.set("items", TemplateValue::Array(items));
    
    // Test sequential processing time
    let start_sequential = Instant::now();
    let mut sequential_results = Vec::new();
    for i in 0..10 {
        let result = engine.render(&format!("template_{}.html", i), &context).unwrap();
        sequential_results.push(result);
    }
    let sequential_duration = start_sequential.elapsed();
    
    // 🔴 This should fail initially - parallel processing not implemented yet
    let start_parallel = Instant::now();
    let parallel_results = engine.render_parallel(
        &(0..10).map(|i| format!("template_{}.html", i)).collect::<Vec<_>>(),
        &context
    ).unwrap();
    let parallel_duration = start_parallel.elapsed();
    
    // Assert parallel is faster (should fail initially)
    assert!(parallel_duration < sequential_duration, 
           "Parallel processing should be faster: parallel={:?} vs sequential={:?}", 
           parallel_duration, sequential_duration);
    
    // Assert results are the same
    assert_eq!(sequential_results, parallel_results, "Results should be identical");
}

// 🔴 RED: Test for memory-mapped file loading
#[test]
fn test_memory_mapped_file_loading() {
    let templates_path = create_temp_dir();
    
    // Create a very large template file (5MB)
    let large_content = "{{for item in items}}".repeat(100000) + 
                       "Large content {{item}}" + 
                       &"{{/for}}".repeat(100000);
    fs::write(templates_path.join("huge_template.html"), &large_content).unwrap();
    
    let mut engine = TemplateEngine::new(templates_path.to_str().unwrap());
    
    // Test regular loading time
    let start_regular = Instant::now();
    let _regular_template = engine.load_template("huge_template.html").unwrap();
    let regular_duration = start_regular.elapsed();
    
    // 🔴 This should fail initially - memory mapping not implemented yet
    let start_mmap = Instant::now();
    let _mmap_template = engine.load_template_mmap("huge_template.html").unwrap();
    let mmap_duration = start_mmap.elapsed();
    
    // Assert memory-mapped loading is faster for large files
    assert!(mmap_duration < regular_duration,
           "Memory-mapped loading should be faster for large files: mmap={:?} vs regular={:?}",
           mmap_duration, regular_duration);
}

// 🔴 RED: Test for template compilation to bytecode
#[test]
fn test_template_compilation_to_bytecode() {
    let templates_path = create_temp_dir();
    
    // Create complex template
    let complex_template = r#"
        {{for user in users}}
            <div class="user">
                <h2>{{user.name}}</h2>
                {{if user.active}}
                    <span class="status">Active</span>
                    {{for skill in user.skills}}
                        <span class="skill">{{skill.name}}: {{skill.level}}</span>
                    {{/for}}
                {{/if}}
            </div>
        {{/for}}
    "#;
    fs::write(templates_path.join("complex.html"), complex_template).unwrap();
    
    let mut engine = TemplateEngine::new(templates_path.to_str().unwrap());
    let mut context = TemplateContext::new();
    
    // Create complex dataset
    let users: Vec<TemplateValue> = (0..100)
        .map(|i| {
            let mut user = HashMap::new();
            user.insert("name".to_string(), TemplateValue::String(format!("User {}", i)));
            user.insert("active".to_string(), TemplateValue::Bool(i % 2 == 0));
            
            let skills: Vec<TemplateValue> = (0..5)
                .map(|j| {
                    let mut skill = HashMap::new();
                    skill.insert("name".to_string(), TemplateValue::String(format!("Skill {}", j)));
                    skill.insert("level".to_string(), TemplateValue::Number(j * 10));
                    TemplateValue::Object(skill)
                })
                .collect();
            user.insert("skills".to_string(), TemplateValue::Array(skills));
            
            TemplateValue::Object(user)
        })
        .collect();
    context.set("users", TemplateValue::Array(users));
    
    // Test interpreted rendering time (10 iterations)
    let start_interpreted = Instant::now();
    for _ in 0..10 {
        let _result = engine.render("complex.html", &context).unwrap();
    }
    let interpreted_duration = start_interpreted.elapsed();
    
    // 🔴 This should fail initially - bytecode compilation not implemented yet
    // Compile template to bytecode
    let compiled_template = engine.compile_to_bytecode("complex.html").unwrap();
    
    // Test compiled rendering time (10 iterations)
    let start_compiled = Instant::now();
    for _ in 0..10 {
        let _result = engine.render_compiled(&compiled_template, &context).unwrap();
    }
    let compiled_duration = start_compiled.elapsed();
    
    // Assert compiled version is faster for repeated renders
    assert!(compiled_duration < interpreted_duration,
           "Compiled template should be faster: compiled={:?} vs interpreted={:?}",
           compiled_duration, interpreted_duration);
}

// 🔴 RED: Test for bytecode cache management
#[test]
fn test_bytecode_cache_management() {
    let templates_path = create_temp_dir();
    
    fs::write(templates_path.join("cached.html"), "Hello {{name}}!").unwrap();
    
    let mut engine = TemplateEngine::new(templates_path.to_str().unwrap());
    
    // 🔴 This should fail initially - bytecode caching not implemented yet
    
    // First compilation should create cache
    let _compiled1 = engine.compile_to_bytecode("cached.html").unwrap();
    assert!(!engine.is_bytecode_cached("cached.html"), "Should not be cached initially");
    
    // Enable caching
    engine.enable_bytecode_cache(true);
    let _compiled2 = engine.compile_to_bytecode("cached.html").unwrap();
    assert!(engine.is_bytecode_cached("cached.html"), "Should be cached after enabling");
    
    // Cache hit should be faster
    let start_cache_miss = Instant::now();
    let _compiled3 = engine.compile_to_bytecode_uncached("cached.html").unwrap();
    let cache_miss_duration = start_cache_miss.elapsed();
    
    let start_cache_hit = Instant::now();
    let _compiled4 = engine.compile_to_bytecode("cached.html").unwrap();
    let cache_hit_duration = start_cache_hit.elapsed();
    
    assert!(cache_hit_duration < cache_miss_duration,
           "Cache hit should be faster than cache miss");
}

// 🔴 RED: Test for parallel rendering with bytecode
#[test]
fn test_parallel_bytecode_rendering() {
    let templates_path = create_temp_dir();
    
    // Create multiple templates
    for i in 0..5 {
        fs::write(
            templates_path.join(format!("bytecode_{}.html", i)),
            format!("Template {} content: {{{{message}}}}", i)
        ).unwrap();
    }
    
    let mut engine = TemplateEngine::new(templates_path.to_str().unwrap());
    let mut context = TemplateContext::new();
    context.set_string("message", "Hello from bytecode!");
    
    // 🔴 This should fail initially - parallel bytecode rendering not implemented yet
    
    // Compile all templates to bytecode
    let template_names: Vec<String> = (0..5).map(|i| format!("bytecode_{}.html", i)).collect();
    let compiled_templates = engine.compile_templates_parallel(&template_names).unwrap();
    
    // Test parallel rendering with compiled templates
    let start_parallel = Instant::now();
    let results = engine.render_compiled_parallel(&compiled_templates, &context).unwrap();
    let parallel_duration = start_parallel.elapsed();
    
    // Verify results
    assert_eq!(results.len(), 5, "Should have 5 results");
    for (i, result) in results.iter().enumerate() {
        assert!(result.contains(&format!("Template {}", i)), "Result should contain template number");
        assert!(result.contains("Hello from bytecode!"), "Result should contain message");
    }
    
    // Should complete in reasonable time (less than 100ms for this test)
    assert!(parallel_duration < Duration::from_millis(100),
           "Parallel bytecode rendering should be fast: {:?}", parallel_duration);
}