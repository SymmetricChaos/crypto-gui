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
use utils::byte_formatting::{ByteFormat, ByteFormatError};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum AesGcmSelect {
    AesGcm128,
    AesGcm192,
    AesGcm256,
}

pub struct AesGcmFrame {
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
}

impl Default for AesGcmFrame {
    fn default() -> Self {
        Self {
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
        }
    }
}

impl AesGcmFrame {
    fn iv_controls(&mut self, ui: &mut Ui) {
        if ui.control_string(&mut self.iv_string).changed() {
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
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/tree/master/ciphers/src/digital/block_ciphers/aes",
        );
        ui.add_space(8.0);

        ui.selectable_value(&mut self.selector, AesGcmSelect::AesGcm128, "AES-GCM-128");
        ui.selectable_value(&mut self.selector, AesGcmSelect::AesGcm192, "AES-GCM-192");
        ui.selectable_value(&mut self.selector, AesGcmSelect::AesGcm256, "AES-GCM-256");

        ui.randomize_reset(self);
        ui.add_space(16.0);

        self.iv_controls(ui);

        ui.add_space(16.0);

        self.ghash_display(ui);

        match self.selector {
            AesGcmSelect::AesGcm128 => {
                ui.byte_io_mode_cipher(
                    &mut self.cipher128.input_format,
                    &mut self.cipher128.output_format,
                );

                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    ui.subheading("Key");
                    if ui.random_bytes_button(&mut self.key128).clicked() {
                        self.cipher128.ksa_u32(self.key128);
                    }
                });
                ui.label("AES-GCM-128 uses a 128-bit key presented here as four 32-bit words.");
                for i in 0..4 {
                    if ui.u32_hex_edit(&mut self.key128[i]).changed() {
                        self.cipher128.ksa_u32(self.key128);
                    }
                }

                ui.add_space(8.0);

                block_cipher_iv_128(ui, &mut self.cipher128.iv, Ctr);
                ui.add_space(16.0);
            }
            AesGcmSelect::AesGcm192 => {
                ui.byte_io_mode_cipher(
                    &mut self.cipher192.input_format,
                    &mut self.cipher192.output_format,
                );

                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    ui.subheading("Key");
                    if ui.random_bytes_button(&mut self.key192).clicked() {
                        self.cipher192.ksa_u32(self.key192);
                    }
                });
                ui.label("AES-GCM-192 uses a 192-bit key presented here as six 32-bit words.");
                for i in 0..6 {
                    if ui.u32_hex_edit(&mut self.key192[i]).changed() {
                        self.cipher192.ksa_u32(self.key192);
                    }
                }

                ui.add_space(8.0);

                block_cipher_iv_128(ui, &mut self.cipher192.iv, Ctr);
            }
            AesGcmSelect::AesGcm256 => {
                ui.byte_io_mode_cipher(
                    &mut self.cipher256.input_format,
                    &mut self.cipher256.output_format,
                );

                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    ui.subheading("Key");
                    if ui.random_bytes_button(&mut self.key256).clicked() {
                        self.cipher256.ksa_u32(self.key256);
                    }
                });
                ui.label("AES-GCM-256 uses a 256-bit key presented here as eight 32-bit words.");
                for i in 0..8 {
                    if ui.u32_hex_edit(&mut self.key256[i]).changed() {
                        self.cipher256.ksa_u32(self.key256);
                    }
                }

                ui.add_space(8.0);

                block_cipher_iv_128(ui, &mut self.cipher256.iv, Ctr);
            }
        }
    }

    fn cipher(&self) -> &dyn Cipher {
        match self.selector {
            AesGcmSelect::AesGcm128 => &self.cipher128,
            AesGcmSelect::AesGcm192 => &self.cipher192,
            AesGcmSelect::AesGcm256 => &self.cipher256,
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
}
