use ciphers::{transposition::RailFence, Cipher};
use egui::{Slider, Ui};

use super::CipherFrame;

#[derive(Default)]
pub struct RailFenceFrame {
    cipher: RailFence,
}

impl CipherFrame for RailFenceFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        // randomize_reset(ui, self);
        ui.add_space(16.0);

        ui.label("Number of Rails");
        ui.add(Slider::new(&mut self.cipher.rails, 2..=12));
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
