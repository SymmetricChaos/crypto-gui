use ciphers::{transposition::RailFence, Cipher};
use egui::{Slider, Ui};
use rand::{thread_rng, Rng};

use super::{CipherFrame, _generic_components::randomize_reset};

#[derive(Default)]
pub struct RailFenceFrame {
    cipher: RailFence,
}

impl CipherFrame for RailFenceFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        randomize_reset(ui, self);
        ui.add_space(16.0);

        ui.label("Number of Rails");
        ui.add(Slider::new(&mut self.cipher.rails, 2..=12));
        ui.add_space(16.0);
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        self.cipher.rails = thread_rng().gen_range(2..12);
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
