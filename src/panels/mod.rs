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


fn run_button(ui: &mut egui::Ui, mode: &mut Mode, cipher: &dyn Cipher, plaintext: &mut String, ciphertext: &mut String) {
    if ui.button(RichText::from("RUN").color(Color32::RED)).clicked() {
        run_cipher(mode, cipher, plaintext, ciphertext)
    }
}

fn randomize_button(ui: &mut egui::Ui, cipher: &mut dyn Cipher) {
    let mut rng = ThreadRng::default();
    if ui.button("Randomize").clicked() {
        cipher.randomize(&mut rng)
    }
}

// ENCRYPT/DECRYPT
#[derive(Debug, PartialEq)]
pub enum Mode {
    Encrypt,
    Decrypt,
}

fn mode_selector(ui: &mut egui::Ui, mode: &mut Mode) {
    ui.horizontal(|ui| {
        ui.selectable_value(mode, Mode::Encrypt, "Encrypt");
        ui.selectable_value(mode, Mode::Decrypt, "Decrypt");
    });
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

        ui.label("Plaintext");
        ui.add(egui::TextEdit::multiline(plaintext).hint_text("Plaintext Here").text_style(TextStyle::Monospace));
        ui.add_space(16.0);
        ui.label("Ciphertext");
        ui.add(egui::TextEdit::multiline(ciphertext).hint_text("Ciphertext Here").text_style(TextStyle::Monospace));
    });
}



// Just in case we need to call this independent of the button
fn run_cipher(mode: &mut Mode, cipher: &dyn Cipher, plaintext: &mut String, ciphertext: &mut String) {
    if *mode == Mode::Encrypt {
        match cipher.encrypt(plaintext) {
            Ok(text) => *ciphertext = text,
            Err(e) => *ciphertext = String::from(e),
        }
    } else {
        match cipher.decrypt(ciphertext) {
            Ok(text) => *plaintext = text,
            Err(e) => *plaintext = String::from(e),
        }
    }
}