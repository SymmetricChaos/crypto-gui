use ciphers::{
    polyalphabetic::{Beaufort, PolyMode},
    Cipher,
};
use egui::{Slider, TextEdit, TextStyle, Ui};
use utils::preset_alphabet::PresetAlphabet;

use super::{CipherFrame, _generic_components::control_string};

pub struct BeaufortFrame {
    cipher: Beaufort,
    alphabet_string: String,
}

impl Default for BeaufortFrame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
            alphabet_string: PresetAlphabet::BasicLatin.into(),
        }
    }
}

impl CipherFrame for BeaufortFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        // randomize_reset(ui, self);
        ui.add_space(16.0);

        ui.label("Alphabet");
        if control_string(ui, &mut self.alphabet_string).changed() {
            self.cipher.assign_alphabet(&self.alphabet_string)
        }
        ui.add_space(10.0);

        ui.label("Mode");
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.cipher.mode, PolyMode::CylicKey, "Cyclic");
            ui.selectable_value(&mut self.cipher.mode, PolyMode::Autokey, "Autokey");
            ui.selectable_value(&mut self.cipher.mode, PolyMode::ProgKey, "Progressive");
        });

        if self.cipher.mode == PolyMode::ProgKey {
            ui.add_space(16.0);
            ui.label("Step size");
            let alpha_range = 0..=(self.cipher.alphabet_len() - 1);
            ui.add(Slider::new(&mut self.cipher.prog_shift, alpha_range));
            ui.add_space(16.0);
        }

        match self.cipher.multikey {
            true => {
                ui.horizontal(|ui| {
                    ui.label("Key Words");
                    ui.checkbox(&mut self.cipher.multikey, "Multikey");
                });
                ui.add(
                    TextEdit::singleline(&mut self.cipher.key_words[0]).font(TextStyle::Monospace),
                );
                ui.add(
                    TextEdit::singleline(&mut self.cipher.key_words[1]).font(TextStyle::Monospace),
                );
                ui.add(
                    TextEdit::singleline(&mut self.cipher.key_words[2]).font(TextStyle::Monospace),
                );
                ui.add(
                    TextEdit::singleline(&mut self.cipher.key_words[3]).font(TextStyle::Monospace),
                );
                ui.add(
                    TextEdit::singleline(&mut self.cipher.key_words[4]).font(TextStyle::Monospace),
                );
            }
            false => {
                ui.horizontal(|ui| {
                    ui.label("Key Word ");
                    ui.checkbox(&mut self.cipher.multikey, "Multikey");
                });
                ui.add(
                    TextEdit::singleline(&mut self.cipher.key_words[0]).font(TextStyle::Monospace),
                );
            }
        }
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {}

    fn reset(&mut self) {
        *self = Self::default()
    }
}
