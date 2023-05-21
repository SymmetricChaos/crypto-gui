use super::{CipherFrame, _generic_components::control_string};
use ciphers::{polybius::PolybiusSquare, Cipher};
use eframe::egui::{Color32, RichText, Ui};
use utils::preset_alphabet::PresetAlphabet;

#[derive(Default)]
pub struct PolybiusSquareFrame {
    cipher: PolybiusSquare,
    alphabet_string: String,
    key_string: String,
}

impl CipherFrame for PolybiusSquareFrame {
    fn ui(&mut self, ui: &mut Ui, errors: &mut String) {
        randomize_reset(ui, self);
        ui.add_space(16.0);

        ui.label("Common Latin Alphabets");
        ui.horizontal(|ui| {
            if ui.button("No Q").clicked() {
                self.cipher.assign_alphabet(PresetAlphabet::BasicLatinNoQ)
            };
            if ui.button("No J").clicked() {
                self.cipher.assign_alphabet(PresetAlphabet::BasicLatinNoJ)
            };
            if ui.button("Alphanumeric").clicked() {
                self.cipher
                    .assign_alphabet(PresetAlphabet::BasicLatinWithDigits)
            };
            if ui.button("Base64").clicked() {
                self.cipher.assign_alphabet(PresetAlphabet::Base64)
            };
        });

        ui.label("Alphabet");
        if control_string(ui, &mut self.alphabet_string).changed() {
            match self.set_alphabet() {
                Ok(_) => (),
                Err(e) => *errors = e.to_string(),
            }
        }

        ui.add_space(10.0);
        ui.label(
            RichText::new(&self.alphabet_string)
                .monospace()
                .background_color(Color32::BLACK),
        );
        ui.add_space(16.0);

        ui.label("Key Word");
        if control_string(ui, &mut self.key_word).changed() {
            self.set_key()
        }

        ui.add_space(16.0);
        ui.label("Labels");
        if control_string(ui, &mut self.labels_string).changed() {
            self.set_labels();
        }

        ui.add_space(16.0);
        ui.label("Grid");
        mono(ui, &self.show_grid(), None);
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {}

    fn reset(&mut self) {
        *self = Self::default()
    }
}
