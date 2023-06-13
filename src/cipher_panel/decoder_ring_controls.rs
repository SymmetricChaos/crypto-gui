use super::CipherFrame;
use crate::ui_elements::{control_string, randomize_reset};
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
        randomize_reset(ui, self);
        ui.add_space(16.0);

        ui.label("Alphabet");
        if control_string(ui, &mut self.alphabet_string).changed() {
            self.cipher.assign_alphabet(&self.alphabet_string)
        }
        ui.add_space(16.0);

        ui.label("Key");
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
