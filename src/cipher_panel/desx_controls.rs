use super::CipherFrame;
use crate::ui_elements::{block_cipher_mode, block_cipher_padding, UiElements};
use ciphers::{
    digital::block_ciphers::des::{des_functions::set_des_key_parity, desx::DesX},
    Cipher,
};
use egui::Ui;
use rand::{thread_rng, Rng};

pub struct DesXFrame {
    cipher: DesX,
    key: u64,
    ksa_error: String,
}

impl Default for DesXFrame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
            key: 0x0101010101010101,
            ksa_error: String::new(),
        }
    }
}

impl DesXFrame {}

impl CipherFrame for DesXFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/tree/master/ciphers/src/digital/block_ciphers/des",
        );
        ui.add_space(8.0);

        ui.randomize_reset(self);
        ui.add_space(16.0);

        ui.byte_io_mode_cipher(
            &mut self.cipher.input_format,
            &mut self.cipher.output_format,
        );

        ui.add_space(16.0);

        block_cipher_mode(ui, &mut self.cipher.mode);
        ui.add_space(4.0);
        block_cipher_padding(ui, &mut self.cipher.padding);
        ui.add_space(8.0);

        ui.subheading("Key");
        ui.label("DES uses a 64-bit key but the eighth bit of each byte is used for parity, reducing the actual key size to 56-bits.\nFor simplicity the parity bits are ignored for this implementation rather than causing an error if they are incorrect.");
        if ui.small_button("set parity").clicked() {
            self.key = set_des_key_parity(self.key)
        }
        if ui.u64_drag_value_hex(&mut self.key).changed() {
            match self.cipher.ksa(self.key) {
                Ok(_) => self.ksa_error.clear(),
                Err(e) => self.ksa_error = e.to_string(),
            }
        }
        ui.error_text(&self.ksa_error);
        ui.add_space(8.0);

        ui.subheading("Extra Keys");
        ui.label("DES-X uses two additional 64-bit keys.");
        ui.u64_drag_value_hex(&mut self.cipher.extra_keys[0]);
        ui.u64_drag_value_hex(&mut self.cipher.extra_keys[1]);

        ui.add_space(8.0);

        ui.add_enabled_ui(self.cipher.mode.iv_needed(), |ui| {
            ui.subheading("IV/Counter");
            ui.label("In the selected mode the cipher must have a 64-bit initial value provided.");
            ui.u64_drag_value_hex(&mut self.cipher.iv);
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

        if self.cipher.mode.iv_needed() {
            self.cipher.iv = rng.gen();
        }
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
