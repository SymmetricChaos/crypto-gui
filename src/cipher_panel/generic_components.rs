use eframe::egui::{self, RichText, Color32, TextStyle, Label};
use rand::prelude::StdRng;
use crate::{ciphers::Cipher, grid::Grid};

pub fn encrypt_decrypt(ui: &mut egui::Ui, cipher: &dyn Cipher, input: &mut String, output: &mut String, errors: &mut String) {
    ui.horizontal(|ui| {
        if ui.button(RichText::from("ENCRYPT").color(Color32::GOLD)).clicked() {
            errors.clear();
            match cipher.encrypt(input) {
                Ok(text) => *output = text,
                Err(e) => *errors = e.to_string(),
            }
        };
        if ui.button(RichText::from("DECRYPT").color(Color32::GOLD)).clicked() {
            errors.clear();
            match cipher.decrypt(input) {
                Ok(text) => *output = text,
                Err(e) => *errors = e.to_string(),
            }
        }
    });
}


pub fn randomize_reset(ui: &mut egui::Ui, cipher: &mut dyn Cipher, rng: &mut StdRng) {
    if ui.button("Randomize").clicked() {
        cipher.randomize(rng)
    }
    if ui.button("Reset").clicked() {
        cipher.reset()
    }
}

pub fn randomize_button(ui: &mut egui::Ui, cipher: &mut dyn Cipher, rng: &mut StdRng) {
    if ui.button("Randomize\nSettings").clicked() {
        cipher.randomize(rng)
    }
}

pub fn reset_button(ui: &mut egui::Ui, cipher: &mut dyn Cipher) {
    if ui.button("Reset").clicked() {
        cipher.reset()
    }
}

pub fn input_alphabet(ui: &mut egui::Ui, alphabet: &mut String) {
    ui.label("Alphabet");
    ui.add(egui::TextEdit::singleline(alphabet).font(TextStyle::Monospace)).on_hover_text("You can change this alphabet to whatever you like.");
}

pub fn control_text_edit(ui: &mut egui::Ui, text: &mut String) {
    ui.add(egui::TextEdit::singleline(text).font(TextStyle::Monospace));
}

pub fn letter_grid(ui: &mut egui::Ui, n_rows: usize, n_cols: usize, text: &String) {
    let grid = Grid::from_rows(text, n_rows, n_cols, '\0', '\0');
    egui::Grid::new("letter_grid").show(ui, |ui| {
        for n in 0..grid.num_rows() {
            ui.spacing_mut().item_spacing.x = 0.0;
            let row = grid.get_row(n);
            for c in row {
                let character = RichText::from(String::from(*c)).monospace();
                ui.add_sized([0.0, 0.0], Label::new(character));
            }
            ui.end_row()
        }
        
    });
}