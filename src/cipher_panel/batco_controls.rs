use super::CipherFrame;
use crate::ui_elements::UiElements;
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
        ui.randomize_reset(self);
        ui.add_space(16.0);

        ui.subheading("Message Key");
        ui.horizontal(|ui| {
            ui.mono(&self.cipher.message_number_to_char());
            ui.add(
                Slider::new(&mut self.cipher.message_number, 0..=5)
                    .clamp_to_range(true)
                    .show_value(false),
            );
        });
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            ui.mono(&self.cipher.message_letter_to_char());

            ui.add(
                Slider::new(&mut self.cipher.message_letter, 0..=25)
                    .clamp_to_range(true)
                    .show_value(false),
            );
        });
        ui.add_space(16.0);

        ui.horizontal(|ui| {
            ui.subheading("Code Page");
            ui.copy_to_clipboard(self.cipher.show_code_page());
        });
        ui.mono(&self.cipher.show_code_page());
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
