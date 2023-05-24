use ciphers::{transposition::Scytale, Cipher};
use egui::{Slider, Ui};

use super::CipherFrame;

#[derive(Default)]
pub struct ScytaleFrame {
    cipher: Scytale,
}

impl CipherFrame for ScytaleFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        // randomize_reset(ui, self);
        ui.add_space(16.0);

        ui.label("Wraps");
        ui.add(Slider::new(&mut self.cipher.key, 2..=12));
        ui.add_space(16.0);
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {}

    fn reset(&mut self) {
        *self = Self::default()
    }
}
