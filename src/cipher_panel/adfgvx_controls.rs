use eframe::egui::{RichText, TextEdit, Ui};
use rand::prelude::StdRng;

use super::{generic_components::*, View};
use crate::{ciphers::ADFGVX, text_aux::PresetAlphabet::*};

impl View for ADFGVX {
    fn ui(&mut self, ui: &mut Ui, rng: &mut StdRng, _errors: &mut String) {
        randomize_reset(ui, self, rng);
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

        ui.label(RichText::new(format!("Grid\n{}", self.polybius)).monospace());
        ui.add_space(16.0);

        ui.label("Columnar Key Word");
        ui.add(TextEdit::singleline(self.columnar.control_key()));
    }
}
