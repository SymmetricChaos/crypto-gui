use super::CipherFrame;
use crate::ui_elements::{block_cipher_iv_128, block_cipher_mode_and_padding, UiElements};

use ciphers::{
    digital::block_ciphers::aria::{Aria128, Aria192, Aria256},
    Cipher,
};
use egui::Ui;
use rand::{thread_rng, Rng};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum AriaSelect {
    Aria128,
    Aria192,
    Aria256,
}

pub struct AriaFrame {
    cipher128: Aria128,
    cipher192: Aria192,
    cipher256: Aria256,
    key128: [u32; 4],
    key192: [u32; 6],
    key256: [u32; 8],
    selector: AriaSelect,
}

impl Default for AriaFrame {
    fn default() -> Self {
        Self {
            cipher128: Default::default(),
            cipher192: Default::default(),
            cipher256: Default::default(),
            key128: Default::default(),
            key192: Default::default(),
            key256: Default::default(),
            selector: AriaSelect::Aria128,
        }
    }
}

impl CipherFrame for AriaFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/tree/master/ciphers/src/digital/block_ciphers/aria.rs",
        );
        ui.add_space(8.0);

        ui.selectable_value(&mut self.selector, AriaSelect::Aria128, "Aria-128");
        ui.selectable_value(&mut self.selector, AriaSelect::Aria192, "Aria-192");
        ui.selectable_value(&mut self.selector, AriaSelect::Aria256, "Aria-256");

        ui.randomize_reset_cipher(self);
        ui.add_space(16.0);

        match self.selector {
            AriaSelect::Aria128 => {
                ui.byte_io_mode_cipher(
                    &mut self.cipher128.input_format,
                    &mut self.cipher128.output_format,
                );

                ui.add_space(16.0);

                block_cipher_mode_and_padding(
                    ui,
                    &mut self.cipher128.mode,
                    &mut self.cipher128.padding,
                );
                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    ui.subheading("Key (128-bits)");
                    if ui.random_bytes_button(&mut self.key128).clicked() {
                        self.cipher128.ksa_u32(self.key128);
                    }
                });
                for i in 0..4 {
                    if ui.u32_hex_edit(&mut self.key128[i]).lost_focus() {
                        self.cipher128.ksa_u32(self.key128);
                    }
                }

                ui.add_space(8.0);

                block_cipher_iv_128(ui, &mut self.cipher128.iv, self.cipher128.mode);
                ui.add_space(16.0);
            }
            AriaSelect::Aria192 => {
                ui.byte_io_mode_cipher(
                    &mut self.cipher192.input_format,
                    &mut self.cipher192.output_format,
                );

                ui.add_space(16.0);

                block_cipher_mode_and_padding(
                    ui,
                    &mut self.cipher192.mode,
                    &mut self.cipher192.padding,
                );
                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    ui.subheading("Key (192-bits)");
                    if ui.random_bytes_button(&mut self.key192).clicked() {
                        self.cipher192.ksa_u32(self.key192);
                    }
                });
                for i in 0..6 {
                    if ui.u32_hex_edit(&mut self.key192[i]).lost_focus() {
                        self.cipher192.ksa_u32(self.key192);
                    }
                }

                ui.add_space(8.0);

                block_cipher_iv_128(ui, &mut self.cipher192.iv, self.cipher192.mode);
            }
            AriaSelect::Aria256 => {
                ui.byte_io_mode_cipher(
                    &mut self.cipher256.input_format,
                    &mut self.cipher256.output_format,
                );

                ui.add_space(16.0);

                block_cipher_mode_and_padding(
                    ui,
                    &mut self.cipher256.mode,
                    &mut self.cipher256.padding,
                );
                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    ui.subheading("Key (256-bits)");
                    if ui.random_bytes_button(&mut self.key256).clicked() {
                        self.cipher256.ksa_u32(self.key256);
                    }
                });
                for i in 0..8 {
                    if ui.u32_hex_edit(&mut self.key256[i]).lost_focus() {
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
            AriaSelect::Aria128 => &self.cipher128,
            AriaSelect::Aria192 => &self.cipher192,
            AriaSelect::Aria256 => &self.cipher256,
        }
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        match self.selector {
            AriaSelect::Aria128 => {
                for k in self.key128.iter_mut() {
                    *k = rng.gen()
                }
                self.cipher128.ksa_u32(self.key128);
                if self.cipher128.mode.iv_needed() {
                    self.cipher128.iv = rng.gen();
                }
            }
            AriaSelect::Aria192 => {
                for k in self.key192.iter_mut() {
                    *k = rng.gen()
                }
                self.cipher192.ksa_u32(self.key192);
                if self.cipher192.mode.iv_needed() {
                    self.cipher192.iv = rng.gen();
                }
            }
            AriaSelect::Aria256 => {
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
