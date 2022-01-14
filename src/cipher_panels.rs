use eframe::{egui, epi};

use crate::ciphers::{Caesar, LATIN, Cipher};

#[derive(Debug, PartialEq)]
pub enum Mode {
    Encrypt,
    Decrypt,
}

pub struct CaesarApp {
    plaintext: String,
    ciphertext: String,
    alphabet: String,
    key: u32,
    mode: Mode,
}

impl Default for CaesarApp {
    fn default() -> Self {
        Self {
            plaintext: String::new(),
            ciphertext: String::new(),
            alphabet: String::from(LATIN),
            key: 0,
            mode: Mode::Encrypt,
        }
    }
}

impl epi::App for CaesarApp {
    fn name(&self) -> &str {
        "Caesar Cipher"
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
        let Self{ plaintext, ciphertext, alphabet, key, mode } = self;

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
            ui.add(egui::TextEdit::singleline(alphabet));
            let alpha_range = 0u32..=((alphabet.chars().count()-1) as u32);
            ui.add(egui::Slider::new(key, alpha_range));


            ui.horizontal(|ui| {
                ui.selectable_value(mode, Mode::Encrypt, "Encrypt");
                ui.selectable_value(mode, Mode::Decrypt, "Decrypt");
            });

            
            let caesar = Caesar::new(*key as usize, LATIN);
            if *mode == Mode::Encrypt {
                *ciphertext = caesar.encrypt(&plaintext);
            } else {
                let caesar = Caesar::new(*key as usize, LATIN);
                *plaintext = caesar.decrypt(&ciphertext);
            }

        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Plaintext");
            ui.add(egui::TextEdit::multiline(plaintext).hint_text("Plaintext Here"));
            ui.label("Ciphertext");
            ui.add(egui::TextEdit::multiline(ciphertext).hint_text("Ciphertext Here"));
        });
        

    }
}