use super::CipherFrame;
use crate::ui_elements::{block_cipher_mode, u64_drag_value, UiElements};
use ciphers::{
    digital::block_ciphers::{
        des::{des::Des, des_functions::set_des_key_parity},
        BlockCipherMode,
    },
    Cipher,
};
use egui::Ui;
use rand::{thread_rng, Rng};

pub struct DesFrame {
    cipher: Des,
    key: u64,
    ksa_error: String,
}

impl Default for DesFrame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
            key: 0x0101010101010101,
            ksa_error: String::new(),
        }
    }
}

impl DesFrame {}

impl CipherFrame for DesFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/tree/master/ciphers/src/digital/block_ciphers/des",
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
        ui.label("DES uses a 64-bit key but the eighth bit of each byte is used for parity, reducing the actual key size to 56-bits.\nFor simplicity the parity bits are ignored for this implementation rather than causing an error if they are incorrect.");
        if ui.small_button("set parity").clicked() {
            self.key = set_des_key_parity(self.key)
        }
        if u64_drag_value(ui, &mut self.key).changed() {
            match self.cipher.ksa(self.key) {
                Ok(_) => self.ksa_error.clear(),
                Err(e) => self.ksa_error = e.to_string(),
            }
        }
        ui.error_text(&self.ksa_error);

        ui.add_space(8.0);

        ui.add_enabled_ui(self.cipher.mode == BlockCipherMode::Ctr, |ui| {
            ui.subheading("Counter");
            ui.label("In CTR mode the cipher must have a 64-bit counter value provided.");
            ui.u64_drag_value(&mut self.cipher.ctr);
        });

        ui.add_space(16.0);
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        self.key = rng.gen();
        match self.cipher.ksa(self.key) {
            Ok(_) => self.ksa_error.clear(),
            Err(e) => self.ksa_error = e.to_string(),
        }

        if self.cipher.mode == BlockCipherMode::Ctr {
            self.cipher.ctr = rng.gen();
        }
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
