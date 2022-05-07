use super::{generic_components::*, View};
use crate::{ciphers::Bifid, text_aux::PresetAlphabet::*};
use eframe::{
    egui::{RichText, Slider, Ui},
    epaint::Color32,
};
use rand::prelude::StdRng;

impl View for Bifid {
    fn ui(&mut self, ui: &mut Ui, rng: &mut StdRng, _errors: &mut String) {
        randomize_reset(ui, self, rng);
        ui.add_space(16.0);

        let block_size_range = 3..=30;
        ui.label("Block Size");
        ui.add(Slider::new(&mut self.block_size, block_size_range));

        ui.label("Select Alphabet");
        ui.horizontal(|ui| {
            if ui.button("No Q").clicked() {
                self.polybius.set_alphabet(BasicLatinNoQ)
            };
            if ui.button("No J").clicked() {
                self.polybius.set_alphabet(BasicLatinNoJ)
            };
            if ui.button("Alphanumeric").clicked() {
                self.polybius.set_alphabet(BasicLatinWithDigits)
            };
            if ui.button("Base64").clicked() {
                self.polybius.set_alphabet(Base64)
            };
        });

        ui.add_space(10.0);
        ui.label(
            RichText::new(self.polybius.alphabet())
                .monospace()
                .background_color(Color32::BLACK),
        );
        ui.add_space(16.0);

        ui.label("Key Word");
        if control_string(ui, &mut self.polybius.key_word).changed() {
            self.polybius.set_key()
        }
        ui.add_space(16.0);

        ui.label(RichText::new(format!("Grid\n{}", self.polybius)).monospace());
        ui.add_space(16.0);
    }
}
