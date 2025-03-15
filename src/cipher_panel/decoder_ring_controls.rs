use super::CipherFrame;
use crate::ui_elements::UiElements;
use ciphers::{substitution::DecoderRing, Cipher};
use egui::{Slider, Ui};
use rand::{thread_rng, Rng};

pub struct DecoderRingFrame {
    cipher: DecoderRing,
    alphabet_string: String,
}

impl Default for DecoderRingFrame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
            alphabet_string: String::from("_ABCDEFGHIJKLMNOPWRSTUVWXYZ"),
        }
    }
}

impl CipherFrame for DecoderRingFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/ciphers/src/substitution/decoder_ring.rs",
        );
        ui.add_space(8.0);

        ui.randomize_reset_cipher(self);
        ui.add_space(16.0);

        ui.subheading("Alphabet");
        if ui.control_string(&mut self.alphabet_string).changed() {
            self.cipher.assign_alphabet(&self.alphabet_string)
        }
        ui.add_space(16.0);

        ui.subheading("Key");
        let alpha_range = 0..=(self.cipher.length() - 1);
        ui.add(Slider::new(&mut self.cipher.index, alpha_range));
        ui.add_space(16.0);

        ui.horizontal(|ui| {
            if ui.button("Little Orphan Annie").clicked() {
                self.alphabet_string = String::from("_ASLWIMVHFKXDPOEJBTNQZGUYRC");
                self.cipher.assign_alphabet("_ASLWIMVHFKXDPOEJBTNQZGUYRC")
            }
            if ui.button("Captain Midnight").clicked() {
                self.alphabet_string = String::from("_AEXDTZKNYCJWSGUMBOQHRIVFPL");
                self.cipher.assign_alphabet("_AEXDTZKNYCJWSGUMBOQHRIVFPL")
            }
        });
        ui.add_space(16.0);
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        self.cipher.index = thread_rng().gen_range(0..self.alphabet_string.chars().count());
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
