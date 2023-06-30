use super::CipherFrame;
use crate::ui_elements::{control_string, mono, randomize_reset};
use ciphers::{playfair::Playfair, Cipher};
use egui::Ui;
use rand::{rngs::StdRng, SeedableRng};
use utils::{
    functions::{filter_string, shuffled_str},
    preset_alphabet::Alphabet,
};

pub struct PlayfairFrame {
    cipher: Playfair,
    key_string: String,
    alphabet_string: String,
    spacer_string: String,
}

impl Default for PlayfairFrame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
            key_string: Default::default(),
            alphabet_string: Alphabet::BasicLatinNoQ.into(),
            spacer_string: "X".into(),
        }
    }
}

impl CipherFrame for PlayfairFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        randomize_reset(ui, self);
        ui.add_space(16.0);

        ui.label("Common Alphabets");
        ui.horizontal(|ui| {
            for (name, alphabet) in [
                ("No C", Alphabet::BasicLatinNoC),
                ("No J", Alphabet::BasicLatinNoJ),
                ("No Q", Alphabet::BasicLatinNoQ),
                ("Alphanumeric", Alphabet::BasicLatinWithDigits),
                ("Base64", Alphabet::Base64),
            ] {
                if ui.button(name).clicked() {
                    self.alphabet_string = alphabet.into();
                    filter_string(&mut self.key_string, &self.alphabet_string);
                    self.cipher
                        .assign_key(&self.key_string, &self.alphabet_string)
                }
            }
        });
        ui.add_space(10.0);

        ui.label("Alphabet");
        if control_string(ui, &mut self.alphabet_string).changed() {
            self.cipher
                .assign_key(&self.key_string, &self.alphabet_string);
        }
        ui.add_space(16.0);

        ui.label("Spacer Character\nInserted at end as padding if needed");
        if control_string(ui, &mut self.spacer_string).changed() {
            if self.spacer_string.is_empty() {
                ui.label("defaulting to X");
            } else {
                self.spacer_string = self.spacer_string.chars().next().unwrap().to_string()
            }
            self.cipher.spacer = self.spacer_string.chars().next().unwrap_or('X');
        }
        ui.add_space(16.0);

        ui.label("Keyword");
        if control_string(ui, &mut self.key_string).changed() {
            self.cipher
                .assign_key(&self.key_string, &self.alphabet_string)
        }
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
