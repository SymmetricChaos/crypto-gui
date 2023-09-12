use crate::ui_elements::UiElements;
use ciphers::{vic::Vic, Cipher};
use egui::{DragValue, Ui};
use utils::preset_alphabet::Alphabet;

use super::CipherFrame;

pub struct VicFrame {
    cipher: Vic,
}

impl Default for VicFrame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
        }
    }
}

impl CipherFrame for VicFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        ui.randomize_reset(self);

        ui.subheading("Key Group");
        if ui.control_string(&mut self.cipher.key_group).changed() {
            self.cipher.key_group = self.cipher.key_group.chars().take(5).collect();
        }
        if self.cipher.key_group.chars().count() != 5 {
            ui.error_text("key group must have exactly five digits");
        } else {
            ui.error_text("");
        }
        ui.add_space(8.0);

        ui.subheading("Date");
        ui.label("The date that the message is sent. Leading zeroes should not be used.");
        ui.control_string(&mut self.cipher.date);
        if self.cipher.date.chars().count() < 6 {
            ui.error_text("date must have at least six digits");
        } else {
            ui.error_text("");
        }
        ui.add_space(8.0);

        ui.subheading("Phrase");
        if ui.control_string(&mut self.cipher.phrase).changed() {
            self.cipher.phrase = self
                .cipher
                .phrase
                .chars()
                .filter(|c| Alphabet::BasicLatin.contains(c))
                .collect();
        }
        if self.cipher.phrase.chars().count() < 20 {
            ui.error_text("phrase must have at least twenty letters");
        } else {
            ui.error_text("");
        }
        ui.add_space(8.0);

        ui.subheading("Personal Number");
        ui.add(DragValue::new(&mut self.cipher.pin).clamp_range(1..=20));

        ui.add_space(8.0);

        ui.subheading("Code Group Position");
        ui.label("The VIC cipher is written into roups of five digits. The last digit of the date is the group where the codegroup is inserted.");
        match self.cipher.date.chars().nth(5) {
            Some(c) => ui.mono(c),
            None => ui.error_text("date does not have a sixth digit"),
        };
        ui.add_space(12.0);

        match self.cipher.key_derivation_string() {
            Ok(text) => ui.mono(text),
            Err(e) => ui.error_text(e),
        };
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {}

    fn reset(&mut self) {
        *self = Self::default()
    }
}
