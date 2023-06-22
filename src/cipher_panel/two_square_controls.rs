use super::CipherFrame;
use crate::ui_elements::{control_string, mono, randomize_reset};
use ciphers::{playfair::TwoSquare, Cipher};
use egui::Ui;
use rand::{rngs::StdRng, SeedableRng};
use utils::{functions::shuffled_str, preset_alphabet::Alphabet};

pub struct TwoSquareFrame {
    cipher: TwoSquare,
    alphabet_string: String,
    keyword_1: String,
    keyword_2: String,
    spacer_string: String,
}

impl Default for TwoSquareFrame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
            alphabet_string: Alphabet::BasicLatinNoQ.into(),
            keyword_1: Default::default(),
            keyword_2: Default::default(),
            spacer_string: "X".into(),
        }
    }
}

impl CipherFrame for TwoSquareFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        randomize_reset(ui, self);
        ui.add_space(16.0);

        ui.label("Select Alphabet");
        ui.horizontal(|ui| {
            if ui.button("No Q").clicked() {
                self.alphabet_string = Alphabet::BasicLatinNoQ.string();
                self.cipher
                    .assign_keys(&self.keyword_1, &self.keyword_2, &self.alphabet_string)
            };
            if ui.button("No J").clicked() {
                self.alphabet_string = Alphabet::BasicLatinNoJ.string();
                self.cipher
                    .assign_keys(&self.keyword_1, &self.keyword_2, &self.alphabet_string)
            };
            if ui.button("Alphanumeric").clicked() {
                self.alphabet_string = Alphabet::BasicLatinWithDigits.string();
                self.cipher
                    .assign_keys(&self.keyword_1, &self.keyword_2, &self.alphabet_string)
            };
            if ui.button("Base64").clicked() {
                self.alphabet_string = Alphabet::Base64.string();
                self.cipher
                    .assign_keys(&self.keyword_1, &self.keyword_2, &self.alphabet_string)
            };
        });
        ui.add_space(10.0);

        ui.label("Alphabet");
        if control_string(ui, &mut self.alphabet_string).changed() {
            self.cipher
                .assign_keys(&self.keyword_1, &self.keyword_2, &self.alphabet_string)
        }
        ui.add_space(16.0);

        ui.label("Spacer Character\nInserted at end as padding if needed");
        if control_string(ui, &mut self.spacer_string).changed() {
            if self.spacer_string.is_empty() {
                ui.label("defaulting to X");
            } else {
                self.spacer_string = self.spacer_string.chars().next().unwrap().to_string()
            }
            self.cipher.spacer = self.spacer_string.chars().next().unwrap_or('X');
        }
        ui.add_space(16.0);

        ui.label("Keyword 1");
        if control_string(ui, &mut self.keyword_1).changed() {
            self.cipher
                .assign_keys(&self.keyword_1, &self.keyword_2, &self.alphabet_string)
        }

        ui.add_space(16.0);
        ui.label("Keyword 2");
        if control_string(ui, &mut self.keyword_2).changed() {
            self.cipher
                .assign_keys(&self.keyword_1, &self.keyword_2, &self.alphabet_string)
        }

        ui.label(mono(self.cipher.show_square1()));
        ui.add_space(8.0);
        ui.label(mono(self.cipher.show_square2()));
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        self.keyword_1 = shuffled_str(&self.alphabet_string, &mut StdRng::from_entropy());
        self.keyword_2 = shuffled_str(&self.alphabet_string, &mut StdRng::from_entropy());
        self.cipher
            .assign_keys(&self.keyword_1, &self.keyword_2, &self.alphabet_string)
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
