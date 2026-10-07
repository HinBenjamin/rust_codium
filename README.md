# RustCodium (Prototype)

Prototype d'éditeur de texte natif et léger développé entièrement en Rust. Ce projet explore une alternative performante et économe en ressources face aux éditeurs basés sur Electron (comme VS Code ou VSCodium), en s'appuyant sur le framework graphique `egui`.

## Fonctionnalités
- Édition de texte (police monospace).
- Interface graphique native avec très faible empreinte mémoire.
- Boîtes de dialogue système pour l'ouverture et l'enregistrement de fichiers.

## Installation

### Via Homebrew (macOS)
*(Note : Ce dépôt étant un prototype, la formule Homebrew nécessitera d'être publiée sur un "tap" pour fonctionner publiquement. Voici la structure de commande prévue)*
```bash
brew tap HinBenjamin/rustcodium
brew install rustcodium
```

### Compilation depuis les sources

1. **Prérequis :** Installer Rust et `cargo`
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Cloner le projet :**
   ```bash
   git clone https://github.com/HinBenjamin/rust_codium.git
   cd rust_codium
   ```

3. **Lancer l'éditeur :**
   ```bash
   cargo run --release
   ```

4. **Installation locale :**
   Pour installer l'exécutable dans `~/.cargo/bin` et le lancer de n'importe où via la commande `rust_codium` :
   ```bash
   cargo install --path .
   ```
