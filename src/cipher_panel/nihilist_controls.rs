use super::CipherFrame;
use crate::ui_elements::{control_string, mono, randomize_reset, subheading};
use ciphers::{polybius::nihilist::Nihilist, Cipher};
use eframe::egui::Ui;
use rand::{thread_rng, Rng};
use utils::{
    preset_alphabet::Alphabet,
    text_functions::{filter_string, random_sample_replace, shuffled_str},
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
        filter_string(&mut self.additive_key_string, &self.alphabet_string);
        filter_string(&mut self.polybius_key_string, &self.alphabet_string);

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

        ui.group(|ui| {
            ui.label(subheading("Common Alphabets"));
            ui.horizontal(|ui| {
                for (name, alphabet) in [
                    ("No C", Alphabet::BasicLatinNoC),
                    ("No J", Alphabet::BasicLatinNoJ),
                    ("No Q", Alphabet::BasicLatinNoQ),
                    ("Alphanumeric", Alphabet::BasicLatinWithDigits),
                    ("Base64", Alphabet::Base64),
                ] {
                    if ui.button(name).clicked() {
                        self.alphabet_string = alphabet.into();
                        self.assign_keys();
                    }
                }
            });
        });

        ui.add_space(10.0);

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
        let mut rng = thread_rng();
        self.polybius_key_string = shuffled_str(&self.alphabet_string, &mut rng);
        self.additive_key_string =
            random_sample_replace(&self.alphabet_string, rng.gen_range(3..12), &mut rng);
        self.assign_keys();
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
