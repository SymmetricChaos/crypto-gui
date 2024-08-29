use crate::ui_elements::{block_cipher_mode, block_cipher_padding, UiElements};
use ciphers::digital::block_ciphers::seed::Seed;

use super::CipherFrame;

pub struct SeedFrame {
    cipher: Seed,
    key: [u32; 4],
}

impl Default for SeedFrame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
            key: Default::default(),
        }
    }
}

impl CipherFrame for SeedFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/tree/master/ciphers/src/digital/block_ciphers/seed.rs",
        );
        ui.add_space(8.0);

        ui.randomize_reset(self);
        ui.add_space(16.0);

        ui.byte_io_mode_cipher(
            &mut self.cipher.input_format,
            &mut self.cipher.output_format,
        );

        ui.add_space(16.0);

        block_cipher_mode(ui, &mut self.cipher.mode);
        ui.add_space(4.0);
        block_cipher_padding(ui, &mut self.cipher.padding);
        ui.add_space(8.0);
    }

    fn cipher(&self) -> &dyn ciphers::Cipher {
        todo!()
    }

    fn randomize(&mut self) {
        todo!()
    }

    fn reset(&mut self) {
        todo!()
    }
}
