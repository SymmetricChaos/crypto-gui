use super::CipherFrame;
use super::_generic_components::control_string;
use ciphers::traits::Cipher;
use ciphers::transposition::Columnar;
use eframe::egui::Ui;

#[derive(Default)]
pub struct ColumnarFrame {
    cipher: Columnar,
    alphabet_string: String,
    key_string: String,
}

impl CipherFrame for ColumnarFrame {
    fn ui(&mut self, ui: &mut Ui, errors: &mut String) {
        // randomize_reset(ui, self);
        ui.add_space(16.0);

        ui.label("Alphabet");
        if control_string(ui, &mut self.alphabet_string).changed() {
            self.cipher.assign_alphabet(&self.alphabet_string)
        }

        ui.label("Key Word");
        if control_string(ui, &mut self.key_string).changed() {
            self.cipher.assign_key(&self.key_string)
        };
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        // let key: String = self
        //     .alphabet
        //     .get_rand_chars_replace(11, &mut get_global_rng())
        //     .iter()
        //     .collect();
        // self.assign_key(&key);
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
