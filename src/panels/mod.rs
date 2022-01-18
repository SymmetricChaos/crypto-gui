use eframe::egui::{self, TextStyle, TextBuffer, RichText, Color32};
use rand::prelude::ThreadRng;

use crate::ciphers::Cipher;

pub mod caesar_panel;
pub mod cipher_windows;
pub mod affine_panel;
pub mod substitution_panel;
mod decorder_ring_panel;


// BUTTONS
fn clear_button(ui: &mut egui::Ui, plaintext: &mut String, ciphertext: &mut String) {
    if ui.button("Clear").clicked() {
        *plaintext = String::new();
        *ciphertext = String::new();
    }
}

fn encrypt_button(ui: &mut egui::Ui, cipher: &dyn Cipher, input: &mut String, output: &mut String) {
    if ui.button(RichText::from("ENCRYPT").color(Color32::GREEN)).clicked() {
        match cipher.encrypt(input) {
            Ok(text) => *output = text,
            Err(e) => *output = String::from(e),
        }
    }
}

fn decrypt_button(ui: &mut egui::Ui, cipher: &dyn Cipher, input: &mut String, output: &mut String) {
    if ui.button(RichText::from("DECRYPT").color(Color32::RED)).clicked() {
        match cipher.decrypt(input) {
            Ok(text) => *output = text,
            Err(e) => *output = String::from(e),
        }
    }
}


fn randomize_button(ui: &mut egui::Ui, cipher: &mut dyn Cipher) {
    let mut rng = ThreadRng::default();
    if ui.button("Randomize").clicked() {
        cipher.randomize(&mut rng)
    }
}


fn input_alphabet(ui: &mut egui::Ui, cipher: &mut dyn Cipher) {
    ui.label("Alphabet");
    ui.add(egui::TextEdit::singleline(cipher.input_alphabet()).text_style(TextStyle::Monospace));
}

fn display_panel(ui: &mut egui::Ui, description: &str, plaintext: &mut dyn TextBuffer, ciphertext: &mut dyn TextBuffer) {
    egui::SidePanel::right("caesar_display_panel")
        .default_width(500.0)
        .show_inside(ui, |ui| {

        ui.label(format!{"Description:\n{}",description});

        ui.add_space(16.0);
        ui.separator();
        ui.add_space(16.0);

        ui.label("INPUT TEXT");
        ui.add(egui::TextEdit::multiline(plaintext).hint_text("").text_style(TextStyle::Monospace));
        ui.add_space(16.0);
        ui.label("OUTPUT TEST");
        ui.add(egui::TextEdit::multiline(ciphertext).hint_text("").text_style(TextStyle::Monospace));
    });
}
