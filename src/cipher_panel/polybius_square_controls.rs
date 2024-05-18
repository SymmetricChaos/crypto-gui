use super::CipherFrame;
use crate::ui_elements::UiElements;
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
            labels_string: String::from("123456789"),
        }
    }
}

impl CipherFrame for PolybiusSquareFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/ciphers/src/polybius/polybius_square.rs",
        );
        ui.add_space(8.0);

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
                        filter_string(&mut self.key_string, &self.alphabet_string);
                        self.cipher
                            .assign_key(&self.key_string, &self.alphabet_string)
                    }
                }
            });
        });

        ui.add_space(16.0);
        ui.subheading("Alphabet");
        if ui.control_string(&mut self.alphabet_string).changed() {
            self.cipher
                .assign_key(&self.key_string, &self.alphabet_string);
        }
        ui.add_space(16.0);
        ui.checkbox(&mut self.cipher.spaced, "Use Spaces")
            .on_hover_text("Insert spaces between the pairs of symbols");

        ui.add_space(16.0);
        ui.subheading("Keyword");
        if ui.control_string(&mut self.key_string).changed() {
            filter_string(&mut self.key_string, &self.alphabet_string);
            self.cipher
                .assign_key(&self.key_string, &self.alphabet_string)
        }

        ui.add_space(16.0);
        ui.subheading("Labels");
        if ui.control_string(&mut self.labels_string).changed() {
            self.cipher.assign_labels(&self.labels_string)
        }

        ui.add_space(16.0);
        ui.horizontal(|ui| {
            ui.subheading("Grid");
            ui.copy_to_clipboard(self.cipher.show_grid());
        });
        ui.mono(self.cipher.show_grid());

        ui.add_space(16.0);
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
