use super::CipherFrame;
use crate::ui_elements::UiElements;
use ciphers::tactical::Dryad;
use egui::Ui;
use rand::thread_rng;
use utils::{preset_alphabet::Alphabet, text_functions::shuffled_str};

#[derive(Default)]
pub struct DryadFrame {
    cipher: Dryad,
}

impl CipherFrame for DryadFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/ciphers/src/tactical/dryad.rs",
        );
        ui.add_space(8.0);

        ui.randomize_reset_cipher(self);
        ui.add_space(16.0);

        ui.subheading("Message Key");
        ui.string_slider("ABCDEFGHIJKLMNOPQRSTUVWXY", &mut self.cipher.message_key);

        ui.add_space(16.0);

        ui.horizontal(|ui| {
            ui.subheading("Code Page");
            ui.copy_to_clipboard(self.cipher.show_code_page());
        });
        ui.mono(self.cipher.show_code_page());
        ui.add_space(16.0);
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        let alpha = Alphabet::BasicLatin.slice();
        for row in self.cipher.cipher_rows.iter_mut() {
            *row = shuffled_str(alpha, &mut rng)
        }
    }

    crate::simple_cipher! {}
}
