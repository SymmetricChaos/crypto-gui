use eframe::egui::{Color32, RichText, Ui};
use rand::prelude::StdRng;

use super::{generic_components::*, View};
use crate::{ciphers::Polybius, text_aux::PresetAlphabet::*};

impl View for Polybius {
    fn ui(&mut self, ui: &mut Ui, rng: &mut StdRng, _errors: &mut String) {
        randomize_reset(ui, self, rng);
        ui.add_space(16.0);

        ui.label("Select Alphabet");
        ui.horizontal(|ui| {
            if ui.button("No Q").clicked() {
                self.set_alphabet(BasicLatinNoQ)
            };
            if ui.button("No J").clicked() {
                self.set_alphabet(BasicLatinNoJ)
            };
            if ui.button("Alphanumeric").clicked() {
                self.set_alphabet(BasicLatinWithDigits)
            };
            if ui.button("Base64").clicked() {
                self.set_alphabet(Base64)
            };
        });

        ui.add_space(10.0);
        ui.label(
            RichText::new(self.alphabet())
                .monospace()
                .background_color(Color32::BLACK),
        );
        ui.add_space(16.0);

        ui.label("Key Word");
        if control_string(ui, &mut self.key_word).changed() {
            self.set_key()
        }
        ui.add_space(16.0);

        ui.label(RichText::new(format!("Grid\n{}", self)).monospace());
        ui.add_space(16.0);
    }
}
