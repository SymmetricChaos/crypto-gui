use super::CipherFrame;
use crate::ui_elements::UiElements;

use ciphers::digital::secret_splitting::XorSecretSplitting;
use egui::Slider;

pub struct XorSecretSplittingFrame {
    cipher: XorSecretSplitting,
}

impl Default for XorSecretSplittingFrame {
    fn default() -> Self {
        Self {
            cipher: XorSecretSplitting::default(),
        }
    }
}

impl CipherFrame for XorSecretSplittingFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/ciphers/src/digital/secret_splitting.rs",
        );
        ui.add_space(8.0);

        ui.byte_io_mode_cipher(
            &mut self.cipher.input_format,
            &mut self.cipher.output_format,
        );

        ui.add_space(8.0);

        ui.subheading("Number of Pieces");
        ui.add(Slider::new(&mut self.cipher.n_splits, 2..=40));

        ui.add_space(16.0);
    }

    fn cipher(&self) -> &dyn ciphers::Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        todo!()
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
