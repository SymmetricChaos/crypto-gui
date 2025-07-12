use crate::{
    cipher_panel::CipherFrame,
    ui_elements::{block_cipher_iv_128, block_cipher_mode_and_padding, UiElements},
};
use ciphers::{digital::block_ciphers::rc6::Rc6, Cipher};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum KeySelect {
    K128,
    K192,
    K256,
}

pub struct Rc6Frame {
    cipher: Rc6,
    key_size: KeySelect,
    key: String,
}

impl Default for Rc6Frame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
            key_size: KeySelect::K128,
            key: Default::default(),
        }
    }
}

impl Rc6Frame {}

impl CipherFrame for Rc6Frame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/ciphers/src/digital/block_ciphers/rc6.rs",
        );
        ui.add_space(8.0);

        ui.randomize_reset_cipher(self);
        ui.add_space(16.0);

        ui.byte_io_mode_cipher(
            &mut self.cipher.input_format,
            &mut self.cipher.output_format,
        );
        ui.add_space(16.0);

        ui.selectable_value(&mut self.key_size, KeySelect::K128, "128-bit Key");
        ui.selectable_value(&mut self.key_size, KeySelect::K192, "192-bit Key");
        ui.selectable_value(&mut self.key_size, KeySelect::K256, "256-bit Key");

        ui.subheading("Key");
        ui.label("Key should be provided as a string of hexadecimal digits representing between 1 and 255 bytes.");
        if ui.text_edit_multiline(&mut self.key).changed() {
            self.key = self.key.chars().filter(|c| c.is_ascii_hexdigit()).collect();
        }
        if ui.button("Set State from Key").clicked() {
            if self.key.len() > 510 {
                self.key.truncate(510)
            }
            if self.key.len() % 2 == 1 {
                self.key.push('0')
            }
            self.run_ksa()
        }
        ui.add_space(16.0);

        block_cipher_mode_and_padding(ui, &mut self.cipher.mode, &mut self.cipher.padding);
        ui.add_space(8.0);

        block_cipher_iv_128(ui, &mut self.cipher.iv, self.cipher.mode);
        todo!()
    }

    fn randomize(&mut self) {
        todo!()
    }

    fn reset(&mut self) {
        *self = Self::default()
    }

    fn encrypt_string(&self, text: &str) -> Result<String, ciphers::CipherError> {
        self.cipher.encrypt(text)
    }

    fn decrypt_string(&self, text: &str) -> Result<String, ciphers::CipherError> {
        self.cipher.decrypt(text)
    }
}
