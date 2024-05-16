use super::CipherFrame;
use crate::ui_elements::{block_cipher_mode, UiElements};
use ciphers::{
    digital::{tea::Tea, BlockCipherMode},
    Cipher,
};
use egui::{DragValue, Ui};
use rand::{thread_rng, Rng};

pub struct TeaFrame {
    cipher: Tea,
}

impl Default for TeaFrame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
        }
    }
}

impl TeaFrame {}

impl CipherFrame for TeaFrame {
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
        ui.label("TEA uses four 32-bit keys or, equivalently, a single 128-bit key.");
        ui.add(DragValue::new(&mut self.cipher.key[0]).hexadecimal(8, false, true));
        ui.add(DragValue::new(&mut self.cipher.key[1]).hexadecimal(8, false, true));
        ui.add(DragValue::new(&mut self.cipher.key[2]).hexadecimal(8, false, true));
        ui.add(DragValue::new(&mut self.cipher.key[3]).hexadecimal(8, false, true));

        ui.add_space(8.0);

        ui.add_enabled_ui(self.cipher.mode == BlockCipherMode::Ctr, |ui| {
            ui.subheading("Counter");
            ui.label("In CTR mode the cipher must have a 64-bit counter value provided.");
            ui.add(DragValue::new(&mut self.cipher.ctr).hexadecimal(16, false, true));
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
