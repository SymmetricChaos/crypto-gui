use super::CipherFrame;
use super::_generic_components::{control_string, randomize_reset};
use ciphers::traits::Cipher;
use ciphers::transposition::Columnar;
use eframe::egui::Ui;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use utils::preset_alphabet::PresetAlphabet;

pub struct ColumnarFrame {
    cipher: Columnar,
    alphabet_string: String,
    key_string: String,
}

impl Default for ColumnarFrame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
            alphabet_string: PresetAlphabet::BasicLatin.into(),
            key_string: Default::default(),
        }
    }
}

impl CipherFrame for ColumnarFrame {
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
        let mut rng = StdRng::from_entropy();
        let len = self.alphabet_string.chars().count();
        let n_chars = rng.gen_range(6..10);

        self.key_string.clear();
        for _ in 0..n_chars {
            self.key_string.push(
                self.alphabet_string
                    .chars()
                    .nth(rng.gen_range(0..len))
                    .unwrap(),
            )
        }

        self.cipher
            .assign_key(&self.key_string, &self.alphabet_string)
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
