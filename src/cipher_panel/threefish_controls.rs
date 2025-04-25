use super::CipherFrame;
use crate::ui_elements::{block_cipher_mode_and_padding, UiElements};
use ciphers::{
    digital::block_ciphers::threefish::{Threefish1024, Threefish256, Threefish512},
    Cipher,
};
use egui::Ui;
use rand::{thread_rng, Rng};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum ThreefishSelect {
    Threefish256,
    Threefish512,
    Threefish1024,
}

pub struct ThreefishFrame {
    cipher256: Threefish256,
    cipher512: Threefish512,
    cipher1024: Threefish1024,
    key256: [u64; 4],
    key512: [u64; 8],
    key1024: [u64; 16],
    tweak: [u64; 2],
    selector: ThreefishSelect,
}

impl Default for ThreefishFrame {
    fn default() -> Self {
        Self {
            cipher256: Default::default(),
            cipher512: Default::default(),
            cipher1024: Default::default(),
            key256: Default::default(),
            key512: Default::default(),
            key1024: Default::default(),
            tweak: Default::default(),
            selector: ThreefishSelect::Threefish256,
        }
    }
}

impl CipherFrame for ThreefishFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/tree/master/ciphers/src/digital/block_ciphers/threefish",
        );
        ui.add_space(8.0);

        ui.selectable_value(
            &mut self.selector,
            ThreefishSelect::Threefish256,
            "Threefish256",
        );
        ui.selectable_value(
            &mut self.selector,
            ThreefishSelect::Threefish512,
            "Threefish512",
        );
        ui.selectable_value(
            &mut self.selector,
            ThreefishSelect::Threefish1024,
            "Threefish1024",
        );

        ui.randomize_reset_cipher(self);
        ui.add_space(16.0);

        match self.selector {
            ThreefishSelect::Threefish256 => {
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
                        self.cipher256 =
                            Threefish256::with_key_and_tweak_u64(&self.key256, &self.tweak);
                    }
                });
                ui.label("Threefish256 uses a 256-bit key presented here as four 64-bit words.");
                for i in 0..4 {
                    if ui.u64_hex_edit(&mut self.key256[i]).changed() {
                        self.cipher256 =
                            Threefish256::with_key_and_tweak_u64(&self.key256, &self.tweak);
                    }
                }

                ui.add_space(8.0);

                block_cipher_iv_256(ui, &mut self.cipher256.iv, self.cipher256.mode);
                ui.add_space(16.0);
            }
            ThreefishSelect::Threefish512 => {
                ui.byte_io_mode_cipher(
                    &mut self.cipher512.input_format,
                    &mut self.cipher512.output_format,
                );

                ui.add_space(16.0);

                block_cipher_mode_and_padding(
                    ui,
                    &mut self.cipher512.mode,
                    &mut self.cipher512.padding,
                );
                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    ui.subheading("Key");
                    if ui.random_bytes_button(&mut self.key512).clicked() {
                        self.cipher512 =
                            Threefish512::with_key_and_tweak_u64(&self.key512, &self.tweak);
                    }
                });
                ui.label("Threefish512 uses a 512-bit key presented here as eight 64-bit words.");
                for i in 0..6 {
                    if ui.u64_hex_edit(&mut self.key512[i]).changed() {
                        self.cipher512 =
                            Threefish512::with_key_and_tweak_u64(&self.key512, &self.tweak);
                    }
                }

                ui.add_space(8.0);

                block_cipher_iv_512(ui, &mut self.cipher512.iv, self.cipher512.mode);
            }
            ThreefishSelect::Threefish1024 => {
                ui.byte_io_mode_cipher(
                    &mut self.cipher1024.input_format,
                    &mut self.cipher1024.output_format,
                );

                ui.add_space(16.0);

                block_cipher_mode_and_padding(
                    ui,
                    &mut self.cipher1024.mode,
                    &mut self.cipher1024.padding,
                );
                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    ui.subheading("Key");
                    if ui.random_bytes_button(&mut self.key1024).clicked() {
                        self.cipher1024 =
                            Threefish1024::with_key_and_tweak_u64(&self.key1024, &self.tweak);
                    }
                });
                ui.label(
                    "Threefish1024 uses a 1024-bit key presented here as sixteen 64-bit words.",
                );
                for i in 0..8 {
                    if ui.u64_hex_edit(&mut self.key1024[i]).changed() {
                        self.cipher1024 =
                            Threefish1024::with_key_and_tweak_u64(&self.key1024, &self.tweak);
                    }
                }

                ui.add_space(8.0);

                block_cipher_iv_1024(ui, &mut self.cipher256.iv, self.cipher256.mode);
            }
        }
    }

    fn cipher(&self) -> &dyn Cipher {
        match self.selector {
            ThreefishSelect::Threefish256 => &self.cipher256,
            ThreefishSelect::Threefish512 => &self.cipher512,
            ThreefishSelect::Threefish1024 => &self.cipher1024,
        }
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        match self.selector {
            ThreefishSelect::Threefish256 => {
                for k in self.key256.iter_mut() {
                    *k = rng.gen()
                }
                self.cipher256 = Threefish256::with_key_and_tweak_u64(&self.key256, &self.tweak);
                if self.cipher256.mode.iv_needed() {
                    self.cipher256.iv = rng.gen();
                }
            }
            ThreefishSelect::Threefish512 => {
                for k in self.key512.iter_mut() {
                    *k = rng.gen()
                }
                self.cipher512 = Threefish512::with_key_and_tweak_u64(&self.key512, &self.tweak);
                if self.cipher512.mode.iv_needed() {
                    self.cipher512.iv = rng.gen();
                }
            }
            ThreefishSelect::Threefish1024 => {
                for k in self.key1024.iter_mut() {
                    *k = rng.gen()
                }
                self.cipher1024 = Threefish1024::with_key_and_tweak_u64(&self.key1024, &self.tweak);
                if self.cipher1024.mode.iv_needed() {
                    self.cipher1024.iv = rng.gen();
                }
            }
        }
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
