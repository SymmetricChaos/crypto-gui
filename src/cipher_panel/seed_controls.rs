use crate::ui_elements::{
    block_cipher_iv_128, block_cipher_mode, block_cipher_padding, UiElements,
};
use ciphers::digital::block_ciphers::seed::Seed;
use utils::byte_formatting::u32s_to_bytes_le;

use super::CipherFrame;

pub struct SeedFrame {
    cipher: Seed,
    key: [u32; 4],
    key_bytes: [u8; 16],
}

impl Default for SeedFrame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
            key: Default::default(),
            key_bytes: Default::default(),
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

        ui.randomize_reset_cipher(self);
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

        ui.subheading("Key (128 bits)");
        for i in 0..4 {
            if ui.u32_hex_edit(&mut self.key[i]).changed() {
                u32s_to_bytes_le(&mut self.key_bytes, &self.key);
                self.cipher.ksa(self.key_bytes);
            }
        }

        block_cipher_iv_128(ui, &mut self.cipher.iv, self.cipher.mode);
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
