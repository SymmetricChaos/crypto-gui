use super::CipherFrame;
use crate::ui_elements::{block_cipher_mode, block_cipher_padding, UiElements};

use ciphers::{
    digital::block_ciphers::{aes::aes::Aes128, block_cipher::BCMode},
    Cipher,
};
use egui::{DragValue, Ui};
use rand::{thread_rng, Rng};

pub struct AesFrame {
    cipher: Aes128,
    ctr_upper: u64,
    ctr_lower: u64,
    iv_upper: u64,
    iv_lower: u64,
}

impl Default for AesFrame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
            ctr_upper: 0,
            ctr_lower: 0,
            iv_upper: 0,
            iv_lower: 0,
        }
    }
}

impl AesFrame {}

impl CipherFrame for AesFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/tree/master/ciphers/src/digital/block_ciphers/aes",
        );
        ui.add_space(8.0);

        ui.randomize_reset(self);
        ui.add_space(16.0);

        ui.byte_io_mode(
            &mut self.cipher.input_format,
            &mut self.cipher.output_format,
        );

        ui.add_space(16.0);

        block_cipher_mode(ui, &mut self.cipher.mode);
        ui.add_space(4.0);
        block_cipher_padding(ui, &mut self.cipher.padding);
        ui.add_space(8.0);

        ui.subheading("Key");
        ui.label("AES128 uses four 32-bit keys or, equivalently, a single 128-bit key.");
        ui.u32_drag_value_hex(&mut self.cipher.key[0]);
        ui.u32_drag_value_hex(&mut self.cipher.key[1]);
        ui.u32_drag_value_hex(&mut self.cipher.key[2]);
        ui.u32_drag_value_hex(&mut self.cipher.key[3]);

        ui.add_space(8.0);

        ui.add_enabled_ui(self.cipher.mode == BCMode::Ctr, |ui| {
            ui.subheading("Counter");
            ui.label("In CTR mode the cipher must have a 128-bit counter value provided. The selectors below control the upper and lower 64-bits respectively.");
            if ui.add(DragValue::new(&mut self.ctr_upper).hexadecimal(16, false, false)) .changed() {
                self.cipher.ctr &= 0x0000000000000000FFFFFFFFFFFFFFFF;
                self.cipher.ctr |= (self.ctr_upper as u128) << 64;
            }
            if ui.add(DragValue::new(&mut self.ctr_lower).hexadecimal(16, false, false)) .changed() {
                self.cipher.ctr &= 0xFFFFFFFFFFFFFFFF0000000000000000;
                self.cipher.ctr |= self.ctr_lower as u128;
            }
            ui.label(format!("{:032x?}",self.cipher.ctr))
        });

        ui.add_enabled_ui(self.cipher.mode == BCMode::Cbc, |ui| {
            ui.subheading("Initialization Vector");
            ui.label("In CBC mode the cipher must have a 128-bit initialization vector provided. The selectors below control the upper and lower 64-bits respectively.");
            if ui.add(DragValue::new(&mut self.iv_upper).hexadecimal(16, false, false)) .changed() {
                self.cipher.cbc &= 0x0000000000000000FFFFFFFFFFFFFFFF;
                self.cipher.cbc |= (self.iv_upper as u128) << 64;
            }
            if ui.add(DragValue::new(&mut self.iv_lower).hexadecimal(16, false, false)) .changed() {
                self.cipher.cbc &= 0xFFFFFFFFFFFFFFFF0000000000000000;
                self.cipher.cbc |= self.iv_lower as u128;
            }
            ui.label(format!("{:032x?}",self.cipher.cbc))
        });

        ui.add_space(16.0);
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        self.cipher.key[0] = rng.gen();
        self.cipher.key[1] = rng.gen();
        self.cipher.key[2] = rng.gen();
        self.cipher.key[3] = rng.gen();

        if self.cipher.mode == BCMode::Ctr {
            self.cipher.ctr = rng.gen();
        }
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
