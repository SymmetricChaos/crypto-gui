use crate::egui_aux::mono;

use super::{
    CipherFrame,
    _generic_components::{control_string, randomize_reset},
};
use ciphers::{polyalphabetic::Alberti, Cipher};
use eframe::egui::{Slider, Ui};
use rand::{rngs::StdRng, Rng, SeedableRng};
use utils::preset_alphabet::PresetAlphabet;

pub struct AlbertiFrame {
    cipher: Alberti,
    fixed_alphabet_string: String,
    moving_alphabet_string: String,
}

impl Default for AlbertiFrame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
            fixed_alphabet_string: String::from(PresetAlphabet::BasicLatin),
            moving_alphabet_string: String::from(PresetAlphabet::BasicLatin).to_ascii_lowercase(),
        }
    }
}

impl CipherFrame for AlbertiFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        randomize_reset(ui, self);
        ui.add_space(16.0);

        ui.label("Fixed Alphabet");
        if control_string(ui, &mut self.fixed_alphabet_string).changed() {
            self.cipher
                .assign_fixed_alphabet(&self.fixed_alphabet_string)
        }

        ui.label("Moving Alphabet");
        if control_string(ui, &mut self.moving_alphabet_string).changed() {
            self.cipher
                .assign_moving_alphabet(&self.moving_alphabet_string)
        }

        ui.label(mono(&self.cipher));

        ui.label("Index");
        let alpha_range = 0..=(self.cipher.alphabet_len() - 1);
        ui.add(Slider::new(
            &mut self.cipher.start_index,
            alpha_range.clone(),
        ));
        ui.add_space(16.0);
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        let length = self.cipher.moving_alphabet.len();
        self.cipher.start_index = StdRng::from_entropy().gen_range(0..length);
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
