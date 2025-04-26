use ciphers::substitution::Affine;
use eframe::egui::{Slider, Ui};
use rand::{thread_rng, Rng};
use utils::{
    math_functions::{mul_inv, prime_factors},
    preset_alphabet::Alphabet,
};

use crate::ui_elements::UiElements;

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
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/ciphers/src/substitution/affine.rs",
        );
        ui.add_space(8.0);

        ui.randomize_reset_cipher(self);
        ui.add_space(8.0);

        ui.group(|ui| {
            ui.subheading("Common Alphabets");
            ui.horizontal(|ui| {
                for alphabet in [
                    Alphabet::BasicLatin,
                    Alphabet::Alphanumeric,
                    Alphabet::Ascii94,
                    Alphabet::Base64,
                ] {
                    if ui.button(alphabet.name()).clicked() {
                        self.alphabet_string = alphabet.into();
                        self.cipher.assign_alphabet(&self.alphabet_string)
                    }
                }
            });
        });
        ui.add_space(8.0);

        ui.subheading("Alphabet");
        if ui.control_string(&mut self.alphabet_string).lost_focus() {
            self.cipher.assign_alphabet(&self.alphabet_string)
        }
        ui.add_space(8.0);

        let slider_len = self.cipher.alphabet_len() - 1;
        ui.subheading("Additive Key");
        ui.add(Slider::new(&mut self.cipher.add_key, 0..=slider_len));
        ui.add_space(8.0);

        ui.subheading("Multiplicative Key");
        ui.label(format!(
            "Must not be divisible by the following numbers: {:?}",
            prime_factors(self.cipher.alphabet_len())
        ));
        ui.add(Slider::new(&mut self.cipher.mul_key, 1..=slider_len));
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
            if mul_inv(&mul, &self.cipher.alphabet_len()).is_some() {
                self.cipher.mul_key = mul;
                break;
            };
        }
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
