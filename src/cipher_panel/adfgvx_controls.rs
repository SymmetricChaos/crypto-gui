use eframe::egui::{TextEdit, Ui};
use utils::preset_alphabet::PresetAlphabet;

use super::{View, ViewableCipher, _generic_components::*};
use crate::ciphers::polybius::Adfgvx;

impl ViewableCipher for Adfgvx {}

impl View for Adfgvx {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        randomize_reset(ui, self);
        ui.add_space(16.0);

        ui.label("Select Mode");
        ui.horizontal(|ui| {
            if ui.button("ADFGX").clicked() {
                self.set_alphabet(PresetAlphabet::BasicLatinNoJ)
            };
            if ui.button("ADFGVX").clicked() {
                self.set_alphabet(PresetAlphabet::BasicLatinWithDigits)
            };
        });

        ui.label("Polybius Key Word");
        if control_string(ui, &mut self.polybius.key_word).changed() {
            self.polybius.set_key()
        }
        ui.add_space(16.0);

        ui.label(mono(format!("Grid\n{}", self.polybius)));
        ui.add_space(16.0);

        ui.label("Columnar Key Word");
        ui.add(TextEdit::singleline(self.columnar.control_key()));
    }
}
