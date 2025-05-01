use super::CipherFrame;
use crate::ui_elements::{block_cipher_mode_and_padding, UiElements};
use ciphers::{
    digital::block_ciphers::threefish::{Threefish1024, Threefish256, Threefish512},
    Cipher,
};
use crypto_bigint::{U1024, U256, U512};
use egui::Ui;
use rand::{thread_rng, Rng};

macro_rules! interface {
    ($ui: ident, $cipher: expr, $key: expr, $tweak: expr, $bits: literal, $words: literal) => {
        $ui.byte_io_mode_cipher(&mut $cipher.input_format, &mut $cipher.output_format);

        $ui.add_space(16.0);

        block_cipher_mode_and_padding($ui, &mut $cipher.mode, &mut $cipher.padding);
        $ui.add_space(8.0);

        $ui.horizontal(|ui| {
            ui.subheading("Key");
            if ui.random_bytes_button(&mut $key).clicked() {
                $cipher.ksa_u64(&$key, &$tweak);
            }
        });
        $ui.label(format!("Threefish-{0} uses a {0}-bit key.", $bits));
        for i in 0..8 {
            if $ui.u64_hex_edit(&mut $key[i]).lost_focus() {
                $cipher.ksa_u64(&$key, &$tweak);
            }
        }

        $ui.add_space(8.0);

        if $cipher.mode.iv_needed() {
            $ui.horizontal(|ui| {
                ui.label(format!(
                    "In this mode a {}-bit initialization vector is needed.",
                    $bits
                ));
                ui.random_bytes_button($cipher.iv.as_words_mut())
            });
            for i in $cipher.iv.as_words_mut() {
                #[cfg(target_pointer_width = "64")]
                $ui.u64_hex_edit(i);
                #[cfg(target_pointer_width = "32")]
                $ui.u32_hex_edit(i);
            }
        }
        $ui.add_space(16.0);
    };
}

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

impl ThreefishFrame {
    fn set_cipher(&mut self) {
        match self.selector {
            ThreefishSelect::Threefish256 => self.cipher256.ksa_u64(&self.key256, &self.tweak),
            ThreefishSelect::Threefish512 => self.cipher512.ksa_u64(&self.key512, &self.tweak),
            ThreefishSelect::Threefish1024 => self.cipher1024.ksa_u64(&self.key1024, &self.tweak),
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
            "Threefish-256",
        );
        ui.selectable_value(
            &mut self.selector,
            ThreefishSelect::Threefish512,
            "Threefish-512",
        );
        ui.selectable_value(
            &mut self.selector,
            ThreefishSelect::Threefish1024,
            "Threefish-1024",
        );
        ui.add_space(8.0);

        ui.randomize_reset_cipher(self);
        ui.add_space(16.0);

        ui.horizontal(|ui| {
            ui.subheading("Tweak");
            if ui.random_bytes_button(&mut self.tweak).clicked() {
                self.set_cipher();
            }
        });
        ui.label("All versions of Threefish use a 128-bit \"tweak\" value to adjust the key schedule, presented here as two 64-bit words.");
        for i in 0..2 {
            if ui.u64_hex_edit(&mut self.tweak[i]).lost_focus() {
                self.set_cipher();
            }
        }

        ui.add_space(16.0);

        match self.selector {
            ThreefishSelect::Threefish256 => {
                interface!(ui, self.cipher256, self.key256, self.tweak, "256", 4);
            }
            ThreefishSelect::Threefish512 => {
                interface!(ui, self.cipher512, self.key512, self.tweak, "512", 8);
            }
            ThreefishSelect::Threefish1024 => {
                interface!(ui, self.cipher1024, self.key1024, self.tweak, "1024", 16);
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
        for t in self.tweak.iter_mut() {
            *t = rng.gen()
        }
        match self.selector {
            ThreefishSelect::Threefish256 => {
                for k in self.key256.iter_mut() {
                    *k = rng.gen()
                }
                if self.cipher256.mode.iv_needed() {
                    self.cipher256.iv = U256::from_words(rng.gen());
                }
            }
            ThreefishSelect::Threefish512 => {
                for k in self.key512.iter_mut() {
                    *k = rng.gen()
                }
                if self.cipher512.mode.iv_needed() {
                    self.cipher512.iv = U512::from_words(rng.gen());
                }
            }
            ThreefishSelect::Threefish1024 => {
                for k in self.key1024.iter_mut() {
                    *k = rng.gen()
                }
                if self.cipher1024.mode.iv_needed() {
                    self.cipher1024.iv = U1024::from_words(rng.gen());
                }
            }
        }
        self.set_cipher();
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
