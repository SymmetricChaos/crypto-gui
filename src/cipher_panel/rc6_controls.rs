use crate::{
    cipher_panel::CipherFrame,
    ui_elements::{block_cipher_iv_128, block_cipher_mode_and_padding, UiElements},
};
use ciphers::{digital::block_ciphers::rc6::Rc6, Cipher};
use rand::{thread_rng, Rng};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum KeySelect {
    K128,
    K192,
    K256,
}

pub struct Rc6Frame {
    cipher: Rc6,
    key_size: KeySelect,
    key_128: [u32; 4],
    key_192: [u32; 6],
    key_256: [u32; 8],
}

impl Default for Rc6Frame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
            key_size: KeySelect::K128,
            key_128: Default::default(),
            key_192: Default::default(),
            key_256: Default::default(),
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

        match self.key_size {
            KeySelect::K128 => {
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    ui.subheading("Key");
                    if ui.random_bytes_button(&mut self.key_128).clicked() {
                        self.cipher.ksa_128_u32(&self.key_128);
                    }
                });
                for i in 0..4 {
                    if ui.u32_hex_edit(&mut self.key_128[i]).lost_focus() {
                        self.cipher.ksa_128_u32(&self.key_128);
                    }
                }
            }
            KeySelect::K192 => {
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    ui.subheading("Key");
                    if ui.random_bytes_button(&mut self.key_192).clicked() {
                        self.cipher.ksa_192_u32(&self.key_192);
                    }
                });
                for i in 0..6 {
                    if ui.u32_hex_edit(&mut self.key_192[i]).lost_focus() {
                        self.cipher.ksa_192_u32(&self.key_192);
                    }
                }
            }
            KeySelect::K256 => {
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    ui.subheading("Key");
                    if ui.random_bytes_button(&mut self.key_256).clicked() {
                        self.cipher.ksa_256_u32(&self.key_256);
                    }
                });
                for i in 0..8 {
                    if ui.u32_hex_edit(&mut self.key_256[i]).lost_focus() {
                        self.cipher.ksa_256_u32(&self.key_256);
                    }
                }
            }
        }

        block_cipher_mode_and_padding(ui, &mut self.cipher.mode, &mut self.cipher.padding);
        ui.add_space(8.0);

        block_cipher_iv_128(ui, &mut self.cipher.iv, self.cipher.mode);
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        match self.key_size {
            KeySelect::K128 => {
                rng.fill(&mut self.key_128);
                self.cipher.ksa_128_u32(&self.key_128)
            }
            KeySelect::K192 => {
                rng.fill(&mut self.key_192);
                self.cipher.ksa_192_u32(&self.key_192)
            }
            KeySelect::K256 => {
                rng.fill(&mut self.key_256);
                self.cipher.ksa_256_u32(&self.key_256)
            }
        }
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
