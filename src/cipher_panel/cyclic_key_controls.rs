use eframe::egui::TextEdit;
use eframe::egui::TextStyle;

use super::View;
use super::generic_components::*;
use crate::ciphers::{CyclicKey,PolyMode};


impl View for CyclicKey {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, input: &mut String, output: &mut String, errors: &mut String) {
        ui.add_space(16.0);
        input_alphabet(ui, self);
        ui.add_space(16.0);

        ui.label("Key Word");
        ui.add(TextEdit::singleline(&mut self.key_word).text_style(TextStyle::Monospace));

        ui.label("Mode");
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.mode, PolyMode::Vigenere, "Vigenere");
            ui.selectable_value(&mut self.mode, PolyMode::Beaufort, "Beaufort");
        });

        encrypt_decrypt(ui, self, input, output, errors);
        ui.add_space(16.0);
        randomize_button(ui, self);
    }
}
