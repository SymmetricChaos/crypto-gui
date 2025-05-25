use super::CipherFrame;
use crate::ui_elements::UiElements;
use ciphers::digital::stream_ciphers::hc128::Hc128;
use rand::{thread_rng, Rng};
use utils::byte_formatting::ByteFormat;

pub struct Hc128Frame {
    input_format: ByteFormat,
    output_format: ByteFormat,
    cipher: Hc128,
    key: [u32; 4],
    iv: [u32; 4],
}

impl Default for Hc128Frame {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            cipher: Default::default(),
            key: [0; 4],
            iv: [0; 4],
        }
    }
}

impl CipherFrame for Hc128Frame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/ciphers/src/digital/stream_ciphers/hc128.rs",
        );

        ui.add_space(8.0);
        ui.subheading("Key");
        for i in 0..4 {
            if ui.u32_hex_edit(&mut self.key[i]).lost_focus() {
                self.cipher = Hc128::with_key_and_iv_u32(self.key, self.iv);
            }
        }

        ui.add_space(8.0);
        ui.subheading("Nonce");
        for i in 0..4 {
            if ui.u32_hex_edit(&mut self.iv[i]).lost_focus() {
                self.cipher = Hc128::with_key_and_iv_u32(self.key, self.iv);
            }
        }
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        rng.fill(&mut self.key);
        rng.fill(&mut self.iv);
    }

    fn reset(&mut self) {
        *self = Self::default()
    }

    fn encrypt_string(&self, text: &str) -> Result<String, ciphers::CipherError> {
        let mut bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|e| ciphers::CipherError::Input(e.to_string()))?;
        self.cipher.encrypt_bytes(&mut bytes);
        Ok(self.output_format.byte_slice_to_text(&bytes))
    }

    fn decrypt_string(&self, text: &str) -> Result<String, ciphers::CipherError> {
        self.encrypt_string(text)
    }
}
