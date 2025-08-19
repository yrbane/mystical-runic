#!/bin/bash

# Script pour générer la documentation Mystical-Runic
# Generate comprehensive documentation for Mystical-Runic

set -e

echo "🔮 Génération de la documentation Mystical-Runic..."

# Clean previous documentation
echo "📝 Nettoyage de la documentation précédente..."
cargo clean --doc

# Generate documentation with all features
echo "🚀 Génération avec toutes les fonctionnalités..."
RUSTDOCFLAGS="--cfg docsrs" cargo doc --all-features --no-deps --open

echo "✨ Documentation générée avec succès !"
echo "🌟 Documentation disponible dans: target/doc/mystical_runic/index.html"

# Optional: Generate documentation for specific features
echo ""
echo "📊 Génération optionnelle pour des fonctionnalités spécifiques:"

echo "  - Génération avec support async..."
RUSTDOCFLAGS="--cfg docsrs" cargo doc --features async --no-deps

echo "  - Génération avec frameworks web..."
RUSTDOCFLAGS="--cfg docsrs" cargo doc --features web-frameworks --no-deps

echo "  - Génération avec support WASM..."
RUSTDOCFLAGS="--cfg docsrs" cargo doc --features wasm --no-deps

echo "  - Génération avec outils CLI..."
RUSTDOCFLAGS="--cfg docsrs" cargo doc --features cli --no-deps

echo ""
echo "🎉 Toutes les variantes de documentation ont été générées !"
echo "📚 Vous pouvez maintenant consulter la documentation complète."