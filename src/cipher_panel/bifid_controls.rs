use super::CipherFrame;
use crate::ui_elements::UiElements;
use ciphers::{polybius::Bifid, Cipher};
use egui::{Slider, Ui};
use rand::{thread_rng, Rng};
use utils::{
    preset_alphabet::Alphabet,
    text_functions::{filter_string, shuffled_str},
};

pub struct BifidFrame {
    cipher: Bifid,
    alphabet_string: String,
    key_string: String,
}

impl Default for BifidFrame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
            alphabet_string: Alphabet::BasicLatinNoQ.into(),
            key_string: Default::default(),
        }
    }
}

impl CipherFrame for BifidFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/ciphers/src/polybius/bifid.rs",
        );
        ui.add_space(8.0);

        ui.randomize_reset_cipher(self);
        ui.add_space(16.0);

        let block_size_range = 3..=30;
        ui.subheading("Block Size");
        ui.add(Slider::new(&mut self.cipher.block_size, block_size_range));

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
                            .polybius
                            .assign_key(&self.key_string, alphabet.into());
                    }
                }
            });
        });
        ui.add_space(10.0);

        ui.subheading("Alphabet");
        if ui.control_string(&mut self.alphabet_string).changed() {
            self.cipher
                .polybius
                .assign_key(&self.key_string, &self.alphabet_string);
        }
        ui.add_space(16.0);

        ui.subheading("Keyword");
        if ui.control_string(&mut self.key_string).changed() {
            filter_string(&mut self.key_string, &self.alphabet_string);
            self.cipher
                .polybius
                .assign_key(&self.key_string, &self.alphabet_string)
        }
        ui.add_space(16.0);

        ui.horizontal(|ui| {
            ui.subheading("Grid");
            ui.copy_to_clipboard(self.cipher.polybius.show_grid());
        });
        ui.mono(self.cipher.polybius.show_grid());
        ui.add_space(16.0);
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        self.cipher.block_size = rng.gen_range(3..=30);
        self.key_string = shuffled_str(&self.alphabet_string, &mut thread_rng());
        self.cipher
            .polybius
            .assign_key(&self.key_string, &self.alphabet_string);
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
