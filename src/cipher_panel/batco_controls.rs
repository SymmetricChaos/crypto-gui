use super::CipherFrame;
use crate::ui_elements::{mono, randomize_reset};
use ciphers::{tactical::Batco, Cipher};
use egui::{Slider, Ui};
use rand::{rngs::StdRng, SeedableRng};
use utils::{preset_alphabet::Alphabet, text_functions::shuffled_str};

#[derive(Default)]
pub struct BatcoFrame {
    cipher: Batco,
}

impl CipherFrame for BatcoFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        randomize_reset(ui, self);
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

    fn randomize(&mut self) {
        let mut rng = StdRng::from_entropy();
        let alpha = Alphabet::BasicLatin.slice();
        for row in self.cipher.cipher_rows.iter_mut() {
            *row = shuffled_str(alpha, &mut rng)
        }
        for col in self.cipher.key_cols.iter_mut() {
            *col = shuffled_str(alpha, &mut rng)
        }
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
