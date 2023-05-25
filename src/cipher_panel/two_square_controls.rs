use ciphers::{playfair::TwoSquare, Cipher};
use egui::Ui;
use rand::{rngs::StdRng, SeedableRng};
use utils::{functions::shuffled_str, preset_alphabet::PresetAlphabet};

use crate::egui_aux::mono;

use super::{
    CipherFrame,
    _generic_components::{control_string, randomize_reset},
};

#[derive(Default)]
pub struct TwoSquareFrame {
    cipher: TwoSquare,
    alphabet_string: String,
    key_word_1: String,
    key_word_2: String,
}

impl CipherFrame for TwoSquareFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        randomize_reset(ui, self);
        ui.add_space(16.0);

        ui.label("Select Alphabet");
        ui.horizontal(|ui| {
            if ui.button("No Q").clicked() {
                self.alphabet_string = PresetAlphabet::BasicLatinNoQ.string();
                self.cipher
                    .assign_keys(&self.key_word_1, &self.key_word_2, &self.alphabet_string)
            };
            if ui.button("No J").clicked() {
                self.alphabet_string = PresetAlphabet::BasicLatinNoJ.string();
                self.cipher
                    .assign_keys(&self.key_word_1, &self.key_word_2, &self.alphabet_string)
            };
            if ui.button("Alphanumeric").clicked() {
                self.alphabet_string = PresetAlphabet::BasicLatinWithDigits.string();
                self.cipher
                    .assign_keys(&self.key_word_1, &self.key_word_2, &self.alphabet_string)
            };
            if ui.button("Base64").clicked() {
                self.alphabet_string = PresetAlphabet::Base64.string();
                self.cipher
                    .assign_keys(&self.key_word_1, &self.key_word_2, &self.alphabet_string)
            };
        });
        ui.add_space(10.0);

        ui.label("Alphabet");
        if control_string(ui, &mut self.alphabet_string).changed() {
            self.cipher
                .assign_keys(&self.key_word_1, &self.key_word_2, &self.alphabet_string)
        }
        ui.add_space(16.0);

        ui.label("Key Word 1");
        if control_string(ui, &mut self.key_word_1).changed() {
            self.cipher
                .assign_keys(&self.key_word_1, &self.key_word_2, &self.alphabet_string)
        }

        ui.add_space(16.0);
        ui.label("Key Word 2");
        if control_string(ui, &mut self.key_word_2).changed() {
            self.cipher
                .assign_keys(&self.key_word_1, &self.key_word_2, &self.alphabet_string)
        }

        ui.label(mono(self.cipher.show_square1()));
        ui.add_space(8.0);
        ui.label(mono(self.cipher.show_square2()));

        // ui.label(RichText::new(format!("Grid\n{}", self)).monospace());
        // ui.add_space(16.0);

        //(ui, self.grid_side_len(), self.grid_side_len(), self.get_input_alphabet())
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        self.key_word_1 = shuffled_str(&self.alphabet_string, &mut StdRng::from_entropy());
        self.key_word_2 = shuffled_str(&self.alphabet_string, &mut StdRng::from_entropy());
        self.cipher
            .assign_keys(&self.key_word_1, &self.key_word_2, &self.alphabet_string)
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
