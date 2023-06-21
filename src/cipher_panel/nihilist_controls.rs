use super::CipherFrame;
use crate::ui_elements::{control_string, mono, randomize_reset};
use ciphers::{polybius::nihilist::Nihilist, Cipher};
use eframe::egui::Ui;
use rand::thread_rng;
use utils::{
    functions::{random_sample_replace, shuffled_str},
    preset_alphabet::Alphabet,
};

#[derive(Default)]
pub struct NihilistFrame {
    cipher: Nihilist,
    alphabet_string: String,
    key_string: String,
    keyword_string: String,
}

impl NihilistFrame {
    fn assign_keys(&mut self) -> Result<(), ciphers::CipherError> {
        self.cipher.assign_keys(
            &self.key_string,
            &self.keyword_string,
            &self.alphabet_string,
        )
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

        ui.label("Alphabet");
        if control_string(ui, &mut self.alphabet_string).changed() {
            self.assign_keys();
        }

        ui.label("Key Word");
        if control_string(ui, &mut self.key_string).changed() {
            self.assign_keys();
        }
        ui.label(format!("{:?}", self.cipher.keyword_vec()));

        ui.add_space(16.0);
        ui.label("Grid");
        ui.label(mono(self.cipher.polybius.show_grid()));
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        self.key_string = shuffled_str(&self.alphabet_string, &mut thread_rng());
        self.keyword_string = random_sample_replace(&self.alphabet_string, 6, &mut thread_rng());
        self.assign_keys().unwrap();
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
