use super::CipherFrame;
use crate::ui_elements::{block_cipher_iv_128, block_cipher_mode_and_padding, UiElements};

use ciphers::{digital::block_ciphers::misty1::Misty1, Cipher};
use egui::Ui;
use rand::{thread_rng, Rng};

pub struct Misty1Frame {
    cipher: Misty1,
    key: [u32; 4],
}

impl Default for Misty1Frame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
            key: Default::default(),
        }
    }
}

impl CipherFrame for Misty1Frame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/tree/master/ciphers/src/digital/block_ciphers/misty1.rs",
        );
        ui.add_space(8.0);

        ui.randomize_reset(self);
        ui.add_space(16.0);

        ui.byte_io_mode_cipher(
            &mut self.cipher.input_format,
            &mut self.cipher.output_format,
        );

        ui.add_space(16.0);

        block_cipher_mode_and_padding(ui, &mut self.cipher.mode, &mut self.cipher.padding);
        ui.add_space(8.0);

        ui.horizontal(|ui| {
            ui.subheading("Key");
            if ui.random_bytes_button(&mut self.key).clicked() {
                self.cipher.ksa_u32(self.key);
            }
        });
        ui.label("MISTY1 uses a 128-bit key presented here as four 32-bit words.");
        for i in 0..4 {
            if ui.u32_hex_edit(&mut self.key[i]).changed() {
                self.cipher.ksa_u32(self.key);
            }
        }

        ui.add_space(8.0);

        block_cipher_iv_128(ui, &mut self.cipher128.iv, self.cipher128.mode);
        ui.add_space(16.0);
    }

    fn cipher(&self) -> &dyn Cipher {
        match self.selector {
            AesSelect::Aes128 => &self.cipher128,
            AesSelect::Aes192 => &self.cipher192,
            AesSelect::Aes256 => &self.cipher256,
        }
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        match self.selector {
            AesSelect::Aes128 => {
                for k in self.key128.iter_mut() {
                    *k = rng.gen()
                }
                self.cipher128.ksa_u32(self.key128);
                if self.cipher128.mode.iv_needed() {
                    self.cipher128.iv = rng.gen();
                }
            }
            AesSelect::Aes192 => {
                for k in self.key192.iter_mut() {
                    *k = rng.gen()
                }
                self.cipher192.ksa_u32(self.key192);
                if self.cipher192.mode.iv_needed() {
                    self.cipher192.iv = rng.gen();
                }
            }
            AesSelect::Aes256 => {
                for k in self.key256.iter_mut() {
                    *k = rng.gen()
                }
                self.cipher256.ksa_u32(self.key256);
                if self.cipher256.mode.iv_needed() {
                    self.cipher256.iv = rng.gen();
                }
            }
        }
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
