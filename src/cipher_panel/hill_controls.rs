use super::CipherFrame;
use crate::ui_elements::UiElements;
use ciphers::hill::Hill;
use egui::Ui;
use rand::{thread_rng, Rng};
use utils::{
    preset_alphabet::Alphabet,
    text_functions::{filter_string, random_string_sample_replace},
};

pub struct HillFrame {
    cipher: Hill,
    alphabet_string: String,
}

impl Default for HillFrame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
            alphabet_string: Alphabet::BasicLatin.into(),
        }
    }
}

impl CipherFrame for HillFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/ciphers/src/hill.rs",
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
                        self.cipher.assign_alphabet(alphabet.slice());
                        filter_string(&mut self.cipher.key1, &self.alphabet_string);
                        filter_string(&mut self.cipher.key2, &self.alphabet_string);
                    }
                }
            });
        });

        ui.subheading("Alphabet");
        if ui.control_string(&mut self.alphabet_string).changed() {
            self.cipher.assign_alphabet(&self.alphabet_string);
            filter_string(&mut self.cipher.key1, &self.alphabet_string);
            filter_string(&mut self.cipher.key2, &self.alphabet_string);
        }
        ui.add_space(16.0);

        ui.subheading("Matrix");
        ui.label(format!("{:>2?}", self.cipher.mat[0]));
        ui.label(format!("{:>2?}", self.cipher.mat[1]));
        ui.label(format!("{:>2?}", self.cipher.mat[2]));
        ui.add_space(8.0);

        ui.subheading("Matrix Inverse");
        ui.label(format!("{:>2?}", self.cipher.mat_inv[0]));
        ui.label(format!("{:>2?}", self.cipher.mat_inv[1]));
        ui.label(format!("{:>2?}", self.cipher.mat_inv[2]));
        ui.add_space(8.0);

        ui.subheading("Key 1");
        if ui.control_string(&mut self.cipher.key1).changed() {
            filter_string(&mut self.cipher.key1, &self.alphabet_string);
        }
        ui.add_space(8.0);

        ui.subheading("Key 2");
        if ui.control_string(&mut self.cipher.key2).changed() {
            filter_string(&mut self.cipher.key2, &self.alphabet_string);
        }
        ui.add_space(16.0);
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        self.cipher.key1 =
            random_string_sample_replace(&self.alphabet_string, rng.gen_range(3..12), &mut rng);
        self.cipher.key2 =
            random_string_sample_replace(&self.alphabet_string, rng.gen_range(3..12), &mut rng);
    }

    crate::simple_cipher! {}
}
