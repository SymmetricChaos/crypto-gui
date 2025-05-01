use super::CipherFrame;
use crate::ui_elements::{block_cipher_iv_128, block_cipher_mode_and_padding, UiElements};
use ciphers::{
    digital::block_ciphers::twofish::twofish::{Twofish128, Twofish192, Twofish256},
    Cipher,
};
use egui::Ui;
use rand::{thread_rng, Rng};

macro_rules! interface {
    ($ui: ident, $cipher: expr, $key: expr, $bits: literal, $words: literal) => {
        $ui.byte_io_mode_cipher(&mut $cipher.input_format, &mut $cipher.output_format);
        $ui.add_space(8.0);

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
enum TwofishSelect {
    Twofish128,
    Twofish192,
    Twofish256,
}

pub struct TwofishFrame {
    cipher128: Twofish128,
    cipher192: Twofish192,
    cipher256: Twofish256,
    key128: [u32; 4],
    key192: [u32; 6],
    key256: [u32; 8],
    selector: TwofishSelect,
}

impl Default for TwofishFrame {
    fn default() -> Self {
        Self {
            cipher128: Default::default(),
            cipher192: Default::default(),
            cipher256: Default::default(),
            key128: Default::default(),
            key192: Default::default(),
            key256: Default::default(),
            selector: TwofishSelect::Twofish128,
        }
    }
}

impl CipherFrame for TwofishFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/tree/master/ciphers/src/digital/block_ciphers/twofish",
        );
        ui.add_space(8.0);

        ui.selectable_value(&mut self.selector, TwofishSelect::Twofish128, "Twofish-128");
        ui.selectable_value(&mut self.selector, TwofishSelect::Twofish192, "Twofish-192");
        ui.selectable_value(&mut self.selector, TwofishSelect::Twofish256, "Twofish-256");

        ui.randomize_reset_cipher(self);
        ui.add_space(16.0);

        match self.selector {
            TwofishSelect::Twofish128 => {
                interface!(ui, self.cipher128, self.key128, "128", 4);
            }
            TwofishSelect::Twofish192 => {
                interface!(ui, self.cipher192, self.key192, "192", 6);
            }
            TwofishSelect::Twofish256 => {
                interface!(ui, self.cipher256, self.key256, "256", 8);
            }
        }
    }

    fn cipher(&self) -> &dyn Cipher {
        match self.selector {
            TwofishSelect::Twofish128 => &self.cipher128,
            TwofishSelect::Twofish192 => &self.cipher192,
            TwofishSelect::Twofish256 => &self.cipher256,
        }
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        match self.selector {
            TwofishSelect::Twofish128 => {
                for k in self.key128.iter_mut() {
                    *k = rng.gen()
                }
                self.cipher128.ksa_u32(self.key128);
                if self.cipher128.mode.iv_needed() {
                    self.cipher128.iv = rng.gen();
                }
            }
            TwofishSelect::Twofish192 => {
                for k in self.key192.iter_mut() {
                    *k = rng.gen()
                }
                self.cipher192.ksa_u32(self.key192);
                if self.cipher192.mode.iv_needed() {
                    self.cipher192.iv = rng.gen();
                }
            }
            TwofishSelect::Twofish256 => {
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
