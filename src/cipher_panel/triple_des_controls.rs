use super::CipherFrame;
use crate::ui_elements::{block_cipher_iv_64, block_cipher_mode_and_padding, UiElements};
use ciphers::digital::block_ciphers::des::{
    des_functions::set_des_key_parity, triple_des::TripleDes,
};
use egui::Ui;
use rand::{thread_rng, Rng};

pub struct TripleDesFrame {
    cipher: TripleDes,
    keys: [u64; 3],
    ksa_error: String,
}

impl Default for TripleDesFrame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
            keys: [0x0101010101010101, 0x0202020202020202, 0x0303030303030303],
            ksa_error: String::new(),
        }
    }
}

impl TripleDesFrame {}

impl CipherFrame for TripleDesFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/tree/master/ciphers/src/digital/block_ciphers/des",
        );
        ui.add_space(8.0);

        ui.randomize_reset_cipher(self);
        ui.add_space(16.0);

        ui.byte_io_mode_cipher(
            &mut self.cipher.input_format,
            &mut self.cipher.output_format,
        );

        ui.add_space(16.0);

        block_cipher_mode_and_padding(ui, &mut self.cipher.mode, &mut self.cipher.padding);
        ui.add_space(8.0);

        ui.subheading("Key");
        ui.label("Triple-DES uses three 64-bit keys but the eighth bit of each byte is used for parity, reducing the actual key size. For simplicity the parity bits are ignored in this implementation. The second key must not be the same as either the first or third key. If the first and third key are the same this is sometimes referred to as Double-DES.");
        if ui.small_button("set parity").clicked() {
            for key in self.keys.iter_mut() {
                *key = set_des_key_parity(*key)
            }
        }

        for i in 0..3 {
            if ui.u64_hex_edit(&mut self.keys[i]).changed() {
                match self.cipher.ksa(self.keys) {
                    Ok(_) => self.ksa_error.clear(),
                    Err(e) => self.ksa_error = e.to_string(),
                }
            }
        }
        ui.error_text(&self.ksa_error);

        ui.add_space(8.0);

        block_cipher_iv_64(ui, &mut self.cipher.iv, self.cipher.mode);

        ui.add_space(16.0);
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        for key in self.keys.iter_mut() {
            *key = rng.gen();
        }

        match self.cipher.ksa(self.keys) {
            Ok(_) => self.ksa_error.clear(),
            Err(e) => self.ksa_error = e.to_string(),
        }

        if self.cipher.mode.iv_needed() {
            self.cipher.iv = rng.gen();
        }
    }

    crate::simple_cipher! {}
}
