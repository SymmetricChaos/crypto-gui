use ciphers::digital::chacha::ChaCha;
use egui::{DragValue, Slider};
use rand::{thread_rng, Rng};

use crate::ui_elements::UiElements;

use super::CipherFrame;

pub struct ChaChaFrame {
    cipher: ChaCha,
}

impl Default for ChaChaFrame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
        }
    }
}

impl CipherFrame for ChaChaFrame {
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
        ui.add_space(8.0);
        ui.subheading("Number of Rounds");
        ui.horizontal(|ui| {
            if ui.small_button("ChaCha8").clicked() {
                self.cipher.rounds = 8;
            }
            if ui.small_button("ChaCha12").clicked() {
                self.cipher.rounds = 8;
            }
            if ui.small_button("ChaCha20").clicked() {
                self.cipher.rounds = 8;
            }
        });
        ui.add(Slider::new(&mut self.cipher.rounds, 2..=20));
        ui.add_space(8.0);
        // ui.subheading("Authenticated");
        // ui.label("The Poly1305 hash function can be used for authenticated encryption.");
        // ui.checkbox(&mut self.authenticated, "");
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
