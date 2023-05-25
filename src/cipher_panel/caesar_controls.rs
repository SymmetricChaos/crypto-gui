use ciphers::{substitution::Caesar, Cipher};
use egui::{Slider, Ui};
use utils::preset_alphabet::PresetAlphabet;

use super::{CipherFrame, _generic_components::control_string};

pub struct CaesarFrame {
    cipher: Caesar,
    alphabet_string: String,
}

impl Default for CaesarFrame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
            alphabet_string: PresetAlphabet::BasicLatin.into(),
        }
    }
}

impl CipherFrame for CaesarFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        // randomize_reset(ui, self);
        ui.add_space(16.0);

        ui.label("Alphabet");
        if control_string(ui, &mut self.alphabet_string).changed() {
            self.cipher.assign_alphabet(&self.alphabet_string)
        }
        ui.add_space(16.0);

        ui.label("Key");
        let alpha_range = 0..=(self.cipher.alphabet.len() as i32 - 1);
        ui.add(Slider::new(&mut self.cipher.shift, alpha_range));
        ui.add_space(16.0);
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {}

    fn reset(&mut self) {
        *self = Self::default()
    }
}
