use super::CipherFrame;
use crate::ui_elements::{control_string, mono, randomize_reset};
use ciphers::{polybius::PolybiusSquare, Cipher};
use eframe::egui::Ui;
use rand::thread_rng;
use utils::{functions::shuffled_str, preset_alphabet::Alphabet};

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

        ui.label("Common Alphabets");
        ui.horizontal(|ui| {
            if ui.button("No Q").clicked() {
                self.alphabet_string = Alphabet::BasicLatinNoQ.string();
                self.cipher
                    .assign_key(&self.key_string, &self.alphabet_string)
            };
            if ui.button("No J").clicked() {
                self.alphabet_string = Alphabet::BasicLatinNoJ.string();
                self.cipher
                    .assign_key(&self.key_string, &self.alphabet_string)
            };
            if ui.button("Alphanumeric").clicked() {
                self.alphabet_string = Alphabet::BasicLatinWithDigits.string();
                self.cipher
                    .assign_key(&self.key_string, &self.alphabet_string)
            };
            if ui.button("Base64").clicked() {
                self.alphabet_string = Alphabet::Base64.string();
                self.cipher
                    .assign_key(&self.key_string, &self.alphabet_string)
            };
        });

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
