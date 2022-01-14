use eframe::{egui::{self, TextStyle}, epi};

use crate::ciphers::{LATIN, Cipher};

use crate::ciphers::Caesar;

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



        egui::SidePanel::left("control_panel").show(ctx, |ui| {
            ui.add_space(16.0);
            ui.label("Alphabet");
            ui.add(egui::TextEdit::singleline(alphabet));
            ui.add_space(16.0);

            ui.label("Key");
            let alpha_range = 0u32..=((alphabet.chars().count()-1) as u32);
            ui.add(egui::Slider::new(key, alpha_range));
            ui.add_space(16.0);

            ui.horizontal(|ui| {
                ui.selectable_value(mode, Mode::Encrypt, "Encrypt");
                ui.selectable_value(mode, Mode::Decrypt, "Decrypt");
            });
            ui.add_space(16.0);

            if ui.button("Clear").clicked() {
                *plaintext = String::new();
                *ciphertext = String::new();
            }
            
            let caesar = Caesar::new(*key as usize, alphabet);
            if *mode == Mode::Encrypt {
                match caesar.encrypt(&plaintext) {
                    Ok(text) => *ciphertext = text ,
                    Err(e) => *ciphertext = String::from(e),
                }
            } else {
                match caesar.decrypt(&ciphertext) {
                    Ok(text) => *plaintext = text ,
                    Err(e) => *plaintext = String::from(e),
                }
            }

        });

        egui::CentralPanel::default().show(ctx, |ui| {

            ui.label("Description:\nThe Caesar Cipher is one of the oldest and simplest forms of cryptography. The key is any positive whole number. Each letter of the plaintext is shifted that many positions in the alphabet, wrapping around at the end.");

            ui.add_space(16.0);
            ui.separator();
            ui.add_space(16.0);

            ui.label("Plaintext");
            ui.add(egui::TextEdit::multiline(plaintext).hint_text("Plaintext Here").text_style(TextStyle::Monospace));
            ui.add_space(16.0);
            ui.label("Ciphertext");
            ui.add(egui::TextEdit::multiline(ciphertext).hint_text("Ciphertext Here").text_style(TextStyle::Monospace));
        });
        

    }
}