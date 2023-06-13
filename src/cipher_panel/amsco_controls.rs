use super::CipherFrame;

use crate::ui_elements::{control_string, randomize_reset};
use ciphers::traits::Cipher;
use ciphers::transposition::Amsco;
use eframe::egui::Ui;
use rand::{thread_rng, Rng};
use utils::functions::random_sample_replace;
use utils::preset_alphabet::Alphabet;

pub struct AmscoFrame {
    cipher: Amsco,
    alphabet_string: String,
    key_string: String,
}

impl Default for AmscoFrame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
            alphabet_string: Alphabet::BasicLatin.into(),
            key_string: Default::default(),
        }
    }
}

impl CipherFrame for AmscoFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        randomize_reset(ui, self);
        ui.add_space(16.0);

        ui.label("Alphabet");
        if control_string(ui, &mut self.alphabet_string).changed() {
            self.cipher
                .assign_key(&self.key_string, &self.alphabet_string)
        }

        ui.label("Key Word");
        if control_string(ui, &mut self.key_string).changed() {
            self.cipher
                .assign_key(&self.key_string, &self.alphabet_string)
        };
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        let n_chars = rng.gen_range(6..10);

        self.key_string = random_sample_replace(&self.alphabet_string, n_chars, &mut rng);

        self.cipher
            .assign_key(&self.key_string, &self.alphabet_string)
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
