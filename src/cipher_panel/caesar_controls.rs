use super::CipherFrame;
use crate::ui_elements::UiElements;
use ciphers::substitution::Caesar;
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

impl CaesarFrame {
    fn shifted_alphabet(&self) -> String {
        let mut s: String = self
            .alphabet_string
            .chars()
            .skip(self.cipher.shift as usize)
            .collect();
        s.push_str(
            &self
                .alphabet_string
                .chars()
                .take(self.cipher.shift as usize)
                .collect::<String>(),
        );
        s
    }
}

impl CipherFrame for CaesarFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/ciphers/src/substitution/caesar.rs",
        );
        ui.add_space(8.0);

        ui.randomize_reset_cipher(self);
        ui.add_space(16.0);

        ui.group(|ui| {
            ui.subheading("Common Alphabets");
            ui.horizontal(|ui| {
                for alphabet in [
                    Alphabet::Alphanumeric,
                    Alphabet::BasicLatin,
                    Alphabet::Ascii94,
                    Alphabet::Base64,
                ] {
                    if ui.button(alphabet.name()).clicked() {
                        self.alphabet_string = alphabet.into();
                        self.cipher.assign_alphabet(&self.alphabet_string)
                    }
                }
            });
        });
        ui.add_space(16.0);

        ui.subheading("Alphabet");
        if ui.control_string(&mut self.alphabet_string).changed() {
            self.cipher.assign_alphabet(&self.alphabet_string)
        }
        ui.mono(self.shifted_alphabet());
        ui.add_space(16.0);

        ui.subheading("Shift Distance");
        ui.add(Slider::new(
            &mut self.cipher.shift,
            0..=(self.cipher.alphabet.len() as i32 - 1),
        ));
        ui.add_space(16.0);

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
        ui.add_space(16.0);
    }

    fn randomize(&mut self) {
        self.cipher.shift = thread_rng().gen_range(0..self.cipher.alphabet.len()) as i32;
    }

    crate::simple_cipher! {}
}
