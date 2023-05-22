use crate::egui_aux::mono;

use super::{CipherFrame, _generic_components::control_string};
use ciphers::{polybius::PolybiusSquare, Cipher};
use eframe::egui::{Color32, RichText, Ui};
use utils::preset_alphabet::PresetAlphabet;

#[derive(Default)]
pub struct PolybiusSquareFrame {
    cipher: PolybiusSquare,
    alphabet_string: String,
    key_string: String,
    labels_string: String,
}

impl CipherFrame for PolybiusSquareFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        // randomize_reset(ui, self);
        ui.add_space(16.0);

        ui.label("Common Latin Alphabets");
        ui.horizontal(|ui| {
            if ui.button("No Q").clicked() {
                self.cipher.pick_alphabet(PresetAlphabet::BasicLatinNoQ)
            };
            if ui.button("No J").clicked() {
                self.cipher.pick_alphabet(PresetAlphabet::BasicLatinNoJ)
            };
            if ui.button("Alphanumeric").clicked() {
                self.cipher
                    .pick_alphabet(PresetAlphabet::BasicLatinWithDigits)
            };
            if ui.button("Base64").clicked() {
                self.cipher.pick_alphabet(PresetAlphabet::Base64)
            };
        });

        // ui.label("Alphabet");
        // if control_string(ui, &mut self.alphabet_string).changed() {
        //     match self.cipher.set_alphabet() {
        //         Ok(_) => (),
        //         Err(e) => *errors = e.to_string(),
        //     }
        // }

        ui.add_space(10.0);
        ui.label(
            RichText::new(&self.alphabet_string)
                .monospace()
                .background_color(Color32::BLACK),
        );
        ui.add_space(16.0);

        ui.label("Key Word");
        if control_string(ui, &mut self.key_string).changed() {
            self.cipher.assign_key(&self.key_string)
        }

        ui.add_space(16.0);
        ui.label("Labels");
        if control_string(ui, &mut self.labels_string).changed() {
            self.cipher.assign_labels(&self.labels_string)
        }

        ui.add_space(16.0);
        ui.label("Grid");
        ui.label(mono(self.cipher.show_grid()));
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        // self.key_word = shuffled_str(&self.alphabet_string, &mut get_global_rng());
        // self.set_key();
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
