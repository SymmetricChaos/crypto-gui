use super::CipherFrame;
use crate::ui_elements::{control_string, mono, randomize_reset, subheading};
use ciphers::{polybius::PolybiusSquare, Cipher};
use eframe::egui::Ui;
use rand::thread_rng;
use utils::{
    preset_alphabet::Alphabet,
    text_functions::{filter_string, shuffled_str},
};

pub struct PolybiusSquareFrame {
    cipher: PolybiusSquare,
    alphabet_string: String,
    key_string: String,
    labels_string: String,
}

impl Default for PolybiusSquareFrame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
            alphabet_string: Alphabet::BasicLatinNoQ.into(),
            key_string: Default::default(),
            labels_string: Alphabet::Digits1.into(),
        }
    }
}

impl CipherFrame for PolybiusSquareFrame {
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
                        filter_string(&mut self.key_string, &self.alphabet_string);
                        self.cipher
                            .assign_key(&self.key_string, &self.alphabet_string)
                    }
                }
            });
        });

        ui.add_space(10.0);

        ui.label("Alphabet");
        if control_string(ui, &mut self.alphabet_string).changed() {
            self.cipher
                .assign_key(&self.key_string, &self.alphabet_string);
        }
        ui.add_space(16.0);

        ui.checkbox(&mut self.cipher.spaced, "Use Spaces")
            .on_hover_text("Insert spaces between the pairs of symbols");

        ui.label("Keyword");
        if control_string(ui, &mut self.key_string).changed() {
            filter_string(&mut self.key_string, &self.alphabet_string);
            self.cipher
                .assign_key(&self.key_string, &self.alphabet_string)
        }

        ui.add_space(16.0);
        ui.label("Labels");
        if control_string(ui, &mut self.labels_string).changed() {
            self.cipher.assign_labels(&self.labels_string)
        }

        ui.add_space(16.0);
        ui.label("Grid");
        ui.label(mono(self.cipher.show_grid()));
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        self.key_string = shuffled_str(&self.alphabet_string, &mut thread_rng());
        self.cipher
            .assign_key(&self.key_string, &self.alphabet_string);
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
