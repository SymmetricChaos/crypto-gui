use ciphers::{playfair::FourSquare, Cipher};
use egui::{Color32, Ui};
use rand::{rngs::StdRng, SeedableRng};
use utils::{functions::shuffled_str, preset_alphabet::PresetAlphabet};

use crate::egui_aux::mono;

use super::{
    CipherFrame,
    _generic_components::{control_string, randomize_reset},
};

pub struct FourSquareFrame {
    cipher: FourSquare,
    alphabet_string: String,
    key_word_1: String,
    key_word_2: String,
}

impl Default for FourSquareFrame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
            alphabet_string: PresetAlphabet::BasicLatinNoQ.into(),
            key_word_1: Default::default(),
            key_word_2: Default::default(),
        }
    }
}

impl CipherFrame for FourSquareFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        randomize_reset(ui, self);
        ui.add_space(16.0);

        ui.label("Select Alphabet");
        ui.horizontal(|ui| {
            if ui.button("No Q").clicked() {
                self.cipher.pick_alphabet(PresetAlphabet::BasicLatinNoQ);
                self.alphabet_string = PresetAlphabet::BasicLatinNoQ.string();
                self.cipher.assign_key1(&self.key_word_1);
                self.cipher.assign_key1(&self.key_word_2);
            };
            if ui.button("No J").clicked() {
                self.cipher.pick_alphabet(PresetAlphabet::BasicLatinNoJ);
                self.alphabet_string = PresetAlphabet::BasicLatinNoJ.string();
                self.cipher.assign_key1(&self.key_word_1);
                self.cipher.assign_key1(&self.key_word_2);
            };
            if ui.button("Alphanumeric").clicked() {
                self.cipher
                    .pick_alphabet(PresetAlphabet::BasicLatinWithDigits);
                self.alphabet_string = PresetAlphabet::BasicLatinWithDigits.string();
                self.cipher.assign_key1(&self.key_word_1);
                self.cipher.assign_key1(&self.key_word_2);
            };
            if ui.button("Base64").clicked() {
                self.cipher.pick_alphabet(PresetAlphabet::Base64);
                self.alphabet_string = PresetAlphabet::Base64.string();
                self.cipher.assign_key1(&self.key_word_1);
                self.cipher.assign_key1(&self.key_word_2);
            };
        });
        ui.add_space(10.0);

        // False alphabet display
        ui.label(mono(&self.alphabet_string).background_color(Color32::BLACK));
        ui.add_space(16.0);

        ui.label("Key Word 1");
        if control_string(ui, &mut self.key_word_1).changed() {
            self.cipher.assign_key1(&self.key_word_1)
        }

        ui.add_space(16.0);
        ui.label("Key Word 2");
        if control_string(ui, &mut self.key_word_2).changed() {
            self.cipher.assign_key1(&self.key_word_2)
        }

        // Need a better method for displaying the grids
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        self.key_word_1 = shuffled_str(&self.alphabet_string, &mut StdRng::from_entropy());
        self.key_word_2 = shuffled_str(&self.alphabet_string, &mut StdRng::from_entropy());
        self.cipher.assign_key1(&self.key_word_1);
        self.cipher.assign_key1(&self.key_word_2);
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
