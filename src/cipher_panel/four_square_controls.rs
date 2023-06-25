use super::CipherFrame;
use crate::ui_elements::{control_string, mono, randomize_reset, string_slider};
use ciphers::{playfair::FourSquare, Cipher};
use egui::Ui;
use rand::{rngs::StdRng, SeedableRng};
use utils::{functions::shuffled_str, preset_alphabet::Alphabet};

pub struct FourSquareFrame {
    cipher: FourSquare,
    alphabet_string: String,
    keyword_1: String,
    keyword_2: String,
    spacer_position: usize,
}

impl Default for FourSquareFrame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
            alphabet_string: Alphabet::BasicLatinNoQ.into(),
            keyword_1: Default::default(),
            keyword_2: Default::default(),
            spacer_position: 23,
        }
    }
}

impl CipherFrame for FourSquareFrame {
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
        if string_slider(ui, &self.alphabet_string, &mut self.spacer_position).changed() {
            self.cipher.spacer = self
                .alphabet_string
                .chars()
                .nth(self.spacer_position)
                .unwrap()
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

        ui.horizontal(|ui| {
            ui.label("Grid");
            if ui.button("📋").on_hover_text("Copy to Clipboard").clicked() {
                ui.output_mut(|o| o.copied_text = self.cipher.grid_lines())
            }
        });
        ui.label(mono(self.cipher.grid_lines()));
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
