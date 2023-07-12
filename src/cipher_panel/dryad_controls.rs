use ciphers::{tactical::Dryad, Cipher};
use egui::{Slider, Ui};
use rand::thread_rng;
use utils::{preset_alphabet::Alphabet, text_functions::shuffled_str};

use crate::ui_elements::{mono, randomize_reset, subheading};

use super::CipherFrame;

#[derive(Default)]
pub struct DryadFrame {
    cipher: Dryad,
}

impl CipherFrame for DryadFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        randomize_reset(ui, self);
        ui.add_space(16.0);

        ui.label(subheading("Message Key"));
        ui.horizontal(|ui| {
            ui.label(mono(self.cipher.message_key_to_char()));
            ui.add(
                Slider::new(&mut self.cipher.message_key, 0..=24)
                    .clamp_to_range(true)
                    .show_value(false),
            );
        });

        ui.add_space(16.0);

        ui.horizontal(|ui| {
            ui.label(subheading("Code Page"));
            if ui.button("📋").on_hover_text("Copy to Clipboard").clicked() {
                ui.output_mut(|o| o.copied_text = self.cipher.show_code_page())
            }
        });
        ui.label(mono(self.cipher.show_code_page()));
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        let alpha = Alphabet::BasicLatin.slice();
        for row in self.cipher.cipher_rows.iter_mut() {
            *row = shuffled_str(alpha, &mut rng)
        }
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
