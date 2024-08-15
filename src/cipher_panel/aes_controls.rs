use super::CipherFrame;
use crate::ui_elements::{block_cipher_mode, block_cipher_padding, UiElements};

use ciphers::{
    digital::block_ciphers::aes::aes::{Aes128, Aes192, Aes256},
    Cipher,
};
use egui::{DragValue, Ui};
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
    iv_upper: u64,
    iv_lower: u64,
    selector: AesSelect,
}

impl Default for AesFrame {
    fn default() -> Self {
        Self {
            cipher128: Default::default(),
            cipher192: Default::default(),
            cipher256: Default::default(),
            iv_upper: 0,
            iv_lower: 0,
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

                ui.subheading("Key");
                ui.label("AES128 uses four 32-bit keys or, equivalently, a single 128-bit key.");
                for k in self.cipher128.key.iter_mut() {
                    ui.u32_drag_value_hex(k);
                }

                ui.add_space(8.0);

                ui.add_enabled_ui(self.cipher128.mode.iv_needed(), |ui| {
                    ui.subheading("Initialization Vector");
                    ui.label(format!("In {} mode the cipher must have a 128-bit initialization vector provided. The selectors below control the upper and lower 64-bits respectively.",self.cipher128.mode));
                    if ui.add(DragValue::new(&mut self.iv_upper).hexadecimal(16, false, false)) .changed() {
                        self.cipher128.iv &= 0x0000000000000000FFFFFFFFFFFFFFFF;
                        self.cipher128.iv |= (self.iv_upper as u128) << 64;
                    }
                    if ui.add(DragValue::new(&mut self.iv_lower).hexadecimal(16, false, false)) .changed() {
                        self.cipher128.iv &= 0xFFFFFFFFFFFFFFFF0000000000000000;
                        self.cipher128.iv |= self.iv_lower as u128;
                    }
                    ui.label(format!("{:032x?}",self.cipher128.iv))
                });

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

                ui.subheading("Key");
                ui.label("AES192 uses siz 32-bit keys or, equivalently, a single 192-bit key.");
                for k in self.cipher192.key.iter_mut() {
                    ui.u32_drag_value_hex(k);
                }

                ui.add_space(8.0);

                ui.add_enabled_ui(self.cipher192.mode.iv_needed(), |ui| {
                    ui.subheading("Initialization Vector");
                    ui.label(format!("In {} mode the cipher must have a 128-bit initialization vector provided. The selectors below control the upper and lower 64-bits respectively.",self.cipher192.mode));
                    if ui.add(DragValue::new(&mut self.iv_upper).hexadecimal(16, false, false)) .changed() {
                        self.cipher192.iv &= 0x0000000000000000FFFFFFFFFFFFFFFF;
                        self.cipher192.iv |= (self.iv_upper as u128) << 64;
                    }
                    if ui.add(DragValue::new(&mut self.iv_lower).hexadecimal(16, false, false)) .changed() {
                        self.cipher192.iv &= 0xFFFFFFFFFFFFFFFF0000000000000000;
                        self.cipher192.iv |= self.iv_lower as u128;
                    }
                    ui.label(format!("{:032x?}",self.cipher192.iv))
                });
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

                ui.subheading("Key");
                ui.label("AES256 uses eight 32-bit keys or, equivalently, a single 256-bit key.");
                for k in self.cipher256.key.iter_mut() {
                    ui.u32_drag_value_hex(k);
                }

                ui.add_space(8.0);

                ui.add_enabled_ui(self.cipher256.mode.iv_needed(), |ui| {
                    ui.subheading("Initialization Vector");
                    ui.label(format!("In {} mode the cipher must have a 128-bit initialization vector provided. The selectors below control the upper and lower 64-bits respectively.",self.cipher256.mode));
                    if ui.add(DragValue::new(&mut self.iv_upper).hexadecimal(16, false, false)) .changed() {
                        self.cipher256.iv &= 0x0000000000000000FFFFFFFFFFFFFFFF;
                        self.cipher256.iv |= (self.iv_upper as u128) << 64;
                    }
                    if ui.add(DragValue::new(&mut self.iv_lower).hexadecimal(16, false, false)) .changed() {
                        self.cipher256.iv &= 0xFFFFFFFFFFFFFFFF0000000000000000;
                        self.cipher256.iv |= self.iv_lower as u128;
                    }
                    ui.label(format!("{:032x?}",self.cipher256.iv))
                });
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
                for k in self.cipher128.key.iter_mut() {
                    *k = rng.gen()
                }
                if self.cipher128.mode.iv_needed() {
                    self.cipher128.iv = rng.gen();
                }
            }
            AesSelect::Aes192 => {
                for k in self.cipher192.key.iter_mut() {
                    *k = rng.gen()
                }
                if self.cipher192.mode.iv_needed() {
                    self.cipher192.iv = rng.gen();
                }
            }
            AesSelect::Aes256 => {
                for k in self.cipher256.key.iter_mut() {
                    *k = rng.gen()
                }
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
