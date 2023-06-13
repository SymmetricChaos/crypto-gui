use ciphers::substitution::Affine;
use eframe::egui::{Slider, Ui};
use rand::{thread_rng, Rng};
use utils::{
    math_functions::{mul_inv, prime_factors},
    preset_alphabet::Alphabet,
};

use crate::ui_elements::{control_string, randomize_reset};

use super::CipherFrame;

pub struct AffineFrame {
    cipher: Affine,
    alphabet_string: String,
}

impl Default for AffineFrame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
            alphabet_string: String::from(Alphabet::BasicLatin),
        }
    }
}

impl CipherFrame for AffineFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        randomize_reset(ui, self);
        ui.add_space(16.0);

        ui.label("Alphabet");
        if control_string(ui, &mut self.alphabet_string).changed() {
            self.cipher.assign_alphabet(&self.alphabet_string)
        }
        ui.add_space(16.0);

        ui.label("Additive Key");
        let alpha_range = 0..=(self.cipher.alphabet_len() - 1);
        ui.add(Slider::new(&mut self.cipher.add_key, alpha_range.clone()));
        ui.add_space(16.0);

        ui.label("Multiplicative Key");
        ui.label(format!(
            "Must not be divisible by the following numbers: {:?}",
            prime_factors(self.cipher.alphabet_len())
        ));
        let alpha_range = 1..=(self.cipher.alphabet_len() - 1);
        ui.add(Slider::new(&mut self.cipher.mul_key, alpha_range));
        ui.add_space(16.0);
    }

    fn cipher(&self) -> &dyn ciphers::Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        let length = self.cipher.alphabet_len();
        self.cipher.add_key = rng.gen_range(0..length);
        loop {
            let mul = rng.gen_range(1..length);
            if mul_inv(mul, self.cipher.alphabet_len()).is_some() {
                self.cipher.mul_key = mul;
                break;
            };
        }
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
