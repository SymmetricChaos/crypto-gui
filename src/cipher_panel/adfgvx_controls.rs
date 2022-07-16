use eframe::egui::{TextEdit, Ui};

use super::{View, ViewableCipher, _generic_components::*};
use crate::{ciphers::polybius::Adfgvx, text_aux::PresetAlphabet::*};

impl ViewableCipher for Adfgvx {}

impl View for Adfgvx {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        randomize_reset(ui, self);
        ui.add_space(16.0);

        ui.label("Select Mode");
        ui.horizontal(|ui| {
            if ui.button("ADFGX").clicked() {
                self.set_alphabet(BasicLatinNoJ)
            };
            if ui.button("ADFGVX").clicked() {
                self.set_alphabet(BasicLatinWithDigits)
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
