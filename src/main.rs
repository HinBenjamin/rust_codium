use eframe::egui;
use rfd::FileDialog;
use std::fs;
use std::path::{Path, PathBuf};

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_title("RustCodium Prototype"),
        ..Default::default()
    };
    eframe::run_native(
        "RustCodium",
        options,
        Box::new(|_cc| Ok(Box::new(RustCodium::default()))),
    )
}

struct RustCodium {
    code: String,
    filepath: Option<PathBuf>,
    workspace_dir: Option<PathBuf>,
    search_query: String,
    show_search: bool,
}

impl Default for RustCodium {
    fn default() -> Self {
        Self {
            code: "// Écris ton code ici...\nfn main() {\n    println!(\"Hello World\");\n}".to_owned(),
            filepath: None,
            workspace_dir: std::env::current_dir().ok(),
            search_query: String::new(),
            show_search: false,
        }
    }
}

impl eframe::App for RustCodium {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Raccourci clavier de recherche (Cmd+F)
        if ctx.input_mut(|i| i.consume_key(egui::Modifiers::COMMAND, egui::Key::F)) {
            self.show_search = !self.show_search;
        }

        // --- Barre de menu (Haut) ---
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("Fichier", |ui| {
                    if ui.button("Ouvrir dossier (Espace de travail)...").clicked() {
                        if let Some(path) = FileDialog::new().pick_folder() {
                            self.workspace_dir = Some(path);
                        }
                        ui.close_menu();
                    }
                    if ui.button("Ouvrir fichier...").clicked() {
                        if let Some(path) = FileDialog::new().pick_file() {
                            if let Ok(content) = fs::read_to_string(&path) {
                                self.code = content;
                                self.filepath = Some(path);
                            }
                        }
                        ui.close_menu();
                    }
                    if ui.button("Enregistrer (Cmd+S)").clicked() {
                        if let Some(path) = &self.filepath {
                            let _ = fs::write(path, &self.code);
                        }
                        ui.close_menu();
                    }
                });
                if let Some(path) = &self.filepath {
                    ui.separator();
                    ui.label(path.display().to_string());
                }
            });
        });

        // --- Barre de recherche (Bas) ---
        if self.show_search {
            egui::TopBottomPanel::bottom("search_bar").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("🔍 Rechercher :");
                    let response = ui.text_edit_singleline(&mut self.search_query);
                    if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                        self.show_search = false;
                    }
                    if ui.button("Fermer").clicked() {
                        self.show_search = false;
                    }
                });
            });
        }

        // --- Explorateur de fichiers (Gauche) ---
        egui::SidePanel::left("file_explorer")
            .resizable(true)
            .min_width(200.0)
            .show(ctx, |ui| {
                ui.heading("📂 Explorateur");
                ui.separator();
                egui::ScrollArea::both().show(ui, |ui| {
                    if let Some(dir) = self.workspace_dir.clone() {
                        self.render_file_tree(ui, &dir);
                    } else {
                        ui.label("Aucun dossier ouvert.");
                    }
                });
            });

        // --- Éditeur de texte (Centre) ---
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                ui.horizontal_top(|ui| {
                    // 1. Numéros de ligne
                    let line_count = self.code.lines().count().max(1);
                    let line_numbers: String = (1..=line_count).map(|i| format!("{i}")).collect::<Vec<_>>().join("\n");
                    
                    ui.label(egui::RichText::new(line_numbers)
                        .monospace()
                        .color(egui::Color32::from_gray(120)));

                    ui.separator();

                    // 2. Zone de texte avec layouter pour la coloration syntaxique
                    let mut layouter = |ui: &egui::Ui, string: &str, wrap_width: f32| {
                        let mut layout_job = highlight_syntax(string, &self.search_query);
                        layout_job.wrap.max_width = wrap_width;
                        ui.fonts(|f| f.layout_job(layout_job))
                    };

                    egui::TextEdit::multiline(&mut self.code)
                        .font(egui::TextStyle::Monospace)
                        .code_editor()
                        .lock_focus(true)
                        .desired_width(f32::INFINITY)
                        .layouter(&mut layouter)
                        .show(ui);
                });
            });
        });
    }
}

// === Méthodes utilitaires ===

impl RustCodium {
    // Affiche l'arborescence des fichiers de manière récursive
    fn render_file_tree(&mut self, ui: &mut egui::Ui, path: &Path) {
        if let Ok(entries) = fs::read_dir(path) {
            let mut entries: Vec<_> = entries.filter_map(|e| e.ok()).collect();
            // Trie : les dossiers d'abord, puis ordre alphabétique
            entries.sort_by_key(|e| (!e.path().is_dir(), e.file_name()));

            for entry in entries {
                let entry_path = entry.path();
                let file_name = entry.file_name().to_string_lossy().to_string();

                if entry_path.is_dir() {
                    // Ignorer les dossiers lourds
                    if file_name == "target" || file_name == ".git" {
                        continue;
                    }
                    egui::collapsing_header::CollapsingState::load_with_default_open(
                        ui.ctx(),
                        ui.make_persistent_id(entry_path.clone()),
                        false,
                    )
                    .show_header(ui, |ui| {
                        ui.label(format!("📁 {file_name}"));
                    })
                    .body(|ui| {
                        self.render_file_tree(ui, &entry_path);
                    });
                } else {
                    let is_selected = self.filepath == Some(entry_path.clone());
                    if ui.selectable_label(is_selected, format!("📄 {file_name}")).clicked() {
                        if let Ok(content) = fs::read_to_string(&entry_path) {
                            self.code = content;
                            self.filepath = Some(entry_path);
                        }
                    }
                }
            }
        }
    }
}

// Parseur basique pour la coloration syntaxique et la recherche
fn highlight_syntax(text: &str, search_query: &str) -> egui::text::LayoutJob {
    let mut job = egui::text::LayoutJob::default();
    let font = egui::FontId::monospace(14.0);
    
    // Mots-clés Rust basiques
    let keywords = ["fn", "let", "mut", "impl", "struct", "enum", "pub", "return", "use", "for", "if", "else", "match"];
    
    // On analyse le texte en le découpant par espaces/symboles tout en les conservant
    let delimiters = [' ', '(', ')', '{', '}', '[', ']', '!', '.', ',', ';', ':', '\"', '\n', '\t'];
    
    for word in text.split_inclusive(&delimiters[..]) {
        let format = if !search_query.is_empty() && word.to_lowercase().contains(&search_query.to_lowercase()) {
            // Surligner la recherche
            egui::TextFormat {
                font_id: font.clone(),
                color: egui::Color32::BLACK,
                background: egui::Color32::YELLOW,
                ..Default::default()
            }
        } else if word.trim_start().starts_with("//") {
            // Commentaires
            egui::TextFormat::simple(font.clone(), egui::Color32::from_rgb(100, 180, 100))
        } else {
            let clean_word = word.trim_matches(&delimiters[..]);
            if keywords.contains(&clean_word) {
                // Mots-clés Rust
                egui::TextFormat::simple(font.clone(), egui::Color32::from_rgb(200, 100, 200))
            } else if word.contains('\"') {
                // Chaînes de caractères
                egui::TextFormat::simple(font.clone(), egui::Color32::from_rgb(200, 150, 50))
            } else {
                // Texte normal
                egui::TextFormat::simple(font.clone(), egui::Color32::LIGHT_GRAY)
            }
        };
        job.append(word, 0.0, format);
    }
    
    job
}
