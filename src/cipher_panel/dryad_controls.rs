use ciphers::{tactical::Dryad, Cipher};
use egui::{Slider, Ui};

use crate::egui_aux::mono;

use super::CipherFrame;

#[derive(Default)]
pub struct DryadFrame {
    cipher: Dryad,
}

impl CipherFrame for DryadFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        // randomize_reset(ui, self);
        ui.add_space(16.0);

        ui.label("Message Key");
        ui.horizontal(|ui| {
            ui.label(mono(self.cipher.message_key_to_char()));
            ui.add(
                Slider::new(&mut self.cipher.message_key, 0..=24)
                    .clamp_to_range(true)
                    .show_value(false),
            );
        });

        ui.add_space(16.0);

        ui.label(mono(self.cipher.show_code_page()));
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {}

    fn reset(&mut self) {
        *self = Self::default()
    }
}
