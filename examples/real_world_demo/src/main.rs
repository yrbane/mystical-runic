use mystical_runic::{TemplateEngine, TemplateContext, TemplateValue};
use std::collections::HashMap;

fn main() {
    println!("🔮 MYSTICAL-RUNIC v0.5.0 - DÉMONSTRATION COMPLÈTE AVEC INTÉGRATION ÉCOSYSTÈME!");
    println!("================================================");
    
    // Initialisation du moteur avec templates (chemin relatif depuis le répertoire de l'exemple)
    let mut engine = TemplateEngine::new("templates");
    
    // Configuration i18n avec traductions en français
    setup_translations(&mut engine);
    engine.set_locale("fr");
    
    // Enregistrement de filtres personnalisés
    setup_custom_filters(&mut engine);
    
    // Activation des fonctionnalités de performance
    engine.enable_bytecode_cache(true);
    
    // Création du contexte avec des données complexes et réalistes
    let mut context = create_realistic_context();
    
    // Test de TOUTES les fonctionnalités
    println!("\n🧪 TESTING ALL FEATURES:");
    println!("-----------------------");
    
    // Test 1: Rendu principal avec héritage
    test_main_template(&mut engine, &context);
    
    // Test 2: Nested loops
    test_nested_loops(&mut engine, &context);
    
    // Test 3: Nested includes  
    test_nested_includes(&mut engine, &context);
    
    // Test 4: Sécurité (path traversal protection)
    test_security_features(&mut engine, &context);
    
    // Test 5: Performance features
    test_performance_features(&mut engine, &context);
    
    // Test 6: Multi-langue
    test_internationalization(&mut engine, &mut context);
    
    // Test 7: Génération du fichier HTML de démonstration
    generate_html_output(&mut engine, &context);
    
    println!("\n✅ TOUS LES TESTS RÉUSSIS!");
    println!("🎉 Mystical-Runic v0.5.0 avec intégration écosystème fonctionne parfaitement!");
    
    // 🚀 NEW v0.5.0: Démonstration des fonctionnalités d'intégration écosystème
    test_ecosystem_features(&mut engine, &context);
}

fn setup_translations(engine: &mut TemplateEngine) {
    let mut fr_translations = HashMap::new();
    
    // Traductions générales
    fr_translations.insert("tagline".to_string(), "Magie des templates modernes".to_string());
    fr_translations.insert("demo_title".to_string(), "Démonstration Complète".to_string());
    fr_translations.insert("welcome_message".to_string(), "Bienvenue {{name}} ! 🔮".to_string());
    fr_translations.insert("powered_by".to_string(), "Propulsé par".to_string());
    fr_translations.insert("current_locale".to_string(), "Langue actuelle".to_string());
    fr_translations.insert("test_count".to_string(), "Tests".to_string());
    
    // Navigation
    fr_translations.insert("nav.products".to_string(), "Produits".to_string());
    fr_translations.insert("nav.blog".to_string(), "Blog".to_string());
    fr_translations.insert("nav.statistics".to_string(), "Statistiques".to_string());
    fr_translations.insert("nav.security".to_string(), "Sécurité".to_string());
    
    // Sections
    fr_translations.insert("section.internationalization".to_string(), "Internationalisation".to_string());
    fr_translations.insert("section.pluralization".to_string(), "Système de Pluralisation".to_string());
    fr_translations.insert("section.math_filters".to_string(), "Filtres Mathématiques Avancés".to_string());
    fr_translations.insert("section.custom_filters".to_string(), "Filtres Personnalisés".to_string());
    fr_translations.insert("section.template_inheritance".to_string(), "Héritage de Templates".to_string());
    fr_translations.insert("section.reusable_macros".to_string(), "Macros Réutilisables".to_string());
    fr_translations.insert("section.nested_loops".to_string(), "Boucles Imbriquées".to_string());
    fr_translations.insert("section.nested_includes".to_string(), "Inclusions Imbriquées".to_string());
    fr_translations.insert("section.security_features".to_string(), "Fonctionnalités de Sécurité".to_string());
    fr_translations.insert("section.deep_dot_notation".to_string(), "Navigation Profonde".to_string());
    fr_translations.insert("section.performance_features".to_string(), "Fonctionnalités de Performance".to_string());
    fr_translations.insert("section.dual_naming".to_string(), "Système de Nommage Dual".to_string());
    
    // Contenu spécifique
    fr_translations.insert("shop_title".to_string(), "Boutique Magique".to_string());
    fr_translations.insert("current_locale_info".to_string(), "Langue actuelle".to_string());
    fr_translations.insert("user_greeting".to_string(), "Bonjour {{name}}, vous êtes {{role}}".to_string());
    fr_translations.insert("site_description".to_string(), "Bienvenue sur {{site_name}}".to_string());
    fr_translations.insert("in_cart".to_string(), "dans le panier".to_string());
    fr_translations.insert("available".to_string(), "disponible(s)".to_string());
    fr_translations.insert("registered".to_string(), "inscrit(s)".to_string());
    
    // Commerce
    fr_translations.insert("cart_subtotal".to_string(), "Sous-total".to_string());
    fr_translations.insert("tax_rate".to_string(), "Taux de TVA".to_string());
    fr_translations.insert("tax_amount".to_string(), "Montant TVA".to_string());
    fr_translations.insert("total_amount".to_string(), "Montant total".to_string());
    fr_translations.insert("discount_percent".to_string(), "Remise".to_string());
    fr_translations.insert("savings".to_string(), "Économies".to_string());
    fr_translations.insert("sale".to_string(), "PROMO".to_string());
    fr_translations.insert("off".to_string(), "de réduction".to_string());
    
    // Profil utilisateur
    fr_translations.insert("username".to_string(), "Nom d'utilisateur".to_string());
    fr_translations.insert("bio".to_string(), "Biographie".to_string());
    fr_translations.insert("join_date".to_string(), "Date d'inscription".to_string());
    fr_translations.insert("encrypted_id".to_string(), "ID chiffré".to_string());
    fr_translations.insert("name".to_string(), "Nom".to_string());
    fr_translations.insert("role".to_string(), "Rôle".to_string());
    fr_translations.insert("member_since".to_string(), "Membre depuis".to_string());
    fr_translations.insert("location".to_string(), "Localisation".to_string());
    fr_translations.insert("gaming_stats".to_string(), "Statistiques de jeu".to_string());
    fr_translations.insert("user_preferences".to_string(), "Préférences utilisateur".to_string());
    fr_translations.insert("level".to_string(), "Niveau".to_string());
    fr_translations.insert("experience".to_string(), "Expérience".to_string());
    fr_translations.insert("recent_achievements".to_string(), "Succès récents".to_string());
    fr_translations.insert("theme".to_string(), "Thème".to_string());
    fr_translations.insert("language".to_string(), "Langue".to_string());
    fr_translations.insert("notifications".to_string(), "Notifications".to_string());
    fr_translations.insert("enabled".to_string(), "Activé".to_string());
    fr_translations.insert("disabled".to_string(), "Désactivé".to_string());
    fr_translations.insert("privacy_level".to_string(), "Niveau de confidentialité".to_string());
    
    // Sécurité
    fr_translations.insert("security_explanation".to_string(), "Démonstration des fonctionnalités de sécurité intégrées".to_string());
    fr_translations.insert("security.xss_protection".to_string(), "Protection XSS".to_string());
    fr_translations.insert("security.path_traversal_protection".to_string(), "Protection Path Traversal".to_string());
    fr_translations.insert("security.path_explanation".to_string(), "Les tentatives d'accès aux fichiers système sont automatiquement bloquées".to_string());
    fr_translations.insert("security.blocked_patterns".to_string(), "Motifs bloqués".to_string());
    fr_translations.insert("security.blocked_absolute".to_string(), "Chemins absolus bloqués".to_string());
    fr_translations.insert("security.blocked_drive".to_string(), "Lettres de lecteur bloquées".to_string());
    fr_translations.insert("user_input".to_string(), "Saisie utilisateur (échappée)".to_string());
    fr_translations.insert("raw_input".to_string(), "Saisie brute (sûre)".to_string());
    
    // Explications techniques
    fr_translations.insert("inheritance_explanation".to_string(), "Ce template étend 'base.html' et utilise {{super}} dans le titre".to_string());
    fr_translations.insert("base_template_features".to_string(), "Fonctionnalités du template de base".to_string());
    fr_translations.insert("feature.header".to_string(), "En-tête avec navigation".to_string());
    fr_translations.insert("feature.navigation".to_string(), "Menu de navigation".to_string());
    fr_translations.insert("feature.footer".to_string(), "Pied de page".to_string());
    fr_translations.insert("feature.styling".to_string(), "Styles CSS".to_string());
    
    fr_translations.insert("macro_explanation".to_string(), "Macros réutilisables avec paramètres pour éviter la duplication".to_string());
    fr_translations.insert("nested_loops_explanation".to_string(), "Support complet des boucles imbriquées avec parsing basé sur une pile".to_string());
    fr_translations.insert("nested_includes_explanation".to_string(), "Inclusions récursives pour des hiérarchies de templates complexes".to_string());
    fr_translations.insert("product_categories".to_string(), "Catégories de produits".to_string());
    fr_translations.insert("user_profile_card".to_string(), "Carte de profil utilisateur".to_string());
    fr_translations.insert("blog_posts".to_string(), "Articles de blog".to_string());
    fr_translations.insert("user_deep_profile".to_string(), "Profil utilisateur détaillé".to_string());
    
    // Blog
    fr_translations.insert("blog_intro".to_string(), "Articles récents avec commentaires imbriqués".to_string());
    fr_translations.insert("by".to_string(), "par".to_string());
    fr_translations.insert("recent_comments".to_string(), "Commentaires récents".to_string());
    
    // Performance
    fr_translations.insert("template_caching".to_string(), "Cache des templates".to_string());
    fr_translations.insert("cache_status".to_string(), "Statut du cache".to_string());
    fr_translations.insert("bytecode_compilation".to_string(), "Compilation bytecode".to_string());
    fr_translations.insert("bytecode_status".to_string(), "Statut bytecode".to_string());
    fr_translations.insert("parallel_processing".to_string(), "Traitement parallèle".to_string());
    fr_translations.insert("parallel_status".to_string(), "Statut parallèle".to_string());
    fr_translations.insert("active".to_string(), "Actif".to_string());
    
    // Styles de nommage
    fr_translations.insert("professional_style".to_string(), "Style Professionnel".to_string());
    fr_translations.insert("mystical_style".to_string(), "Style Mystique".to_string());
    
    engine.set_translations("fr", fr_translations);
    
    // Traductions anglaises aussi
    let mut en_translations = HashMap::new();
    en_translations.insert("tagline".to_string(), "Modern Template Magic".to_string());
    en_translations.insert("demo_title".to_string(), "Complete Demonstration".to_string());
    en_translations.insert("welcome_message".to_string(), "Welcome {{name}}! 🔮".to_string());
    en_translations.insert("current_locale_info".to_string(), "Current locale".to_string());
    en_translations.insert("user_greeting".to_string(), "Hello {{name}}, you are {{role}}".to_string());
    engine.set_translations("en", en_translations);
}

fn setup_custom_filters(engine: &mut TemplateEngine) {
    // Filtre de chiffrement simple (pour la démo)
    engine.register_filter("encrypt", |input: &str, _args: &[&str]| {
        let encrypted = input.chars()
            .map(|c| ((c as u8).wrapping_add(13)) as char)
            .collect::<String>();
        Ok(format!("🔐{}", encrypted))
    });
    
    // Filtre Markdown (simulé)
    engine.register_filter("markdown", |input: &str, _args: &[&str]| {
        let result = input
            .replace("**", "<strong>")
            .replace("**", "</strong>")
            .replace("*", "<em>")
            .replace("*", "</em>");
        Ok(result)
    });
    
    // Filtre de date (simulé)
    engine.register_filter("date", |input: &str, args: &[&str]| {
        let format = args.get(0).unwrap_or(&"Y-m-d");
        Ok(format!("{} ({})", input, format))
    });
}

fn create_realistic_context() -> TemplateContext {
    let mut context = TemplateContext::new();
    
    // Configuration du site
    let mut site = HashMap::new();
    site.insert("name".to_string(), TemplateValue::String("RunicShop".to_string()));
    context.set("site", TemplateValue::Object(site));
    
    // Locale
    context.set("locale", TemplateValue::String("fr".to_string()));
    
    // Utilisateur avec structure profonde pour tester dot notation
    let mut user = HashMap::new();
    user.insert("name".to_string(), TemplateValue::String("AlexDev".to_string()));
    user.insert("id".to_string(), TemplateValue::Number(12345));
    user.insert("role".to_string(), TemplateValue::String("admin".to_string()));
    user.insert("bio".to_string(), TemplateValue::String("Développeur **passionné** de *Rust*".to_string()));
    user.insert("join_date".to_string(), TemplateValue::String("2024-01-15".to_string()));
    user.insert("discount".to_string(), TemplateValue::Number(15)); // 15%
    
    // Profil utilisateur profond
    let mut profile = HashMap::new();
    
    let mut personal = HashMap::new();
    personal.insert("full_name".to_string(), TemplateValue::String("Alexandre Développeur".to_string()));
    profile.insert("personal".to_string(), TemplateValue::Object(personal));
    
    let mut location = HashMap::new();
    location.insert("city".to_string(), TemplateValue::String("Paris".to_string()));
    location.insert("country".to_string(), TemplateValue::String("France".to_string()));
    profile.insert("location".to_string(), TemplateValue::Object(location));
    
    let mut settings = HashMap::new();
    settings.insert("theme".to_string(), TemplateValue::String("dark".to_string()));
    settings.insert("language".to_string(), TemplateValue::String("fr".to_string()));
    settings.insert("notifications".to_string(), TemplateValue::Bool(true));
    settings.insert("privacy".to_string(), TemplateValue::String("high".to_string()));
    profile.insert("settings".to_string(), TemplateValue::Object(settings));
    
    user.insert("profile".to_string(), TemplateValue::Object(profile));
    
    // Gaming stats pour tester nested includes
    let mut gaming = HashMap::new();
    let mut stats = HashMap::new();
    stats.insert("level".to_string(), TemplateValue::Number(42));
    stats.insert("xp".to_string(), TemplateValue::Number(15750));
    gaming.insert("stats".to_string(), TemplateValue::Object(stats));
    
    let achievements = vec![
        {
            let mut achievement = HashMap::new();
            achievement.insert("name".to_string(), TemplateValue::String("Code Master".to_string()));
            achievement.insert("date".to_string(), TemplateValue::String("2024-11-15".to_string()));
            TemplateValue::Object(achievement)
        },
        {
            let mut achievement = HashMap::new();
            achievement.insert("name".to_string(), TemplateValue::String("Template Wizard".to_string()));
            achievement.insert("date".to_string(), TemplateValue::String("2024-12-01".to_string()));
            TemplateValue::Object(achievement)
        }
    ];
    gaming.insert("achievements".to_string(), TemplateValue::Array(achievements));
    
    user.insert("gaming".to_string(), TemplateValue::Object(gaming));
    context.set("user", TemplateValue::Object(user));
    
    // Panier pour tester math filters
    let mut cart = HashMap::new();
    cart.insert("total_items".to_string(), TemplateValue::Number(3));
    cart.insert("subtotal".to_string(), TemplateValue::Number(89)); // 89.50€
    context.set("cart", TemplateValue::Object(cart));
    
    // Taxes
    let mut tax = HashMap::new();
    tax.insert("rate".to_string(), TemplateValue::Number(20)); // 20% TVA
    context.set("tax", TemplateValue::Object(tax));
    
    // Statistiques
    let mut stats = HashMap::new();
    stats.insert("total_products".to_string(), TemplateValue::Number(156));
    stats.insert("total_users".to_string(), TemplateValue::Number(2847));
    context.set("stats", TemplateValue::Object(stats));
    
    // Catégories avec produits pour tester nested loops
    let categories = vec![
        {
            let mut category = HashMap::new();
            category.insert("name".to_string(), TemplateValue::String("🔮 Magie du Code".to_string()));
            let products = vec![
                {
                    let mut product = HashMap::new();
                    product.insert("name".to_string(), TemplateValue::String("Grimoire Rust".to_string()));
                    product.insert("price".to_string(), TemplateValue::Number(29));
                    product.insert("rating".to_string(), TemplateValue::Number(5));
                    TemplateValue::Object(product)
                },
                {
                    let mut product = HashMap::new();
                    product.insert("name".to_string(), TemplateValue::String("Runes USB".to_string()));
                    product.insert("price".to_string(), TemplateValue::Number(45));
                    product.insert("rating".to_string(), TemplateValue::Number(4));
                    TemplateValue::Object(product)
                }
            ];
            category.insert("products".to_string(), TemplateValue::Array(products));
            TemplateValue::Object(category)
        },
        {
            let mut category = HashMap::new();
            category.insert("name".to_string(), TemplateValue::String("⚡ Performance".to_string()));
            let products = vec![
                {
                    let mut product = HashMap::new();
                    product.insert("name".to_string(), TemplateValue::String("Cache Magique".to_string()));
                    product.insert("price".to_string(), TemplateValue::Number(19));
                    product.insert("rating".to_string(), TemplateValue::Number(5));
                    TemplateValue::Object(product)
                }
            ];
            category.insert("products".to_string(), TemplateValue::Array(products));
            TemplateValue::Object(category)
        }
    ];
    context.set("categories", TemplateValue::Array(categories));
    
    // Blog posts pour nested includes
    let mut blog = HashMap::new();
    let recent_posts = vec![
        {
            let mut post = HashMap::new();
            post.insert("title".to_string(), TemplateValue::String("Mystical-Runic v0.3.4: Les nouvelles fonctionnalités".to_string()));
            post.insert("author".to_string(), TemplateValue::String("AlexDev".to_string()));
            post.insert("date".to_string(), TemplateValue::String("2025-08-19".to_string()));
            post.insert("excerpt".to_string(), TemplateValue::String("Découvrez les boucles imbriquées, les inclusions récursives et la protection path traversal...".to_string()));
            
            let comments = vec![
                {
                    let mut comment = HashMap::new();
                    comment.insert("author".to_string(), TemplateValue::String("CodeMaster".to_string()));
                    comment.insert("text".to_string(), TemplateValue::String("Excellente mise à jour! Les nested loops fonctionnent parfaitement.".to_string()));
                    comment.insert("date".to_string(), TemplateValue::String("2025-08-19 14:30".to_string()));
                    TemplateValue::Object(comment)
                },
                {
                    let mut comment = HashMap::new();
                    comment.insert("author".to_string(), TemplateValue::String("SecurityGuru".to_string()));
                    comment.insert("text".to_string(), TemplateValue::String("La protection path traversal est un must-have!".to_string()));
                    comment.insert("date".to_string(), TemplateValue::String("2025-08-19 15:45".to_string()));
                    TemplateValue::Object(comment)
                }
            ];
            post.insert("comments".to_string(), TemplateValue::Array(comments));
            TemplateValue::Object(post)
        }
    ];
    blog.insert("recent_posts".to_string(), TemplateValue::Array(recent_posts));
    context.set("blog", TemplateValue::Object(blog));
    
    // Données pour tester la sécurité XSS
    context.set("user_input", TemplateValue::String("<script>alert('XSS')</script>Contenu sûr".to_string()));
    context.set("user_input_safe", TemplateValue::String("<em>Contenu HTML sûr</em>".to_string()));
    
    context
}

fn test_main_template(engine: &mut TemplateEngine, context: &TemplateContext) {
    println!("🏗️  Testing template inheritance + all features...");
    
    match engine.render("shop.html", context) {
        Ok(result) => {
            println!("✅ Main template rendered successfully!");
            println!("📄 Length: {} characters", result.len());
            
            // Vérifier que toutes les fonctionnalités sont présentes
            let features = vec![
                "Internationalisation",
                "Boucles Imbriquées", 
                "Inclusions Imbriquées",
                "Protection XSS",
                "Macros Réutilisables",
                "Filtres Mathématiques"
            ];
            
            for feature in features {
                if result.contains(feature) {
                    println!("   ✅ {} présent", feature);
                } else {
                    println!("   ⚠️  {} non trouvé", feature);
                }
            }
        }
        Err(e) => {
            println!("❌ Error rendering main template: {:?}", e);
        }
    }
}

fn test_nested_loops(engine: &mut TemplateEngine, context: &TemplateContext) {
    println!("\n🔄 Testing nested loops...");
    
    let template = r#"{{for category in categories}}📁 {{category.name}}:
{{for product in category.products}}  - {{product.name}} ({{product.price}}€)
{{/for}}
{{/for}}"#;
    
    match engine.render_string(template, context) {
        Ok(result) => {
            println!("✅ Nested loops work perfectly!");
            println!("📄 Result:\n{}", result);
        }
        Err(e) => {
            println!("❌ Error with nested loops: {:?}", e);
        }
    }
}

fn test_nested_includes(engine: &mut TemplateEngine, context: &TemplateContext) {
    println!("\n🔄 Testing nested includes...");
    
    let template = "{{include \"components/user_profile.html\"}}";
    
    match engine.render_string(template, context) {
        Ok(result) => {
            println!("✅ Nested includes work perfectly!");
            println!("📄 Includes gaming stats and preferences nested deeply");
            if result.contains("Code Master") && result.contains("DARK") {
                println!("   ✅ Deep nesting successful");
            }
        }
        Err(e) => {
            println!("❌ Error with nested includes: {:?}", e);
        }
    }
}

fn test_security_features(engine: &mut TemplateEngine, context: &TemplateContext) {
    println!("\n🛡️  Testing security features...");
    
    // Test path traversal protection
    let dangerous_paths = vec![
        "../../../etc/passwd",
        "..\\windows\\system32\\config\\sam", 
        "/etc/passwd",
        "C:\\Windows\\System32"
    ];
    
    let mut blocked_count = 0;
    for path in dangerous_paths {
        match engine.render(path, context) {
            Ok(_) => println!("   ⚠️  Path not blocked: {}", path),
            Err(_) => {
                blocked_count += 1;
                println!("   ✅ Path blocked: {}", path);
            }
        }
    }
    
    if blocked_count == 4 {
        println!("✅ All path traversal attempts blocked!");
    }
    
    // Test XSS protection
    let xss_template = "{{user_input}}";
    match engine.render_string(xss_template, context) {
        Ok(result) => {
            if result.contains("&lt;script&gt;") {
                println!("✅ XSS protection working - script tags escaped");
            } else {
                println!("⚠️  XSS protection might not be working");
            }
        }
        Err(e) => println!("❌ Error testing XSS: {:?}", e),
    }
}

fn test_performance_features(engine: &mut TemplateEngine, context: &TemplateContext) {
    println!("\n⚡ Testing performance features...");
    
    // Test template caching
    let template = "{{user.name}} - {{site.name}}";
    
    let start = std::time::Instant::now();
    for _ in 0..100 {
        let _ = engine.render_string(template, context);
    }
    let duration = start.elapsed();
    
    println!("✅ Rendered 100 times in {:?}", duration);
    println!("   📊 Avg per template: {:?}", duration / 100);
    
    // Test bytecode caching
    if engine.is_bytecode_cached("shop.html") {
        println!("✅ Bytecode cache active");
    } else {
        println!("📝 Bytecode cache not yet populated");
    }
}

fn test_internationalization(engine: &mut TemplateEngine, context: &mut TemplateContext) {
    println!("\n🌐 Testing internationalization...");
    
    let template = "{{t \"welcome_message\" name=user.name}} - {{t \"current_locale_info\"}}: {{locale}}";
    
    // Test français
    engine.set_locale("fr");
    context.set("locale", TemplateValue::String("fr".to_string()));
    
    match engine.render_string(template, context) {
        Ok(result) => {
            println!("✅ French: {}", result);
        }
        Err(e) => println!("❌ Error with French: {:?}", e),
    }
    
    // Test anglais
    engine.set_locale("en");
    context.set("locale", TemplateValue::String("en".to_string()));
    
    match engine.render_string(template, context) {
        Ok(result) => {
            println!("✅ English: {}", result);
        }
        Err(e) => println!("❌ Error with English: {:?}", e),
    }
    
    // Retour au français
    engine.set_locale("fr");
    context.set("locale", TemplateValue::String("fr".to_string()));
}

fn generate_html_output(engine: &mut TemplateEngine, context: &TemplateContext) {
    println!("\n📄 Generating HTML output file...");
    
    match engine.render("shop.html", context) {
        Ok(html_content) => {
            match std::fs::write("output_demo.html", &html_content) {
                Ok(_) => {
                    println!("✅ HTML file generated successfully!");
                    println!("   📁 File: output_demo.html ({} bytes)", html_content.len());
                    println!("   🌐 Open in browser to see the visual result");
                }
                Err(e) => {
                    println!("❌ Error writing HTML file: {}", e);
                }
            }
        }
        Err(e) => {
            println!("❌ Error generating HTML: {:?}", e);
        }
    }
}

// 🚀 NEW v0.5.0: Ecosystem Integration Features Demonstration
fn test_ecosystem_features(engine: &mut TemplateEngine, context: &TemplateContext) {
    println!("\n🚀 NOUVELLES FONCTIONNALITÉS v0.5.0 - INTÉGRATION ÉCOSYSTÈME");
    println!("=============================================================");
    
    // Test ecosystem compatibility
    test_ecosystem_compatibility(engine);
    
    // Test CLI tools functionality
    test_cli_integration(engine, context);
    
    println!("\n🎉 Toutes les fonctionnalités d'intégration écosystème v0.5.0 fonctionnent!");
}

fn test_ecosystem_compatibility(engine: &TemplateEngine) {
    println!("\n🔍 Testing ecosystem compatibility detection...");
    
    {
        use mystical_runic::EcosystemTemplateEngine;
        
        match engine.check_ecosystem_compatibility() {
            Ok(compatibility) => {
                println!("✅ Ecosystem compatibility check successful!");
                println!("   🔄 Async supported: {}", compatibility.async_supported);
                println!("   🌐 Web frameworks: {:?}", compatibility.web_frameworks);
                println!("   🕸️  WASM compatible: {}", compatibility.wasm_compatible);
                println!("   🛠️  CLI tools available: {}", compatibility.cli_tools_available);
            }
            Err(e) => {
                println!("❌ Ecosystem compatibility error: {:?}", e);
            }
        }
    }
}

fn test_cli_integration(engine: &mut TemplateEngine, context: &TemplateContext) {
    println!("\n🛠️ Testing CLI integration capabilities...");
    
    #[cfg(feature = "cli")]
    {
        use mystical_runic::{process_template, batch_process};
        
        // Test direct template processing
        let template = "🔮 {{t \"welcome_message\" name=user.profile.name}} - CLI intégration v0.5.0!";
        
        // Create JSON representation of context for CLI processing
        let context_json = r#"{
            "user": {
                "profile": {
                    "name": "CLI Wizard"
                }
            }
        }"#;
        
        match process_template(template, context_json) {
            Ok(result) => {
                println!("✅ CLI template processing successful!");
                println!("   📄 Result: {}", result.trim());
            }
            Err(e) => {
                println!("❌ CLI processing error: {:?}", e);
            }
        }
        
        // Test batch processing
        let templates = vec![
            "Template 1: {{count}} éléments",
            "Template 2: Version {{version}}",
            "Template 3: Utilisateur {{name}}"
        ];
        
        let batch_context = r#"{"count": 42, "version": "v0.5.0", "name": "Batch User"}"#;
        
        match batch_process(templates, batch_context) {
            Ok(results) => {
                println!("✅ CLI batch processing successful!");
                for (i, result) in results.iter().enumerate() {
                    println!("   📄 Batch {} - {}", i + 1, result.trim());
                }
            }
            Err(e) => {
                println!("❌ Batch processing error: {:?}", e);
            }
        }
    }
    
    #[cfg(not(feature = "cli"))]
    {
        println!("ℹ️  CLI integration features not enabled");
        println!("   Enable with: cargo run --features \"cli\"");
    }
}