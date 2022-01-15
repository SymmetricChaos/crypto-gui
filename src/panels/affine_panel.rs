use eframe::{egui::{self, TextStyle}};

use crate::ciphers::{LATIN, Cipher};

use crate::ciphers::Affine;

use super::cipher_windows::View;

#[derive(Debug, PartialEq)]
pub enum Mode {
    Encrypt,
    Decrypt,
}

pub struct AffineWindow {
    plaintext: String,
    ciphertext: String,
    alphabet: String,
    mode: Mode,
}

impl Default for AffineWindow {
    fn default() -> Self {
        Self {
            plaintext: String::new(),
            ciphertext: String::new(),
            alphabet: String::from(LATIN),
            mode: Mode::Encrypt,
        }
    }
}


impl crate::panels::cipher_windows::View for AffineWindow {
    fn ui(&mut self, ui: &mut egui::Ui) {

        let Self{ plaintext, ciphertext, alphabet, mode } = self;

        let mut add_key = 0;
        let mut mul_key = 0;

        egui::SidePanel::left("affine_control_panel").show_inside(ui, |ui| {
            ui.add_space(16.0);
            ui.label("Alphabet");
            ui.add(egui::TextEdit::singleline(alphabet));
            ui.add_space(16.0);

            ui.label("Additive Key");
            let alpha_range = 0u32..=((alphabet.chars().count()-1) as u32);
            ui.add(egui::Slider::new(&mut add_key, alpha_range));
            ui.add_space(16.0);

            ui.label("Multiplicative Key");
            let alpha_range = 0u32..=((alphabet.chars().count()-1) as u32);
            ui.add(egui::Slider::new(&mut mul_key, alpha_range));
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
            
            let cipher = Affine::new(add_key as usize, mul_key as usize, &alphabet);
            run_cipher(mode, cipher, plaintext, ciphertext);


        });

        egui::CentralPanel::default().show_inside(ui, |ui| {

            ui.label("Description:\n");

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

fn run_cipher(mode: &mut Mode, cipher: Affine, plaintext: &mut String, ciphertext: &mut String) {
    if *mode == Mode::Encrypt {
        match cipher.encrypt(plaintext) {
            Ok(text) => *ciphertext = text ,
            Err(e) => *ciphertext = String::from(e),
        }
    } else {
        match cipher.decrypt(ciphertext) {
            Ok(text) => *plaintext = text ,
            Err(e) => *plaintext = String::from(e),
        }
    }
}


impl crate::panels::cipher_windows::CipherFrame for AffineWindow {
    fn name(&self) -> &'static str {
        "Affine Cipher"
    }

    fn show(&mut self, ctx: &egui::CtxRef, open: &mut bool) {
        let window = egui::Window::new("Affine Cipher")
            .default_width(600.0)
            .default_height(400.0)
            .vscroll(false)
            .open(open);
        window.show(ctx, |ui| self.ui(ui));
    }
}