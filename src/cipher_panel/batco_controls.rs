use super::CipherFrame;
use crate::ui_elements::UiElements;
use ciphers::tactical::Batco;
use egui::{Slider, SliderClamping::Always, Ui};
use rand::{rngs::StdRng, SeedableRng};
use utils::{preset_alphabet::Alphabet, text_functions::shuffled_str};

#[derive(Default)]
pub struct BatcoFrame {
    cipher: Batco,
}

impl CipherFrame for BatcoFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/ciphers/src/tactical/batco.rs",
        );
        ui.add_space(8.0);

        ui.randomize_reset_cipher(self);
        ui.add_space(16.0);

        ui.subheading("Message Key");
        ui.horizontal(|ui| {
            ui.mono(&self.cipher.message_number_to_char());
            ui.add(
                Slider::new(&mut self.cipher.message_number, 0..=5)
                    .clamping(Always)
                    .show_value(false),
            );
        });
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            ui.mono(&self.cipher.message_letter_to_char());

            ui.add(
                Slider::new(&mut self.cipher.message_letter, 0..=25)
                    .clamping(Always)
                    .show_value(false),
            );
        });
        ui.add_space(16.0);

        ui.horizontal(|ui| {
            ui.subheading("Code Page");
            ui.copy_to_clipboard(self.cipher.show_code_page());
        });
        ui.mono(&self.cipher.show_code_page());
        ui.add_space(16.0);
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

    crate::simple_cipher! {}
}
