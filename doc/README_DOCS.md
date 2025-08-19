# Documentation de Mystical-Runic

Ce répertoire contient la documentation HTML statique de Mystical-Runic, qui complète la documentation générée par `cargo doc`.

## Structure de la documentation

### Documentation HTML statique (`/doc/`)
- **`index.html`** : Page d'accueil avec vue d'ensemble des fonctionnalités
- **`installation.html`** : Guide d'installation et configuration
- **`whats_new.html`** : Nouveautés et changelog détaillé
- **`concepts/`** : Documentation conceptuelle détaillée
- **`api/`** : Référence API complémentaire
- **`examples.html`** : Exemples d'utilisation pratiques

### Documentation Rust (`cargo doc`)
La documentation principale est générée via `cargo doc` et inclut :
- Documentation complète de l'API avec rustdoc
- Exemples de code testables
- Documentation conditionnelle selon les features

## Génération de la documentation

### Documentation Rust complète
```bash
# Génération avec toutes les fonctionnalités
RUSTDOCFLAGS="--cfg docsrs" cargo doc --all-features --no-deps --open

# Script automatisé
./generate_docs.sh
```

### Documentation pour features spécifiques
```bash
# Support asynchrone uniquement
cargo doc --features async --no-deps

# Frameworks web uniquement  
cargo doc --features web-frameworks --no-deps

# Support WASM uniquement
cargo doc --features wasm --no-deps

# Outils CLI uniquement
cargo doc --features cli --no-deps
```

## Configuration Cargo.toml

La documentation est configurée avec :

```toml
[package.metadata.docs.rs]
all-features = true
default-target = "x86_64-unknown-linux-gnu"
rustdoc-args = ["--cfg", "docsrs"]
```

## Liens utiles

- **Documentation en ligne** : https://docs.rs/mystical-runic
- **Repository** : https://github.com/yrbane/mystical-runic
- **Crates.io** : https://crates.io/crates/mystical-runic

## Maintenance

1. **Mise à jour de version** : Mettre à jour les numéros de version dans tous les fichiers HTML
2. **Nouvelles fonctionnalités** : Ajouter la documentation dans `whats_new.html` et les sections appropriées
3. **Exemples de code** : Vérifier que tous les exemples dans la documentation rustdoc sont testables avec `cargo test --doc`

## Style et cohérence

- **Thème sombre** : Utilisation cohérente du thème sombre avec Tailwind CSS
- **Icônes Font Awesome** : Icônes cohérentes dans toute la documentation
- **Coloration syntaxique** : Highlight.js pour les exemples de code
- **Responsif** : Interface adaptée mobile et desktop

Cette documentation complète celle générée automatiquement par `cargo doc` et offre une expérience utilisateur riche pour découvrir et apprendre Mystical-Runic.