use eframe::{egui, epi};

use crate::ciphers::{Caesar, LATIN, Cipher};

pub struct CryptoApp {
    plaintext: String,
    ciphertext: String,
    alphabet: String,
    encrypt_mode: bool,
}

impl Default for CryptoApp {
    fn default() -> Self {
        Self {
            plaintext: String::new(),
            ciphertext: String::new(),
            alphabet: String::from(LATIN),
            encrypt_mode: true,
        }
    }
}

impl epi::App for CryptoApp {
    fn name(&self) -> &str {
        "Classic Cryptography"
    }

    /// Called once before the first frame.
    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {
        let Self{ plaintext, ciphertext, alphabet, encrypt_mode } = self;

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
            egui::warn_if_debug_build(ui);
        });

        egui::SidePanel::left("control_panel").show(ctx, |ui| {
            ui.label("Alphabet");
            ui.add(egui::Slider::new(&mut 0, 0..=25));
            ui.add(egui::TextEdit::singleline(alphabet));
            if ui.button("Encode").clicked() {
                let caesar = Caesar::new(1, LATIN);
                *ciphertext = caesar.encrypt(plaintext);
            };
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Plaintext");
            ui.add(egui::TextEdit::multiline(plaintext).hint_text("Plaintext Here"));
            ui.label("Ciphertext");
            ui.add(egui::TextEdit::multiline(ciphertext).hint_text(""));
        });
        

    }
}