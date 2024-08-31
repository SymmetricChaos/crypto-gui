use super::CipherFrame;
use crate::ui_elements::{
    block_cipher_iv_128, block_cipher_mode, block_cipher_padding, UiElements,
};

use ciphers::{
    digital::block_ciphers::aes::aes::{Aes128, Aes192, Aes256},
    Cipher,
};
use egui::Ui;
use rand::{thread_rng, Rng};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum AesSelect {
    Aes128,
    Aes192,
    Aes256,
}

pub struct AesFrame {
    cipher128: Aes128,
    cipher192: Aes192,
    cipher256: Aes256,
    key128: [u32; 4],
    key192: [u32; 6],
    key256: [u32; 8],
    selector: AesSelect,
}

impl Default for AesFrame {
    fn default() -> Self {
        Self {
            cipher128: Default::default(),
            cipher192: Default::default(),
            cipher256: Default::default(),
            key128: Default::default(),
            key192: Default::default(),
            key256: Default::default(),
            selector: AesSelect::Aes128,
        }
    }
}

impl CipherFrame for AesFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/tree/master/ciphers/src/digital/block_ciphers/aes",
        );
        ui.add_space(8.0);

        ui.selectable_value(&mut self.selector, AesSelect::Aes128, "AES128");
        ui.selectable_value(&mut self.selector, AesSelect::Aes192, "AES192");
        ui.selectable_value(&mut self.selector, AesSelect::Aes256, "AES256");

        ui.randomize_reset(self);
        ui.add_space(16.0);

        match self.selector {
            AesSelect::Aes128 => {
                ui.byte_io_mode_cipher(
                    &mut self.cipher128.input_format,
                    &mut self.cipher128.output_format,
                );

                ui.add_space(16.0);

                block_cipher_mode(ui, &mut self.cipher128.mode);
                ui.add_space(4.0);
                block_cipher_padding(ui, &mut self.cipher128.padding);
                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    ui.subheading("Key");
                    if ui.random_bytes_button(&mut self.key128).clicked() {
                        self.cipher128.ksa_u32(self.key128);
                    }
                });
                ui.label("AES128 uses a 128-bit key presented here as four 32-bit words.");
                for i in 0..4 {
                    if ui.u32_drag_value_hex(&mut self.key128[i]).changed() {
                        self.cipher128.ksa_u32(self.key128);
                    }
                }

                ui.add_space(8.0);

                block_cipher_iv_128(ui, &mut self.cipher128.iv, self.cipher128.mode);
                ui.add_space(16.0);
            }
            AesSelect::Aes192 => {
                ui.byte_io_mode_cipher(
                    &mut self.cipher192.input_format,
                    &mut self.cipher192.output_format,
                );

                ui.add_space(16.0);

                block_cipher_mode(ui, &mut self.cipher192.mode);
                ui.add_space(4.0);
                block_cipher_padding(ui, &mut self.cipher192.padding);
                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    ui.subheading("Key");
                    if ui.random_bytes_button(&mut self.key192).clicked() {
                        self.cipher192.ksa_u32(self.key192);
                    }
                });
                ui.label("AES192 uses a 192-bit key presented here as six 32-bit words.");
                for i in 0..6 {
                    if ui.u32_drag_value_hex(&mut self.key192[i]).changed() {
                        self.cipher192.ksa_u32(self.key192);
                    }
                }

                ui.add_space(8.0);

                block_cipher_iv_128(ui, &mut self.cipher192.iv, self.cipher192.mode);
            }
            AesSelect::Aes256 => {
                ui.byte_io_mode_cipher(
                    &mut self.cipher256.input_format,
                    &mut self.cipher256.output_format,
                );

                ui.add_space(16.0);

                block_cipher_mode(ui, &mut self.cipher256.mode);
                ui.add_space(4.0);
                block_cipher_padding(ui, &mut self.cipher256.padding);
                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    ui.subheading("Key");
                    if ui.random_bytes_button(&mut self.key256).clicked() {
                        self.cipher256.ksa_u32(self.key256);
                    }
                });
                ui.label("AES256 uses a 256-bit key presented here as eight 32-bit words.");
                for i in 0..8 {
                    if ui.u32_drag_value_hex(&mut self.key256[i]).changed() {
                        self.cipher256.ksa_u32(self.key256);
                    }
                }

                ui.add_space(8.0);

                block_cipher_iv_128(ui, &mut self.cipher256.iv, self.cipher256.mode);
            }
        }
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
