use eframe::egui::{Color32, RichText, Ui};

use super::{View, ViewableCipher, _generic_components::*};
use crate::{ciphers::playfair::TwoSquare, egui_aux::mono, text_aux::PresetAlphabet::*};

impl ViewableCipher for TwoSquare {}

impl View for TwoSquare {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        randomize_reset(ui, self);
        ui.add_space(16.0);

        ui.label("Select Alphabet");
        ui.horizontal(|ui| {
            if ui.button("No Q").clicked() {
                self.assign_alphabet(BasicLatinNoQ)
            };
            if ui.button("No J").clicked() {
                self.assign_alphabet(BasicLatinNoJ)
            };
            if ui.button("Alphanumeric").clicked() {
                self.assign_alphabet(BasicLatinWithDigits)
            };
            if ui.button("Base64").clicked() {
                self.assign_alphabet(Base64)
            };
        });
        ui.add_space(10.0);

        ui.label(
            RichText::new(&self.alphabet.to_string())
                .monospace()
                .background_color(Color32::BLACK),
        );
        ui.add_space(16.0);

        ui.label("Key Word #1");
        if control_string(ui, &mut self.key_word1).changed() {
            self.set_key1()
        }

        ui.label("Key Word #2");
        if control_string(ui, &mut self.key_word2).changed() {
            self.set_key2()
        }

        mono(ui, &self.show_square1(), None);
        ui.add_space(8.0);
        mono(ui, &self.show_square2(), None);

        // ui.label(RichText::new(format!("Grid\n{}", self)).monospace());
        // ui.add_space(16.0);

        //(ui, self.grid_side_len(), self.grid_side_len(), self.get_input_alphabet())
    }
}
