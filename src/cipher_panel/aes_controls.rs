use super::CipherFrame;
use crate::ui_elements::{block_cipher_mode, UiElements};

use ciphers::{
    digital::block_ciphers::{aes::aes::Aes128, BlockCipherMode},
    Cipher,
};
use egui::{DragValue, Ui};
use rand::{thread_rng, Rng};

pub struct AesFrame {
    cipher: Aes128,
    ctr_upper: u64,
    ctr_lower: u64,
}

impl Default for AesFrame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
            ctr_upper: 0,
            ctr_lower: 0,
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
        ui.add_space(8.0);

        ui.subheading("Key");
        ui.label("AES128 uses four 32-bit keys or, equivalently, a single 128-bit key.");
        ui.u32_drag_value(&mut self.cipher.key[0]);
        ui.u32_drag_value(&mut self.cipher.key[1]);
        ui.u32_drag_value(&mut self.cipher.key[2]);
        ui.u32_drag_value(&mut self.cipher.key[3]);

        ui.add_space(8.0);

        ui.add_enabled_ui(self.cipher.mode == BlockCipherMode::Ctr, |ui| {
            ui.subheading("Counter");
            ui.label("In CTR mode the cipher must have a 128-bit counter value provided. The selectors below control the upper and lower 64-bits respectively.");
            if ui.add(DragValue::new(&mut self.ctr_upper).hexadecimal(16, false, true)) .changed() {
                self.cipher.ctr &= 0x0000000000000000FFFFFFFFFFFFFFFF;
                self.cipher.ctr |= (self.ctr_upper as u128) << 64;
            }
            if ui.add(DragValue::new(&mut self.ctr_lower).hexadecimal(16, false, true)) .changed() {
                self.cipher.ctr &= 0xFFFFFFFFFFFFFFFF0000000000000000;
                self.cipher.ctr |= self.ctr_lower as u128;
            }
            ui.label(format!("{:032x?}",self.cipher.ctr))
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

        if self.cipher.mode == BlockCipherMode::Ctr {
            self.cipher.ctr = rng.gen();
        }
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
