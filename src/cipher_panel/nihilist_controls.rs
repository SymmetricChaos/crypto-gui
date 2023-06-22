use super::CipherFrame;
use crate::ui_elements::{control_string, mono, randomize_reset};
use ciphers::{polybius::nihilist::Nihilist, Cipher};
use eframe::egui::Ui;
use rand::thread_rng;
use utils::{
    functions::{random_sample_replace, shuffled_str},
    preset_alphabet::Alphabet,
};

pub struct NihilistFrame {
    cipher: Nihilist,
    alphabet_string: String,
    polybius_key_string: String,
    additive_key_string: String,
}

impl Default for NihilistFrame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
            alphabet_string: Alphabet::BasicLatinNoQ.into(),
            polybius_key_string: Default::default(),
            additive_key_string: Default::default(),
        }
    }
}

impl NihilistFrame {
    fn assign_keys(&mut self) {
        // Filter the keys
        self.additive_key_string = self
            .additive_key_string
            .chars()
            .filter(|c| !self.alphabet_string.contains(*c))
            .collect();

        self.polybius_key_string = self
            .polybius_key_string
            .chars()
            .filter(|c| !self.alphabet_string.contains(*c))
            .collect();

        self.cipher
            .assign_keys(
                &self.polybius_key_string,
                &self.additive_key_string,
                &self.alphabet_string,
            )
            .unwrap()
    }
}

impl CipherFrame for NihilistFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        randomize_reset(ui, self);
        ui.add_space(16.0);

        ui.label("Common Alphabets");
        ui.horizontal(|ui| {
            if ui.button("No Q").clicked() {
                self.alphabet_string = Alphabet::BasicLatinNoQ.string();
                self.assign_keys();
            };
            if ui.button("No J").clicked() {
                self.alphabet_string = Alphabet::BasicLatinNoJ.string();
                self.assign_keys();
            };
            if ui.button("Alphanumeric").clicked() {
                self.alphabet_string = Alphabet::BasicLatinWithDigits.string();
                self.assign_keys();
            };
            if ui.button("Base64").clicked() {
                self.alphabet_string = Alphabet::Base64.string();
                self.assign_keys();
            };
        });
        ui.add_space(8.0);

        ui.label("Alphabet");
        if control_string(ui, &mut self.alphabet_string).changed() {
            self.assign_keys();
        }
        ui.add_space(8.0);

        ui.label("Polybius Keyword");
        if control_string(ui, &mut self.polybius_key_string).changed() {
            self.assign_keys();
        }
        ui.add_space(8.0);

        ui.label("Additive Keyword");
        if control_string(ui, &mut self.additive_key_string).changed() {
            self.assign_keys();
        }
        if !self.cipher.keyword_vec().is_empty() {
            ui.label(format!("{:?}", self.cipher.keyword_vec()));
        }

        ui.add_space(16.0);
        ui.label("Grid");
        ui.label(mono(self.cipher.polybius.show_grid()));
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        self.polybius_key_string = shuffled_str(&self.alphabet_string, &mut thread_rng());
        self.additive_key_string =
            random_sample_replace(&self.alphabet_string, 6, &mut thread_rng());
        self.assign_keys();
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
