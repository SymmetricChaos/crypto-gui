use super::CipherFrame;
use crate::ui_elements::{block_cipher_iv_64, block_cipher_mode, block_cipher_padding, UiElements};
use ciphers::{digital::block_ciphers::fealnx::FealNx, Cipher};
use rand::thread_rng;

#[derive(Default)]
pub struct FealNxFrame {
    cipher: FealNx,
    key: [u32; 4],
}

impl CipherFrame for FealNxFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/tree/master/ciphers/src/digital/block_ciphers/fealnx.rs",
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

        ui.label("FEAL-NX uses a 128-bit key presented here as four 32-bit words.");
        for i in 0..4 {
            if ui.u32_hex_edit(&mut self.key[i]).changed() {
                self.cipher.ksa_u32(self.key);
            }
        }

        ui.add_space(8.0);

        block_cipher_iv_64(ui, &mut self.cipher.iv, self.cipher.mode);
        ui.add_space(16.0);
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
