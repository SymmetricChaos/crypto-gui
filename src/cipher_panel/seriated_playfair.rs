use super::CipherFrame;
use crate::ui_elements::{control_string, mono, randomize_reset};
use ciphers::{playfair::seriated_playfair::SeriatedPlayfair, Cipher};
use egui::{Slider, Ui};
use rand::{rngs::StdRng, SeedableRng};
use utils::{functions::shuffled_str, preset_alphabet::Alphabet};

pub struct SeriatedPlayfairFrame {
    cipher: SeriatedPlayfair,
    key_string: String,
    alphabet_string: String,
}

impl Default for SeriatedPlayfairFrame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
            key_string: Default::default(),
            alphabet_string: Alphabet::BasicLatinNoQ.into(),
        }
    }
}

impl CipherFrame for SeriatedPlayfairFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        randomize_reset(ui, self);
        ui.add_space(16.0);

        ui.label("Common Alphabets");
        ui.horizontal(|ui| {
            if ui.button("No Q").clicked() {
                self.alphabet_string = Alphabet::BasicLatinNoQ.string();
                self.cipher
                    .playfair
                    .assign_key(&self.key_string, &self.alphabet_string)
            };
            if ui.button("No J").clicked() {
                self.alphabet_string = Alphabet::BasicLatinNoJ.string();
                self.cipher
                    .playfair
                    .assign_key(&self.key_string, &self.alphabet_string)
            };
            if ui.button("Alphanumeric").clicked() {
                self.alphabet_string = Alphabet::BasicLatinWithDigits.string();
                self.cipher
                    .playfair
                    .assign_key(&self.key_string, &self.alphabet_string)
            };
            if ui.button("Base64").clicked() {
                self.alphabet_string = Alphabet::Base64.string();
                self.cipher
                    .playfair
                    .assign_key(&self.key_string, &self.alphabet_string)
            };
        });
        ui.add_space(8.0);

        ui.label("Alphabet");
        if control_string(ui, &mut self.alphabet_string).changed() {
            self.cipher
                .playfair
                .assign_key(&self.key_string, &self.alphabet_string);
        }
        ui.add_space(16.0);

        ui.label("Key Word");
        if control_string(ui, &mut self.key_string).changed() {
            self.cipher
                .playfair
                .assign_key(&self.key_string, &self.alphabet_string)
        }
        ui.add_space(16.0);

        ui.label("Period");
        ui.add(Slider::new(&mut self.cipher.period, 4..=12));

        // ui.menu_button("Spacer Character", |ui| {
        //     for c in self.alphabet_string.chars() {
        //         if ui.button(c.to_string()).clicked() {
        //             self.cipher.spacer = c
        //         }
        //     }
        // });
        // ui.label(self.cipher.spacer.to_string());

        ui.horizontal(|ui| {
            ui.label("Grid");
            if ui.button("📋").on_hover_text("Copy to Clipboard").clicked() {
                ui.output_mut(|o| o.copied_text = self.cipher.playfair.to_string())
            }
        });
        ui.label(mono(self.cipher.playfair.to_string()));
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        self.key_string = shuffled_str(&self.cipher.playfair.square, &mut StdRng::from_entropy());
        self.cipher
            .playfair
            .assign_key(&self.key_string, &self.alphabet_string);
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
