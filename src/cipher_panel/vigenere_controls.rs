use eframe::egui::TextEdit;
use eframe::egui::TextStyle;

use super::View;
use super::generic_components::*;
use crate::ciphers::Vigenere;
use crate::ciphers::vigenere::VigenereMode;


impl View for Vigenere {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, input: &mut String, output: &mut String) {
        ui.add_space(16.0);
        input_alphabet(ui, self);
        ui.add_space(16.0);

        ui.label("Key Word");
        ui.add(TextEdit::singleline(&mut self.key_word).text_style(TextStyle::Monospace));

        ui.label("Mode");
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.mode, VigenereMode::Standard, "Standard");
            ui.selectable_value(&mut self.mode, VigenereMode::Autokey, "Autokey");
        });

        encrypt_decrypt(ui, self, input, output);
        ui.add_space(16.0);
        randomize_button(ui, self);
        ui.add_space(16.0);
        clear_button(ui, input, output);
    }
}
