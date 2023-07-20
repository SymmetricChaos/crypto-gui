use super::CipherFrame;
use crate::ui_elements::UiElements;
use ciphers::{playfair::TwoSquare, Cipher};
use egui::Ui;
use rand::{rngs::StdRng, SeedableRng};
use utils::{
    preset_alphabet::Alphabet,
    text_functions::{filter_string, shuffled_str},
};

pub struct TwoSquareFrame {
    cipher: TwoSquare,
    alphabet_string: String,
    keyword_1: String,
    keyword_2: String,
    spacer_position: usize,
}

impl Default for TwoSquareFrame {
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

impl CipherFrame for TwoSquareFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        ui.randomize_reset(self);
        ui.add_space(16.0);

        ui.group(|ui| {
            ui.subheading("Common Alphabets");
            ui.horizontal(|ui| {
                for alphabet in [
                    Alphabet::Alphanumeric,
                    Alphabet::BasicLatinNoC,
                    Alphabet::BasicLatinNoJ,
                    Alphabet::BasicLatinNoQ,
                    Alphabet::Base64,
                ] {
                    if ui.button(alphabet.name()).clicked() {
                        self.alphabet_string = alphabet.into();
                        filter_string(&mut self.keyword_1, &self.alphabet_string);
                        filter_string(&mut self.keyword_2, &self.alphabet_string);
                        self.cipher.assign_keys(
                            &self.keyword_1,
                            &self.keyword_2,
                            &self.alphabet_string,
                        )
                    }
                }
            });
        });

        ui.add_space(10.0);

        ui.subheading("Alphabet");
        if ui.control_string(&mut self.alphabet_string).changed() {
            self.cipher
                .assign_keys(&self.keyword_1, &self.keyword_2, &self.alphabet_string)
        }
        ui.add_space(16.0);

        ui.subheading("Spacer Character");
        ui.label("Inserted at the end if needed.");
        if ui
            .string_slider(&self.alphabet_string, &mut self.spacer_position)
            .changed()
        {
            self.cipher.spacer = self
                .alphabet_string
                .chars()
                .nth(self.spacer_position)
                .unwrap()
        }
        ui.add_space(16.0);

        ui.subheading("Keyword 1");
        if ui.control_string(&mut self.keyword_1).changed() {
            self.cipher
                .assign_keys(&self.keyword_1, &self.keyword_2, &self.alphabet_string)
        }

        ui.add_space(8.0);
        ui.subheading("Keyword 2");
        if ui.control_string(&mut self.keyword_2).changed() {
            self.cipher
                .assign_keys(&self.keyword_1, &self.keyword_2, &self.alphabet_string)
        }

        ui.horizontal(|ui| {
            ui.subheading("Grids");
            if ui.button("📋").on_hover_text("Copy to Clipboard").clicked() {
                ui.output_mut(|o| {
                    o.copied_text = format!(
                        "{}\n\n{}",
                        self.cipher.show_square1(),
                        self.cipher.show_square2()
                    )
                })
            }
        });
        ui.mono(self.cipher.show_square1());
        ui.add_space(8.0);
        ui.mono(self.cipher.show_square2());
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
