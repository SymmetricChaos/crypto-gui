use eframe::egui::{TextEdit, TextStyle};
use crate::ciphers::{Beaufort, PolyMode};
use super::{View, generic_components::*};


impl View for Beaufort {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, input: &mut String, output: &mut String, errors: &mut String) {
        ui.add_space(16.0);
        input_alphabet(ui, self);
        ui.add_space(16.0);

        ui.label("Key Word");
        ui.add(TextEdit::singleline(&mut self.key_word).text_style(TextStyle::Monospace));

        ui.label("Mode");
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.mode, PolyMode::CylicKey, "Cyclic");
            ui.selectable_value(&mut self.mode, PolyMode::Autokey, "Autokey");
            ui.selectable_value(&mut self.mode, PolyMode::ProgKey, "Progressive Key");
        });

        encrypt_decrypt(ui, self, input, output, errors);
        ui.add_space(16.0);
        randomize_button(ui, self);
    }
}
