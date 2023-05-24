use ciphers::{playfair::TwoSquare, Cipher};
use egui::{Color32, RichText, Ui};
use utils::preset_alphabet::PresetAlphabet;

use crate::egui_aux::mono;

use super::{CipherFrame, _generic_components::control_string};

#[derive(Default)]
pub struct TwoSquareFrame {
    cipher: TwoSquare,
    keyword1: String,
    keyword2: String,
}

impl CipherFrame for TwoSquareFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        // randomize_reset(ui, self);
        ui.add_space(16.0);

        ui.label("Select Alphabet");
        ui.horizontal(|ui| {
            if ui.button("No Q").clicked() {
                self.cipher.pick_alphabet(PresetAlphabet::BasicLatinNoQ);
                self.cipher.assign_key1(&self.keyword1);
                self.cipher.assign_key2(&self.keyword2);
            };
            if ui.button("No J").clicked() {
                self.cipher.pick_alphabet(PresetAlphabet::BasicLatinNoJ);
                self.cipher.assign_key1(&self.keyword1);
                self.cipher.assign_key2(&self.keyword2);
            };
            if ui.button("Alphanumeric").clicked() {
                self.cipher
                    .pick_alphabet(PresetAlphabet::BasicLatinWithDigits);
                self.cipher.assign_key1(&self.keyword1);
                self.cipher.assign_key2(&self.keyword2);
            };
            if ui.button("Base64").clicked() {
                self.cipher.pick_alphabet(PresetAlphabet::Base64);
                self.cipher.assign_key1(&self.keyword1);
                self.cipher.assign_key2(&self.keyword2);
            };
        });
        ui.add_space(10.0);

        ui.label(
            RichText::new(&self.cipher.alphabet.to_string())
                .monospace()
                .background_color(Color32::BLACK),
        );
        ui.add_space(16.0);

        ui.label("Key Word #1");
        if control_string(ui, &mut self.keyword1).changed() {
            self.cipher.assign_key1(&self.keyword1)
        }

        ui.label("Key Word #2");
        if control_string(ui, &mut self.keyword2).changed() {
            self.cipher.assign_key2(&self.keyword2)
        }

        ui.label(mono(self.cipher.show_square1()));
        ui.add_space(8.0);
        ui.label(mono(self.cipher.show_square2()));

        // ui.label(RichText::new(format!("Grid\n{}", self)).monospace());
        // ui.add_space(16.0);

        //(ui, self.grid_side_len(), self.grid_side_len(), self.get_input_alphabet())
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {}

    fn reset(&mut self) {
        *self = Self::default()
    }
}
