use super::CipherFrame;
use crate::ui_elements::{block_cipher_mode, UiElements};
use ciphers::digital::aes::aes::Aes128;
use ciphers::{digital::BlockCipherMode, Cipher};
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
        ui.add(DragValue::new(&mut self.cipher.key[0]).hexadecimal(8, false, true));
        ui.add(DragValue::new(&mut self.cipher.key[1]).hexadecimal(8, false, true));
        ui.add(DragValue::new(&mut self.cipher.key[2]).hexadecimal(8, false, true));
        ui.add(DragValue::new(&mut self.cipher.key[3]).hexadecimal(8, false, true));

        ui.add_space(8.0);

        ui.add_enabled_ui(self.cipher.mode == BlockCipherMode::Ctr, |ui| {
            ui.subheading("Counter");
            ui.label("In CTR mode the cipher must have a 128-bit counter value provided. The selectors below control the upper and lower 64-bit respectively.");
            if ui.add(DragValue::new(&mut self.ctr_upper).hexadecimal(16, false, true)) .changed() {
                self.cipher.ctr &= 0x00000000000000001111111111111111;
                self.cipher.ctr |= (self.ctr_upper as u128) << 64;
            }
            if ui.add(DragValue::new(&mut self.ctr_lower).hexadecimal(16, false, true)) .changed() {
                self.cipher.ctr &= 0x11111111111111110000000000000000;
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
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}