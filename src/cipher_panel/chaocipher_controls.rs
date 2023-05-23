use ciphers::{polyalphabetic::Chaocipher, Cipher};
use egui::Ui;

use super::{CipherFrame, _generic_components::control_string};

pub struct ChaocipherFrame {
    cipher: Chaocipher,
    left_string: String,
    right_string: String,
}

impl Default for ChaocipherFrame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
            left_string: String::from("HXUCZVAMDSLKPEFJRIGTWOBNYQ"),
            right_string: String::from("PTLNBQDEOYSFAVZKGJRIHWXUMC"),
        }
    }
}

impl CipherFrame for ChaocipherFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        // randomize_reset(ui, self);
        ui.add_space(16.0);

        if control_string(ui, &mut self.left_string).changed() {
            self.cipher.assign_left(&self.left_string)
        }

        if control_string(ui, &mut self.right_string).changed() {
            self.cipher.assign_right(&self.right_string)
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
