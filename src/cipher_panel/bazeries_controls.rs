use super::CipherFrame;
use crate::ui_elements::UiElements;
use ciphers::{polyalphabetic::Bazeries, Cipher};
use egui::{Slider, Ui};
use rand::{rngs::StdRng, SeedableRng};
use utils::{
    preset_alphabet::Alphabet,
    text_functions::{filter_string, shuffled_str},
};

pub struct BazeriesFrame {
    cipher: Bazeries,
    alphabet_string: String,
}

impl Default for BazeriesFrame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
            alphabet_string: Alphabet::BasicLatin.into(),
        }
    }
}

impl CipherFrame for BazeriesFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        ui.randomize_reset(self);
        ui.add_space(16.0);

        ui.subheading("Alphabet");
        if ui.control_string(&mut self.alphabet_string).changed() {
            self.cipher.assign_alphabet(&self.alphabet_string)
        }
        ui.add_space(16.0);

        ui.subheading("Offset");
        let alpha_range = 0..=(self.cipher.alphabet_len());
        ui.add(Slider::new(&mut self.cipher.offset, alpha_range));
        ui.add_space(16.0);

        ui.horizontal(|ui| {
            ui.subheading("Wheels");
            if ui.button("+").clicked() {
                self.cipher.add_wheel()
            }
            if ui.button("-").clicked() {
                self.cipher.del_wheel()
            }
        });
        for wheel in self.cipher.wheels.iter_mut() {
            if ui.control_string(wheel).changed() {
                filter_string(wheel, &self.alphabet_string)
            }
        }
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        let mut rng = StdRng::from_entropy();
        for wheel in self.cipher.wheels.iter_mut() {
            *wheel = shuffled_str(&self.alphabet_string, &mut rng);
        }
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
