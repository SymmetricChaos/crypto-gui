use eframe::egui::{Color32, RichText, Ui};

use super::{View, ViewableCipher, _generic_components::*};
use crate::{ciphers::polybius::PolybiusSquare, egui_aux::mono, text_aux::PresetAlphabet::*};

impl ViewableCipher for PolybiusSquare {}

impl View for PolybiusSquare {
    fn ui(&mut self, ui: &mut Ui, errors: &mut String) {
        randomize_reset(ui, self);
        ui.add_space(16.0);

        ui.label("Common Latin Alphabets");
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

        ui.label("Alphabet");
        if control_string(ui, &mut self.alphabet_string).changed() {
            match self.set_alphabet() {
                Ok(_) => (),
                Err(e) => *errors = e.to_string(),
            }
        }

        ui.add_space(10.0);
        ui.label(
            RichText::new(&self.alphabet_string)
                .monospace()
                .background_color(Color32::BLACK),
        );
        ui.add_space(16.0);

        ui.label("Key Word");
        if control_string(ui, &mut self.key_word).changed() {
            self.set_key()
        }

        ui.add_space(16.0);
        ui.label("Labels");
        if control_string(ui, &mut self.labels_string).changed() {
            self.set_labels();
        }

        ui.add_space(16.0);
        ui.label("Grid");
        mono(ui, &self.show_grid(), None);
    }
}
