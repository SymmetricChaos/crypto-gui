use super::CipherFrame;
use crate::egui_aux::mono;
use ciphers::{tactical::Batco, Cipher};
use egui::{Slider, Ui};

#[derive(Default)]
pub struct BatcoFrame {
    cipher: Batco,
}

impl CipherFrame for BatcoFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        // randomize_reset(ui, self);
        ui.add_space(16.0);

        ui.label("Message Key");

        ui.horizontal(|ui| {
            ui.label(mono(&self.cipher.message_number_to_char()));
            ui.add(
                Slider::new(&mut self.cipher.message_number, 0..=5)
                    .clamp_to_range(true)
                    .show_value(false),
            );
        });

        ui.horizontal(|ui| {
            ui.label(mono(&self.cipher.message_letter_to_char()));

            ui.add(
                Slider::new(&mut self.cipher.message_letter, 0..=25)
                    .clamp_to_range(true)
                    .show_value(false),
            );
        });

        ui.add_space(16.0);

        ui.label(mono(&self.cipher.show_code_page()));
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {}

    fn reset(&mut self) {
        *self = Self::default()
    }
}
