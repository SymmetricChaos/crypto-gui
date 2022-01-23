use eframe::egui::{self, RichText, Color32, TextStyle};
use rand::prelude::ThreadRng;
use crate::ciphers::Cipher;

pub fn encrypt_decrypt(ui: &mut egui::Ui, cipher: &dyn Cipher, input: &mut String, output: &mut String) {
    ui.horizontal(|ui| {
        if ui.button(RichText::from("ENCRYPT").color(Color32::GOLD)).clicked() {
            match cipher.encrypt(input) {
                Ok(text) => *output = text,
                Err(e) => *output = String::from(e),
            }
        };
        if ui.button(RichText::from("DECRYPT").color(Color32::GOLD)).clicked() {
            match cipher.decrypt(input) {
                Ok(text) => *output = text,
                Err(e) => *output = String::from(e),
            }
        }
    });
}

pub fn randomize_button(ui: &mut egui::Ui, cipher: &mut dyn Cipher) {
    let mut rng = ThreadRng::default();
    if ui.button("Randomize").clicked() {
        cipher.randomize(&mut rng)
    }
}

pub fn clear_button(ui: &mut egui::Ui, plaintext: &mut String, ciphertext: &mut String) {
    if ui.button("Clear").clicked() {
        *plaintext = String::new();
        *ciphertext = String::new();
    }
}

pub fn input_alphabet(ui: &mut egui::Ui, cipher: &mut dyn Cipher) {
    ui.label("Alphabet");
    ui.add(egui::TextEdit::singleline(cipher.input_alphabet()).text_style(TextStyle::Monospace));
}