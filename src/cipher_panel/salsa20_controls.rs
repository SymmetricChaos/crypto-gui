use ciphers::digital::salsa20::Salsa20;
use egui::DragValue;
use rand::{thread_rng, Rng};

use crate::ui_elements::UiElements;

use super::CipherFrame;

pub struct Salsa20Frame {
    cipher: Salsa20,
}

impl Default for Salsa20Frame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
        }
    }
}

impl CipherFrame for Salsa20Frame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.randomize_reset(self);
        ui.add_space(16.0);

        ui.subheading("Key");
        for i in 0..4 {
            ui.add(DragValue::new(&mut self.cipher.key[i]).hexadecimal(8, false, true));
        }
        ui.add_space(8.0);
        ui.subheading("Nonce");
        for i in 0..2 {
            ui.add(DragValue::new(&mut self.cipher.nonce[i]).hexadecimal(8, false, true));
        }
    }

    fn cipher(&self) -> &dyn ciphers::Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        rng.fill(&mut self.cipher.key);
        rng.fill(&mut self.cipher.nonce);
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
