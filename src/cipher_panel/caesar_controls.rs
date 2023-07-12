use super::CipherFrame;
use crate::ui_elements::{control_string, randomize_reset, subheading};
use ciphers::{substitution::Caesar, Cipher};
use egui::{Slider, Ui};
use rand::{thread_rng, Rng};
use utils::preset_alphabet::Alphabet;

pub struct CaesarFrame {
    cipher: Caesar,
    alphabet_string: String,
}

impl Default for CaesarFrame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
            alphabet_string: Alphabet::BasicLatin.into(),
        }
    }
}

impl CipherFrame for CaesarFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        randomize_reset(ui, self);
        ui.add_space(16.0);

        ui.group(|ui| {
            ui.label(subheading("Common Alphabets"));
            ui.horizontal(|ui| {
                for (name, alphabet) in [
                    ("Basic Latin", Alphabet::BasicLatin),
                    ("Classical Latin", Alphabet::ClassicalLatin),
                    ("Alphanumeric", Alphabet::BasicLatinWithDigits),
                    ("ASCII", Alphabet::Ascii94),
                    ("Base64", Alphabet::Base64),
                ] {
                    if ui.button(name).clicked() {
                        self.alphabet_string = alphabet.into();
                        self.cipher.assign_alphabet(&self.alphabet_string)
                    }
                }
            });
        });
        ui.add_space(8.0);

        ui.label(subheading("Alphabet"));
        if control_string(ui, &mut self.alphabet_string).changed() {
            self.cipher.assign_alphabet(&self.alphabet_string)
        }
        ui.add_space(8.0);

        ui.label(subheading("Shift Distance"));
        ui.add(Slider::new(
            &mut self.cipher.shift,
            0..=(self.cipher.alphabet.len() as i32 - 1),
        ));
        ui.add_space(8.0);

        ui.horizontal(|ui| {
            if ui.button("ROT13").clicked() {
                self.alphabet_string = Alphabet::BasicLatin.into();
                self.cipher.assign_alphabet(Alphabet::BasicLatin.into());
                self.cipher.shift = 13;
            }
            if ui.button("ROT47").clicked() {
                self.alphabet_string = Alphabet::Ascii94.into();
                self.cipher.assign_alphabet(Alphabet::Ascii94.into());
                self.cipher.shift = 47;
            }
        });
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        self.cipher.shift = thread_rng().gen_range(0..self.cipher.alphabet.len()) as i32;
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
