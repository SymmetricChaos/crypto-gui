use super::CipherFrame;
use crate::ui_elements::{string_slider, UiElements};
use ciphers::playfair::FourSquare;
use egui::Ui;
use rand::{rngs::StdRng, SeedableRng};
use utils::{
    preset_alphabet::Alphabet,
    text_functions::{filter_string, shuffled_str},
};

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
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/ciphers/src/playfair/four_square.rs",
        );
        ui.add_space(8.0);

        ui.randomize_reset_cipher(self);
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
            filter_string(&mut self.keyword_1, &self.alphabet_string);
            filter_string(&mut self.keyword_2, &self.alphabet_string);
            self.cipher
                .assign_keys(&self.keyword_1, &self.keyword_2, &self.alphabet_string)
        }
        ui.add_space(16.0);

        ui.subheading("Spacer Character\nInserted at end as padding if needed");
        if string_slider(ui, &self.alphabet_string, &mut self.spacer_position).changed() {
            self.cipher.spacer = self
                .alphabet_string
                .chars()
                .nth(self.spacer_position)
                .unwrap()
        }

        ui.add_space(8.0);
        ui.subheading("Keyword 1");
        if ui.control_string(&mut self.keyword_1).changed() {
            filter_string(&mut self.keyword_1, &self.alphabet_string);
            self.cipher
                .assign_keys(&self.keyword_1, &self.keyword_2, &self.alphabet_string);
        }

        ui.add_space(8.0);
        ui.subheading("Keyword 2");
        if ui.control_string(&mut self.keyword_2).changed() {
            filter_string(&mut self.keyword_2, &self.alphabet_string);
            self.cipher
                .assign_keys(&self.keyword_1, &self.keyword_2, &self.alphabet_string);
        }

        ui.add_space(16.0);
        ui.horizontal(|ui| {
            ui.subheading("Grid");
            ui.copy_to_clipboard(self.cipher.grid_lines());
        });
        ui.mono(self.cipher.grid_lines());
        ui.add_space(16.0);
    }

    fn randomize(&mut self) {
        self.keyword_1 = shuffled_str(&self.alphabet_string, &mut StdRng::from_entropy());
        self.keyword_2 = shuffled_str(&self.alphabet_string, &mut StdRng::from_entropy());
        self.cipher
            .assign_keys(&self.keyword_1, &self.keyword_2, &self.alphabet_string)
    }

    crate::simple_cipher! {}
}
