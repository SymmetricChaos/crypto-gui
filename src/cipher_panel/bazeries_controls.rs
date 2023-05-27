use ciphers::{polyalphabetic::Bazeries, Cipher};
use egui::{Slider, Ui};
use rand::{rngs::StdRng, SeedableRng};
use utils::{functions::shuffled_str, preset_alphabet::Alphabet};

use super::{
    CipherFrame,
    _generic_components::{control_string, randomize_reset},
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

        ui.label("Wheels");
        for wheel in &self.cipher.wheels {
            ui.add(egui::Label::new(egui::RichText::from(wheel).monospace()));
        }

        ui.horizontal(|ui| {
            if ui.button("+").clicked() {
                self.cipher.add_wheel()
            }
            if ui.button("-").clicked() {
                self.cipher.del_wheel()
            }
        });
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
