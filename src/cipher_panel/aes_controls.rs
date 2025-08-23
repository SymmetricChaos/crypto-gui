use super::CipherFrame;
use crate::ui_elements::{block_cipher_iv_128, block_cipher_mode_and_padding, UiElements};
use ciphers::{
    digital::block_ciphers::aes::aes::{Aes128, Aes192, Aes256},
    Cipher,
};
use egui::Ui;
use rand::{thread_rng, Rng};
use utils::byte_formatting::ByteFormat;

macro_rules! interface {
    ($ui: ident, $cipher: expr, $key: expr, $bits: literal, $words: literal) => {
        block_cipher_mode_and_padding($ui, &mut $cipher.mode, &mut $cipher.padding);
        $ui.add_space(8.0);

        $ui.horizontal(|ui| {
            ui.subheading(format!("Key ({} bits)", $bits));
            if ui.random_bytes_button(&mut $key).clicked() {
                $cipher.ksa_u32($key);
            }
        });
        for i in 0..$words {
            if $ui.u32_hex_edit(&mut $key[i]).lost_focus() {
                $cipher.ksa_u32($key);
            }
        }

        $ui.add_space(8.0);

        block_cipher_iv_128($ui, &mut $cipher.iv, $cipher.mode);
        $ui.add_space(16.0);
    };
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum AesSelect {
    Aes128,
    Aes192,
    Aes256,
}

pub struct AesFrame {
    input_format: ByteFormat,
    output_format: ByteFormat,
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
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
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

        ui.selectable_value(&mut self.selector, AesSelect::Aes128, "AES-128");
        ui.selectable_value(&mut self.selector, AesSelect::Aes192, "AES-192");
        ui.selectable_value(&mut self.selector, AesSelect::Aes256, "AES-256");

        ui.randomize_reset_cipher(self);
        ui.add_space(16.0);

        ui.byte_io_mode_cipher(&mut self.input_format, &mut self.output_format);
        ui.add_space(8.0);

        match self.selector {
            AesSelect::Aes128 => {
                interface!(ui, self.cipher128, self.key128, "128", 4);
            }
            AesSelect::Aes192 => {
                interface!(ui, self.cipher192, self.key192, "192", 6);
            }
            AesSelect::Aes256 => {
                interface!(ui, self.cipher256, self.key256, "256", 8);
            }
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

    fn encrypt_string(&self, text: &str) -> Result<String, utils::errors::GeneralError> {
        match self.selector {
            AesSelect::Aes128 => self.cipher128.encrypt(text),
            AesSelect::Aes192 => self.cipher192.encrypt(text),
            AesSelect::Aes256 => self.cipher256.encrypt(text),
        }
    }

    fn decrypt_string(&self, text: &str) -> Result<String, utils::errors::GeneralError> {
        match self.selector {
            AesSelect::Aes128 => self.cipher128.decrypt(text),
            AesSelect::Aes192 => self.cipher192.decrypt(text),
            AesSelect::Aes256 => self.cipher256.decrypt(text),
        }
    }
}
