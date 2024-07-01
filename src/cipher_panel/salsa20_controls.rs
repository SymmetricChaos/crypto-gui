use ciphers::digital::salsa20::Salsa20;
use egui::Slider;
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
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/ciphers/src/digital/salsa20.rs",
        );
        ui.add_space(8.0);

        ui.randomize_reset(self);
        ui.add_space(16.0);

        ui.subheading("Key");
        for i in 0..4 {
            ui.u32_drag_value_hex(&mut self.cipher.key[i]);
        }
        ui.add_space(8.0);
        ui.subheading("Nonce");
        for i in 0..2 {
            ui.u32_drag_value_hex(&mut self.cipher.nonce[i]);
        }
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            if ui.small_button("Salsa20/8").clicked() {
                self.cipher.rounds = 8;
            }
            if ui.small_button("Salsa20/12").clicked() {
                self.cipher.rounds = 8;
            }
            if ui.small_button("Salsa20/20").clicked() {
                self.cipher.rounds = 8;
            }
        });
        ui.add(Slider::new(&mut self.cipher.rounds, 2..=20));
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
