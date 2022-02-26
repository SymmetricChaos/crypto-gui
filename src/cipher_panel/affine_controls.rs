use eframe::egui::Slider;
use super::View;
use super::generic_components::*;
use crate::math_functions::prime_factors;
use crate::ciphers::Affine;


impl View for Affine {
    fn ui(&mut self, ui: &mut eframe::egui::Ui) {
        ui.add_space(16.0);
        input_alphabet(ui, self);
        ui.add_space(16.0);

        ui.label("Additive Key");
        let alpha_range = 0..=((self.length()-1));
        ui.add(Slider::new(&mut self.add_key, alpha_range.clone()));
        ui.add_space(16.0);

        ui.label("Multiplicative Key");
        ui.label(format!("Must not be divisible by the following numbers: {:?}",prime_factors(self.length())));
        let alpha_range = 1..=((self.length()-1));
        ui.add(Slider::new(&mut self.mul_key, alpha_range));
        ui.add_space(16.0);

        randomize_button(ui, self);
    }
}
