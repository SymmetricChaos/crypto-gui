use super::CipherFrame;
use crate::ui_elements::{block_cipher_iv_128, block_cipher_mode_and_padding, UiElements};

use ciphers::{
    digital::block_ciphers::lea::{Lea128, Lea192, Lea256},
    Cipher,
};
use egui::Ui;
use rand::{thread_rng, Rng};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum LeaSelect {
    Lea128,
    Lea192,
    Lea256,
}

pub struct LeaFrame {
    cipher128: Lea128,
    cipher192: Lea192,
    cipher256: Lea256,
    key128: [u32; 4],
    key192: [u32; 6],
    key256: [u32; 8],
    selector: LeaSelect,
}

impl Default for LeaFrame {
    fn default() -> Self {
        Self {
            cipher128: Default::default(),
            cipher192: Default::default(),
            cipher256: Default::default(),
            key128: Default::default(),
            key192: Default::default(),
            key256: Default::default(),
            selector: LeaSelect::Lea128,
        }
    }
}

impl CipherFrame for LeaFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/tree/master/ciphers/src/digital/block_ciphers/Lea",
        );
        ui.add_space(8.0);

        ui.selectable_value(&mut self.selector, LeaSelect::Lea128, "Lea128");
        ui.selectable_value(&mut self.selector, LeaSelect::Lea192, "Lea192");
        ui.selectable_value(&mut self.selector, LeaSelect::Lea256, "Lea256");

        ui.randomize_reset_cipher(self);
        ui.add_space(16.0);

        match self.selector {
            LeaSelect::Lea128 => {
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
                    ui.subheading("Key");
                    if ui.random_bytes_button(&mut self.key128).clicked() {
                        self.cipher128.ksa_u32(self.key128);
                    }
                });
                ui.label("LEA-128 uses a 128-bit key presented here as four 32-bit words.");
                for i in 0..4 {
                    if ui.u32_hex_edit(&mut self.key128[i]).changed() {
                        self.cipher128.ksa_u32(self.key128);
                    }
                }

                ui.add_space(8.0);

                block_cipher_iv_128(ui, &mut self.cipher128.iv, self.cipher128.mode);
                ui.add_space(16.0);
            }
            LeaSelect::Lea192 => {
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
                    ui.subheading("Key");
                    if ui.random_bytes_button(&mut self.key192).clicked() {
                        self.cipher192.ksa_u32(self.key192);
                    }
                });
                ui.label("LEA-192 uses a 192-bit key presented here as six 32-bit words.");
                for i in 0..6 {
                    if ui.u32_hex_edit(&mut self.key192[i]).changed() {
                        self.cipher192.ksa_u32(self.key192);
                    }
                }

                ui.add_space(8.0);

                block_cipher_iv_128(ui, &mut self.cipher192.iv, self.cipher192.mode);
            }
            LeaSelect::Lea256 => {
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
                    ui.subheading("Key");
                    if ui.random_bytes_button(&mut self.key256).clicked() {
                        self.cipher256.ksa_u32(self.key256);
                    }
                });
                ui.label("LEA-256 uses a 256-bit key presented here as eight 32-bit words.");
                for i in 0..8 {
                    if ui.u32_hex_edit(&mut self.key256[i]).changed() {
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
            LeaSelect::Lea128 => &self.cipher128,
            LeaSelect::Lea192 => &self.cipher192,
            LeaSelect::Lea256 => &self.cipher256,
        }
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        match self.selector {
            LeaSelect::Lea128 => {
                for k in self.key128.iter_mut() {
                    *k = rng.gen()
                }
                self.cipher128.ksa_u32(self.key128);
                if self.cipher128.mode.iv_needed() {
                    self.cipher128.iv = rng.gen();
                }
            }
            LeaSelect::Lea192 => {
                for k in self.key192.iter_mut() {
                    *k = rng.gen()
                }
                self.cipher192.ksa_u32(self.key192);
                if self.cipher192.mode.iv_needed() {
                    self.cipher192.iv = rng.gen();
                }
            }
            LeaSelect::Lea256 => {
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
