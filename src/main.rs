use eframe::egui;
use rfd::FileDialog;
use std::fs;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
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
    filepath: Option<std::path::PathBuf>,
}

impl Default for RustCodium {
    fn default() -> Self {
        Self {
            code: "// Écris ton code ici...\nfn main() {\n    println!(\"Hello World\");\n}".to_owned(),
            filepath: None,
        }
    }
}

impl eframe::App for RustCodium {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Barre de menu
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("Fichier", |ui| {
                    if ui.button("Ouvrir...").clicked() {
                        if let Some(path) = FileDialog::new().pick_file() {
                            if let Ok(content) = fs::read_to_string(&path) {
                                self.code = content;
                                self.filepath = Some(path);
                            }
                        }
                        ui.close_menu();
                    }
                    if ui.button("Enregistrer").clicked() {
                        if let Some(path) = &self.filepath {
                            let _ = fs::write(path, &self.code);
                        } else if let Some(path) = FileDialog::new().save_file() {
                            let _ = fs::write(&path, &self.code);
                            self.filepath = Some(path);
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

        // Zone d'édition de texte
        egui::CentralPanel::default().show(ctx, |ui| {
            let editor = egui::TextEdit::multiline(&mut self.code)
                .font(egui::TextStyle::Monospace)
                .code_editor()
                .lock_focus(true)
                .desired_width(f32::INFINITY);
            
            ui.add_sized(ui.available_size(), editor);
        });
    }
}
