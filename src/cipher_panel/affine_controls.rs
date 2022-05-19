use super::{generic_components::*, View};
use crate::{ciphers::Affine, math_functions::prime_factors};
use eframe::egui::{Slider, Ui};
use rand::prelude::StdRng;

impl View for Affine {
    fn ui(&mut self, ui: &mut Ui, rng: &mut StdRng, _errors: &mut String) {
        randomize_reset(ui, self, rng);
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
