use super::{View, ViewableCipher, _generic_components::*};
use crate::{ciphers::substitution::Affine, math_functions::prime_factors};
use eframe::egui::{Slider, Ui};

impl ViewableCipher for Affine {}

impl View for Affine {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        randomize_reset(ui, self);
        ui.add_space(16.0);

        ui.label("Alphabet");
        if control_string(ui, &mut self.alphabet_string).changed() {
            self.set_alphabet()
        }
        ui.add_space(16.0);

        ui.label("Additive Key");
        let alpha_range = 0..=(self.alphabet_len() - 1);
        ui.add(Slider::new(&mut self.add_key, alpha_range.clone()));
        ui.add_space(16.0);

        ui.label("Multiplicative Key");
        ui.label(format!(
            "Must not be divisible by the following numbers: {:?}",
            prime_factors(self.alphabet_len())
        ));
        let alpha_range = 1..=(self.alphabet_len() - 1);
        ui.add(Slider::new(&mut self.mul_key, alpha_range));
        ui.add_space(16.0);
    }
}
