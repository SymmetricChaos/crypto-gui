use super::CipherFrame;
use crate::ui_elements::UiElements;
use ciphers::transposition::skip_cipher::SkipCipher;
use egui::{DragValue, Ui};
use rand::{thread_rng, Rng};

pub struct SkipCipherFrame {
    cipher: SkipCipher,
}

impl Default for SkipCipherFrame {
    fn default() -> Self {
        Self {
            cipher: SkipCipher::default(),
        }
    }
}

impl CipherFrame for SkipCipherFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/ciphers/src/transposition/skip_cipher.rs",
        );
        ui.add_space(8.0);

        ui.randomize_reset_cipher(self);
        ui.add_space(16.0);

        ui.subheading("Starting Position");
        ui.add(DragValue::new(&mut self.cipher.initial));
        ui.add_space(4.0);

        ui.subheading("Number of Letters to Skip");
        if ui.add(DragValue::new(&mut self.cipher.skip)).changed() {
            if self.cipher.skip == 0 {
                self.cipher.skip = 1;
            }
        }
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        self.cipher.initial = rng.gen_range(0..=20);
        self.cipher.skip = rng.gen_range(1..=20)
    }

    crate::simple_cipher! {}
}
