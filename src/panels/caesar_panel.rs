use eframe::{egui::{self, TextStyle}};

use crate::ciphers::{LATIN, Cipher};

use crate::ciphers::Caesar;

use super::cipher_windows::View;

#[derive(Debug, PartialEq)]
pub enum Mode {
    Encrypt,
    Decrypt,
}

pub struct CaesarWindow {
    plaintext: String,
    ciphertext: String,
    alphabet: String,
    key: u32,
    mode: Mode,
}

impl Default for CaesarWindow {
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


impl crate::panels::cipher_windows::View for CaesarWindow {
    fn ui(&mut self, ui: &mut egui::Ui) {

        let Self{ plaintext, ciphertext, alphabet, key, mode } = self;

        egui::SidePanel::left("control_panel").show_inside(ui, |ui| {
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

        egui::CentralPanel::default().show_inside(ui, |ui| {

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


impl crate::panels::cipher_windows::CipherFrame for CaesarWindow {
    fn name(&self) -> &'static str {
        "Caesar Cipher"
    }

    fn show(&mut self, ctx: &egui::CtxRef, open: &mut bool) {
        let window = egui::Window::new("Caesar Cipher")
            .default_width(600.0)
            .default_height(400.0)
            .vscroll(false)
            .open(open);
        window.show(ctx, |ui| self.ui(ui));
    }
}