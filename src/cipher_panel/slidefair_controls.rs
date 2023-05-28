use ciphers::{playfair::Slidefair, Cipher};
use egui::Ui;
use rand::thread_rng;
use utils::{functions::shuffled_str, preset_alphabet::Alphabet};

use crate::egui_aux::mono;

use super::{
    CipherFrame,
    _generic_components::{control_string, randomize_reset},
};

pub struct SlidefairFrame {
    cipher: Slidefair,
    alphabet_string: String,
    key_word_string: String,
    spacer_string: String,
}

impl Default for SlidefairFrame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
            alphabet_string: Alphabet::BasicLatin.into(),
            key_word_string: Default::default(),
            spacer_string: String::from("X"),
        }
    }
}

impl CipherFrame for SlidefairFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        randomize_reset(ui, self);
        ui.add_space(16.0);

        ui.label("Alphabet");
        if control_string(ui, &mut self.alphabet_string).changed() {
            self.cipher.assign_alphabet(&self.alphabet_string)
        }
        ui.add_space(16.0);

        ui.label("Key Word");
        if control_string(ui, &mut self.key_word_string).changed() {
            self.cipher.assign_key(&self.key_word_string)
        }
        ui.add_space(16.0);

        ui.label("Spacer Character\nInserted at end as padding if needed");
        if control_string(ui, &mut self.spacer_string).changed() {
            if self.spacer_string.is_empty() {
                ui.label("defaulting to X");
            } else {
                self.spacer_string = self.spacer_string.chars().next().unwrap().to_string()
            }
            self.cipher.assign_spacer(&self.key_word_string)
        }
        ui.add_space(16.0);

        ui.label("Grid");
        for row in self.cipher.rows() {
            ui.label(mono(row));
        }

        ui.add_space(16.0);
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        self.key_word_string = shuffled_str(&self.alphabet_string, &mut thread_rng());
        self.cipher.assign_key(&self.key_word_string)
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
