use ciphers::{polybius::Bifid, Cipher};
use egui::{Color32, Slider, Ui};
use utils::preset_alphabet::PresetAlphabet;

use crate::egui_aux::mono;

use super::{CipherFrame, _generic_components::control_string};

pub struct BifidFrame {
    cipher: Bifid,
    alphabet_string: String,
    key_string: String,
}

impl Default for BifidFrame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
            alphabet_string: PresetAlphabet::BasicLatinNoQ.into(),
            key_string: Default::default(),
        }
    }
}

impl CipherFrame for BifidFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        // randomize_reset(ui, self);
        ui.add_space(16.0);

        let block_size_range = 3..=30;
        ui.label("Block Size");
        ui.add(Slider::new(&mut self.cipher.block_size, block_size_range));

        ui.label("Select Alphabet");
        ui.horizontal(|ui| {
            if ui.button("No Q").clicked() {
                self.cipher
                    .polybius
                    .pick_alphabet(PresetAlphabet::BasicLatinNoQ);
                self.alphabet_string = PresetAlphabet::BasicLatinNoQ.string();
                self.cipher.polybius.assign_key(&self.key_string)
            };
            if ui.button("No J").clicked() {
                self.cipher
                    .polybius
                    .pick_alphabet(PresetAlphabet::BasicLatinNoJ);
                self.alphabet_string = PresetAlphabet::BasicLatinNoJ.string();
                self.cipher.polybius.assign_key(&self.key_string)
            };
            if ui.button("Alphanumeric").clicked() {
                self.cipher
                    .polybius
                    .pick_alphabet(PresetAlphabet::BasicLatinWithDigits);
                self.alphabet_string = PresetAlphabet::BasicLatinWithDigits.string();
                self.cipher.polybius.assign_key(&self.key_string)
            };
            if ui.button("Base64").clicked() {
                self.cipher.polybius.pick_alphabet(PresetAlphabet::Base64);
                self.alphabet_string = PresetAlphabet::Base64.string();
                self.cipher.polybius.assign_key(&self.key_string)
            };
        });

        // False alphabet display
        ui.label(mono(&self.alphabet_string).background_color(Color32::BLACK));
        ui.add_space(16.0);

        ui.label("Key Word");
        if control_string(ui, &mut self.key_string).changed() {
            self.cipher.polybius.assign_key(&self.key_string)
        }
        ui.add_space(16.0);
        ui.label(mono(format!("Grid\n{}", self.cipher.polybius)));
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {}

    fn reset(&mut self) {
        *self = Self::default()
    }
}
