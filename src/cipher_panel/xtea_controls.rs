use super::CipherFrame;
use crate::ui_elements::{block_cipher_iv_64, block_cipher_mode_and_padding, UiElements};
use ciphers::{digital::block_ciphers::tea::xtea::Xtea, Cipher};
use egui::Ui;
use rand::{thread_rng, Rng};

pub struct XteaFrame {
    cipher: Xtea,
}

impl Default for XteaFrame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
        }
    }
}

impl CipherFrame for XteaFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/ciphers/src/digital/block_ciphers/tea/xtea.rs",
        );
        ui.add_space(8.0);

        ui.randomize_reset_cipher(self);
        ui.add_space(16.0);

        ui.byte_io_mode_cipher(
            &mut self.cipher.input_format,
            &mut self.cipher.output_format,
        );

        ui.add_space(16.0);

        block_cipher_mode_and_padding(ui, &mut self.cipher.mode, &mut self.cipher.padding);
        ui.add_space(8.0);

        ui.horizontal(|ui| {
            ui.subheading("Key (128 bits)");
            ui.random_bytes_button(&mut self.cipher.subkeys)
        });
        for i in 0..4 {
            ui.u32_hex_edit(&mut self.cipher.subkeys[i]);
        }

        ui.add_space(8.0);

        block_cipher_iv_64(ui, &mut self.cipher.iv, self.cipher.mode);

        ui.add_space(16.0);
    }


    fn randomize(&mut self) {
        let mut rng = thread_rng();
        self.cipher.subkeys[0] = rng.gen();
        self.cipher.subkeys[1] = rng.gen();
        self.cipher.subkeys[2] = rng.gen();
        self.cipher.subkeys[3] = rng.gen();

        if self.cipher.mode.iv_needed() {
            self.cipher.iv = rng.gen();
        }
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
