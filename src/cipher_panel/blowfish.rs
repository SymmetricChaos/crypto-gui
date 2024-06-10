use ciphers::{
    digital::{blowfish::Blowfish, BlockCipherMode},
    Cipher,
};
use rand::{thread_rng, Rng};

use crate::ui_elements::{block_cipher_mode, UiElements};

use super::CipherFrame;

pub struct BlowfishFrame {
    cipher: Blowfish,
}

impl Default for BlowfishFrame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
        }
    }
}

impl CipherFrame for BlowfishFrame {
    fn ui(&mut self, ui: &mut egui::Ui, errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/ciphers/src/digital/blowfish.rs",
        );
        ui.add_space(8.0);

        ui.randomize_reset(self);
        ui.add_space(16.0);

        ui.byte_io_mode(
            &mut self.cipher.input_format,
            &mut self.cipher.output_format,
        );

        ui.add_space(16.0);

        block_cipher_mode(ui, &mut self.cipher.mode);
        ui.add_space(8.0);
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        self.cipher.key.clear();
        for _ in 0..rng.gen_range(16..=32) {
            self.cipher.key.push(rng.gen());
        }

        if self.cipher.mode == BlockCipherMode::Ctr {
            self.cipher.ctr = rng.gen();
        }
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
