use super::CipherFrame;
use crate::ui_elements::{block_cipher_iv_128, UiElements};

use ciphers::{
    digital::{
        block_ciphers::block_cipher::BCMode::Ctr,
        stream_ciphers::aes_gcm::aes_functions::{AesGcm128, AesGcm192, AesGcm256},
    },
    Cipher,
};
use egui::Ui;
use rand::{thread_rng, Rng};
use strum::IntoEnumIterator;
use utils::byte_formatting::{ByteFormat, ByteFormatError};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum AesGcmSelect {
    AesGcm128,
    AesGcm192,
    AesGcm256,
}

macro_rules! interface {
    ($ui: ident, $cipher: expr, $key: expr, $bits: literal, $words: literal, $ad_mode: expr, $ad: expr, $errors: expr) => {
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

        block_cipher_iv_128($ui, &mut $cipher.iv, Ctr);
        $ui.add_space(16.0);

        $ui.subheading("Associated Data");
        $ui.label("Arbitrary data can be associated with the message. This is usually data that cannot be encrypted such as routing information. The tag authenticates this data as well as the ciphertext.");
        $ui.horizontal(|ui| {
            for variant in ByteFormat::iter() {
                if ui
                    .selectable_value(&mut $ad_mode, variant, variant.to_string())
                    .clicked()
                {
                    match $ad_mode.text_to_bytes(&$ad) {
                        Ok(v) => $cipher.ad = v,
                        Err(_) => $errors.push_str("Error formatting associated data as bytes"),
                    }
                }
            }
        });
        if $ui.control_string(&mut $ad).lost_focus() {
            match $ad_mode.text_to_bytes(&$ad) {
                Ok(v) => $cipher.ad = v,
                Err(_) => {
                    $errors.push_str("Error formatting associated data as bytes");
                    $cipher.ad.clear();
                }
            }
        };
    };
}

pub struct AesGcmFrame {
    input_format: ByteFormat,
    output_format: ByteFormat,
    cipher128: AesGcm128,
    cipher192: AesGcm192,
    cipher256: AesGcm256,
    key128: [u32; 4],
    key192: [u32; 6],
    key256: [u32; 8],
    selector: AesGcmSelect,
    iv_input: ByteFormat,
    iv_string: String,
    iv_bytes: Result<Vec<u8>, ByteFormatError>,
    ad: String,
    ad_mode: ByteFormat,
}

impl Default for AesGcmFrame {
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
            selector: AesGcmSelect::AesGcm128,
            iv_input: ByteFormat::Hex,
            iv_string: String::from("a1b2c3d4e5f6a7b8c9d0eafb"),
            iv_bytes: Ok(vec![
                0xa1, 0xb2, 0xc3, 0xd4, 0xe5, 0xf6, 0xa7, 0xb8, 0xc9, 0xd0, 0xea, 0xfb,
            ]),
            ad: String::new(),
            ad_mode: ByteFormat::Hex,
        }
    }
}

impl AesGcmFrame {
    fn iv_controls(&mut self, ui: &mut Ui) {
        if ui.control_string(&mut self.iv_string).lost_focus() {
            self.iv_string = self
                .iv_string
                .chars()
                .filter(|c| c.is_ascii_hexdigit())
                .collect();
            if self.iv_string.len() % 2 != 0 {
                self.iv_string.insert(0, '0');
            }
            self.iv_bytes = self.iv_input.text_to_bytes(&self.iv_string);
            if let Ok(bytes) = &self.iv_bytes {
                self.cipher128.set_iv(bytes.clone());
                self.cipher192.set_iv(bytes.clone());
                self.cipher256.set_iv(bytes.clone());
            }
        };
        if self.iv_bytes.is_ok() {
            ui.heading("Initialization Vector");
            ui.monospace(match self.selector {
                AesGcmSelect::AesGcm128 => format!("{:032?}", self.cipher128.iv),
                AesGcmSelect::AesGcm192 => format!("{:032?}", self.cipher192.iv),
                AesGcmSelect::AesGcm256 => format!("{:032?}", self.cipher256.iv),
            });
        } else {
            ui.heading("Initialization Vector");
            ui.error_text("BYTES NOT ACCEPTED");
        }
    }

    fn ghash_display(&mut self, ui: &mut Ui) {
        let h = u128::from_be_bytes(match self.selector {
            AesGcmSelect::AesGcm128 => self.cipher128.h(),
            AesGcmSelect::AesGcm192 => self.cipher192.h(),
            AesGcmSelect::AesGcm256 => self.cipher256.h(),
        });
        let c = u128::from_be_bytes(match self.selector {
            AesGcmSelect::AesGcm128 => self.cipher128.c(),
            AesGcmSelect::AesGcm192 => self.cipher192.c(),
            AesGcmSelect::AesGcm256 => self.cipher256.c(),
        });
        ui.heading("GHASH H-value");
        ui.monospace(format!("{:032?}", h));
        ui.add_space(8.0);
        ui.heading("GHASH C-value");
        ui.monospace(format!("{:032?}", c));
    }
}

impl CipherFrame for AesGcmFrame {
    fn ui(&mut self, ui: &mut Ui, errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/tree/master/ciphers/src/digital/block_ciphers/aes",
        );
        ui.add_space(8.0);

        ui.selectable_value(&mut self.selector, AesGcmSelect::AesGcm128, "AES-GCM-128");
        ui.selectable_value(&mut self.selector, AesGcmSelect::AesGcm192, "AES-GCM-192");
        ui.selectable_value(&mut self.selector, AesGcmSelect::AesGcm256, "AES-GCM-256");

        ui.randomize_reset_cipher(self);
        ui.add_space(16.0);

        self.iv_controls(ui);

        ui.add_space(16.0);

        self.ghash_display(ui);

        ui.byte_io_mode_cipher(&mut self.input_format, &mut self.output_format);
        ui.add_space(8.0);

        match self.selector {
            AesGcmSelect::AesGcm128 => {
                interface!(
                    ui,
                    self.cipher128,
                    self.key128,
                    "128",
                    4,
                    self.ad_mode,
                    self.ad,
                    errors
                );
            }
            AesGcmSelect::AesGcm192 => {
                interface!(
                    ui,
                    self.cipher192,
                    self.key192,
                    "192",
                    6,
                    self.ad_mode,
                    self.ad,
                    errors
                );
            }
            AesGcmSelect::AesGcm256 => {
                interface!(
                    ui,
                    self.cipher256,
                    self.key256,
                    "256",
                    8,
                    self.ad_mode,
                    self.ad,
                    errors
                );
            }
        }
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        match self.selector {
            AesGcmSelect::AesGcm128 => {
                for k in self.key128.iter_mut() {
                    *k = rng.gen()
                }
                self.cipher128.ksa_u32(self.key128);
                self.cipher128.iv = rng.gen();
            }
            AesGcmSelect::AesGcm192 => {
                for k in self.key192.iter_mut() {
                    *k = rng.gen()
                }
                self.cipher192.ksa_u32(self.key192);

                self.cipher192.iv = rng.gen();
            }
            AesGcmSelect::AesGcm256 => {
                for k in self.key256.iter_mut() {
                    *k = rng.gen()
                }
                self.cipher256.ksa_u32(self.key256);

                self.cipher256.iv = rng.gen();
            }
        }
    }

    fn reset(&mut self) {
        *self = Self::default()
    }

    fn encrypt_string(&self, text: &str) -> Result<String, utils::errors::GeneralError> {
        match self.selector {
            AesGcmSelect::AesGcm128 => self.cipher128.encrypt(text),
            AesGcmSelect::AesGcm192 => self.cipher192.encrypt(text),
            AesGcmSelect::AesGcm256 => self.cipher256.encrypt(text),
        }
    }

    fn decrypt_string(&self, text: &str) -> Result<String, utils::errors::GeneralError> {
        match self.selector {
            AesGcmSelect::AesGcm128 => self.cipher128.decrypt(text),
            AesGcmSelect::AesGcm192 => self.cipher192.decrypt(text),
            AesGcmSelect::AesGcm256 => self.cipher256.decrypt(text),
        }
    }
}
