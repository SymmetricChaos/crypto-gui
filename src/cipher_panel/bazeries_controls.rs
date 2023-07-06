use super::CipherFrame;
use crate::ui_elements::{control_string, randomize_reset};
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
        randomize_reset(ui, self);
        ui.add_space(16.0);

        ui.label("Alphabet");
        if control_string(ui, &mut self.alphabet_string).changed() {
            self.cipher.assign_alphabet(&self.alphabet_string)
        }
        ui.add_space(16.0);

        ui.label("Offset");
        let alpha_range = 0..=(self.cipher.alphabet_len());
        ui.add(Slider::new(&mut self.cipher.offset, alpha_range));
        ui.add_space(16.0);

        ui.horizontal(|ui| {
            ui.label("Wheels");
            if ui.button("+").clicked() {
                self.cipher.add_wheel()
            }
            if ui.button("-").clicked() {
                self.cipher.del_wheel()
            }
        });
        for wheel in self.cipher.wheels.iter_mut() {
            if control_string(ui, wheel).changed() {
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
