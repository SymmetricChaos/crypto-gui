use ciphers::{playfair::Playfair, Cipher};
use egui::{TextEdit, TextStyle, Ui};
use rand::{rngs::StdRng, SeedableRng};
use utils::{functions::shuffled_str, preset_alphabet::Alphabet};

use crate::egui_aux::mono;

use super::{
    CipherFrame,
    _generic_components::{control_string, randomize_reset},
};

pub struct PlayfairFrame {
    cipher: Playfair,
    key_string: String,
    alphabet_string: String,
}

impl Default for PlayfairFrame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
            key_string: Default::default(),
            alphabet_string: Alphabet::BasicLatinNoQ.into(),
        }
    }
}

impl CipherFrame for PlayfairFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        randomize_reset(ui, self);
        ui.add_space(16.0);

        ui.label("Common Alphabets");
        ui.horizontal(|ui| {
            if ui.button("No Q").clicked() {
                self.alphabet_string = Alphabet::BasicLatinNoQ.string();
                self.cipher
                    .assign_key(&self.key_string, &self.alphabet_string)
            };
            if ui.button("No J").clicked() {
                self.alphabet_string = Alphabet::BasicLatinNoJ.string();
                self.cipher
                    .assign_key(&self.key_string, &self.alphabet_string)
            };
            if ui.button("Alphanumeric").clicked() {
                self.alphabet_string = Alphabet::BasicLatinWithDigits.string();
                self.cipher
                    .assign_key(&self.key_string, &self.alphabet_string)
            };
            if ui.button("Base64").clicked() {
                self.alphabet_string = Alphabet::Base64.string();
                self.cipher
                    .assign_key(&self.key_string, &self.alphabet_string)
            };
        });
        ui.add_space(8.0);

        ui.label("Alphabet");
        if control_string(ui, &mut self.alphabet_string).changed() {
            self.cipher
                .assign_key(&self.key_string, &self.alphabet_string);
        }
        ui.add_space(16.0);

        ui.label("Key Word");
        if control_string(ui, &mut self.key_string).changed() {
            self.cipher
                .assign_key(&self.key_string, &self.alphabet_string)
        }
        ui.add_space(16.0);

        ui.label("Spacer Character\nInserted as padding where needed");
        ui.add(
            TextEdit::singleline(&mut self.cipher.control_spacer().to_string())
                .font(TextStyle::Monospace)
                .desired_width(15.0),
        );

        ui.add_space(16.0);

        ui.horizontal(|ui| {
            ui.label("Grid");
            if ui.button("📋").on_hover_text("Copy to Clipboard").clicked() {
                ui.output_mut(|o| o.copied_text = self.cipher.to_string())
            }
        });
        ui.label(mono(self.cipher.to_string()));
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        self.key_string = shuffled_str(&self.cipher.square, &mut StdRng::from_entropy());
        self.cipher
            .assign_key(&self.key_string, &self.alphabet_string);
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
