use super::CipherFrame;
use crate::ui_elements::{control_string, mono, randomize_reset};
use ciphers::{polybius::Bifid, Cipher};
use egui::{Slider, Ui};
use rand::{thread_rng, Rng};
use utils::{functions::shuffled_str, preset_alphabet::Alphabet};

pub struct BifidFrame {
    cipher: Bifid,
    alphabet_string: String,
    key_string: String,
}

impl Default for BifidFrame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
            alphabet_string: Alphabet::BasicLatinNoQ.into(),
            key_string: Default::default(),
        }
    }
}

impl CipherFrame for BifidFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        randomize_reset(ui, self);
        ui.add_space(16.0);

        let block_size_range = 3..=30;
        ui.label("Block Size");
        ui.add(Slider::new(&mut self.cipher.block_size, block_size_range));

        ui.label("Select Alphabet");
        ui.horizontal(|ui| {
            if ui.button("No C").clicked() {
                self.cipher
                    .polybius
                    .assign_key(&self.key_string, Alphabet::BasicLatinNoC.into());
                self.alphabet_string = Alphabet::BasicLatinNoC.string();
            };
            if ui.button("No J").clicked() {
                self.cipher
                    .polybius
                    .assign_key(&self.key_string, Alphabet::BasicLatinNoJ.into());
                self.alphabet_string = Alphabet::BasicLatinNoJ.string();
            };
            if ui.button("No Q").clicked() {
                self.cipher
                    .polybius
                    .assign_key(&self.key_string, Alphabet::BasicLatinNoQ.into());
                self.alphabet_string = Alphabet::BasicLatinNoQ.string();
            };
            if ui.button("Alphanumeric").clicked() {
                self.cipher
                    .polybius
                    .assign_key(&self.key_string, Alphabet::BasicLatinWithDigits.into());
                self.alphabet_string = Alphabet::BasicLatinWithDigits.string();
            };
            if ui.button("Base64").clicked() {
                self.cipher
                    .polybius
                    .assign_key(&self.key_string, Alphabet::Base64.into());
                self.alphabet_string = Alphabet::Base64.string();
            };
        });

        ui.label("Alphabet");
        if control_string(ui, &mut self.alphabet_string).changed() {
            self.cipher
                .polybius
                .assign_key(&self.key_string, &self.alphabet_string);
        }
        ui.add_space(16.0);

        ui.label("Keyword");
        if control_string(ui, &mut self.key_string).changed() {
            self.cipher
                .polybius
                .assign_key(&self.key_string, &self.alphabet_string)
        }
        ui.add_space(16.0);
        ui.label(mono(format!("Grid\n{}", self.cipher.polybius)));
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        self.cipher.block_size = rng.gen_range(3..=30);
        self.key_string = shuffled_str(&self.alphabet_string, &mut thread_rng());
        self.cipher
            .polybius
            .assign_key(&self.key_string, &self.alphabet_string);
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
