use crate::{
    ciphers::Cipher,
    grid::{str_to_char_grid, Grid},
};
use eframe::{
    egui::{self, Color32, Label, RichText, TextStyle},
    epaint::FontFamily,
};

use super::ViewableCipher;

pub fn encrypt_decrypt(
    ui: &mut egui::Ui,
    cipher: &dyn ViewableCipher,
    input: &mut String,
    output: &mut String,
    errors: &mut String,
) {
    ui.horizontal(|ui| {
        if ui
            .button(RichText::from("ENCRYPT").color(Color32::GOLD))
            .clicked()
        {
            errors.clear();
            match cipher.encrypt(input) {
                Ok(text) => *output = text,
                Err(e) => *errors = e.to_string(),
            }
        };
        if ui
            .button(RichText::from("DECRYPT").color(Color32::GOLD))
            .clicked()
        {
            errors.clear();
            match cipher.decrypt(input) {
                Ok(text) => *output = text,
                Err(e) => *errors = e.to_string(),
            }
        }
    });
}

pub fn randomize_reset(ui: &mut egui::Ui, cipher: &mut dyn Cipher) {
    if ui.button("Randomize").clicked() {
        cipher.randomize()
    }
    if ui.button("Reset").clicked() {
        cipher.reset()
    }
}

pub fn randomize_button(ui: &mut egui::Ui, cipher: &mut dyn Cipher) {
    if ui.button("Randomize\nSettings").clicked() {
        cipher.randomize()
    }
}

pub fn reset_button(ui: &mut egui::Ui, cipher: &mut dyn Cipher) {
    if ui.button("Reset").clicked() {
        cipher.reset()
    }
}

pub fn control_string(ui: &mut egui::Ui, string: &mut String) -> egui::Response {
    ui.add(egui::TextEdit::singleline(string).font(TextStyle::Monospace))
}

pub fn input_alphabet(ui: &mut egui::Ui, alphabet: &mut String) {
    ui.label("Alphabet");
    ui.add(egui::TextEdit::singleline(alphabet).font(TextStyle::Monospace));
}

pub fn text_edit(ui: &mut egui::Ui, text: &mut String) {
    ui.add(egui::TextEdit::singleline(text).font(TextStyle::Monospace));
}

pub fn letter_grid(ui: &mut egui::Ui, n_rows: usize, n_cols: usize, text: &String) {
    let symbols = str_to_char_grid(text, '\0', '\0');
    let grid = Grid::from_cols(symbols, n_rows, n_cols);

    egui::Grid::new("letter_grid").show(ui, |ui| {
        for n in 0..grid.num_rows() {
            ui.spacing_mut().item_spacing.x = 0.0;
            let row = grid.get_row(n);
            for c in row {
                let character = mono(*c.contents().unwrap()); // RichText::from(String::from(*c.contents().unwrap())).monospace();
                ui.add_sized([0.0, 0.0], Label::new(character));
            }
            ui.end_row()
        }
    });
}
// Usable arrows
// ← ↑ → ↓
pub fn mono<T: ToString>(text: T) -> RichText {
    RichText::from(text.to_string()).family(FontFamily::Monospace)
}
