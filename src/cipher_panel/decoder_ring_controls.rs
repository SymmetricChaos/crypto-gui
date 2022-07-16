use crate::ciphers::substitution::DecoderRing;

use super::{View, ViewableCipher, _generic_components::*};
use eframe::egui::{Slider, Ui};

impl ViewableCipher for DecoderRing {}

impl View for DecoderRing {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        randomize_reset(ui, self);
        ui.add_space(16.0);

        input_alphabet(ui, &mut self.control_alphabet());
        ui.add_space(16.0);

        ui.label("Key");
        let alpha_range = 0..=(self.length() - 1);
        ui.add(Slider::new(&mut self.index, alpha_range));
        ui.add_space(16.0);

        ui.horizontal(|ui| {
            if ui.button("Annie").clicked() {
                self.annie();
            }
            if ui.button("Midnight").clicked() {
                self.midnight();
            }
        });
    }
}
