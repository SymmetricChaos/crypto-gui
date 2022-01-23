use eframe::egui::Slider;
use super::View;
use super::generic_components::*;
use crate::math_functions::prime_factors;
use crate::{ciphers::Affine, text_functions::LATIN_UPPER};

pub struct AffineControls {
    cipher: Affine,
}

impl Default for AffineControls {
    fn default() -> Self {
        Self { 
            cipher: Affine::new(0, 1, LATIN_UPPER),
        }
    }
}

impl View for AffineControls {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, input: &mut String, output: &mut String) {
        ui.add_space(16.0);
        input_alphabet(ui, &mut self.cipher);
        ui.add_space(16.0);

        ui.label("Additive Key");
        let alpha_range = 0..=((self.cipher.length()-1));
        ui.add(Slider::new(&mut self.cipher.add_key, alpha_range.clone()));
        ui.add_space(16.0);

        ui.label("Multiplicative Key");
        ui.label(format!("Must not be divisible by the following numbers: {:?}",prime_factors(self.cipher.length())));
        let alpha_range = 1..=((self.cipher.length()-1));
        ui.add(Slider::new(&mut self.cipher.mul_key, alpha_range));
        ui.add_space(16.0);

        // Currently we call this every frame even though we only need to do so when the Multiplicative Key slider is changed
        self.cipher.set_inverse();

        encrypt_decrypt(ui, &mut self.cipher, input, output);
        ui.add_space(16.0);
        randomize_button(ui, &mut self.cipher);
        ui.add_space(16.0);
        clear_button(ui, input, output);
    }
}
