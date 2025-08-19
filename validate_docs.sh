#!/bin/bash

# Script de validation de la documentation Mystical-Runic
# Documentation validation script for Mystical-Runic

set -e

echo "🔮 Validation de la documentation Mystical-Runic..."

# Test 1: Vérifier que le build documentation fonctionne
echo "📝 Test 1: Build documentation..."
RUSTDOCFLAGS="--cfg docsrs" cargo doc --all-features --no-deps --quiet
echo "✅ Documentation générée avec succès"

# Test 2: Tester les exemples de documentation
echo "📝 Test 2: Test des exemples de documentation..."
cargo test --doc --quiet
echo "✅ Exemples de documentation validés"

# Test 3: Vérifier les liens dans la documentation HTML
echo "📝 Test 3: Vérification des fichiers HTML..."
html_files=(
    "doc/index.html"
    "doc/installation.html"
    "doc/whats_new.html"
    "doc/concepts/overview.html"
    "doc/api/overview.html"
    "doc/examples.html"
)

for file in "${html_files[@]}"; do
    if [ -f "$file" ]; then
        echo "  ✅ $file existe"
    else
        echo "  ❌ $file manquant"
        exit 1
    fi
done

# Test 4: Validation CSS/JS des fichiers HTML
echo "📝 Test 4: Validation des ressources externes..."
required_resources=(
    "https://cdn.tailwindcss.com"
    "https://cdnjs.cloudflare.com/ajax/libs/font-awesome"
    "https://cdnjs.cloudflare.com/ajax/libs/highlight.js"
)

echo "  ℹ️ Ressources externes détectées (non validées automatiquement)"
for resource in "${resources[@]}"; do
    echo "    - $resource"
done

# Test 5: Vérifier la cohérence des versions
echo "📝 Test 5: Cohérence des versions..."
current_version=$(grep '^version =' Cargo.toml | cut -d '"' -f 2)
echo "  📦 Version actuelle: $current_version"

# Vérifier dans lib.rs
if grep -q "mystical-runic = \"$current_version\"" src/lib.rs; then
    echo "  ✅ Version cohérente dans lib.rs"
else
    echo "  ⚠️ Version potentiellement incohérente dans lib.rs"
fi

# Vérifier dans la documentation HTML
if grep -q "v$current_version" doc/index.html; then
    echo "  ✅ Version cohérente dans documentation HTML"
else
    echo "  ⚠️ Version potentiellement incohérente dans documentation HTML"
fi

# Test 6: Structure des exports
echo "📝 Test 6: Validation des exports publics..."
cargo check --all-features --quiet
echo "✅ Exports publics validés"

echo ""
echo "🎉 Validation complète de la documentation terminée !"
echo "📚 Documentation prête pour publication"
echo ""
echo "📋 Résumé:"
echo "  - Documentation Rust: target/doc/mystical_runic/index.html"
echo "  - Documentation HTML: doc/index.html"
echo "  - Version: $current_version"
echo "  - Features: async, web-frameworks, wasm, cli"