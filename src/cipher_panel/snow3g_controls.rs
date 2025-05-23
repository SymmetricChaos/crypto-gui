use super::CipherFrame;
use crate::ui_elements::UiElements;
use ciphers::digital::stream_ciphers::snow::snow3g::Snow3G;
use rand::{thread_rng, Rng};
use utils::byte_formatting::ByteFormat;

pub struct Snow3GFrame {
    input_format: ByteFormat,
    output_format: ByteFormat,
    cipher: Snow3G,
    key: [u32; 4],
    iv: [u32; 4],
}

impl Default for Snow3GFrame {
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

impl Snow3GFrame {}

impl CipherFrame for Snow3GFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/ciphers/src/digital/stream_ciphers/snow",
        );
        ui.add_space(8.0);

        ui.randomize_reset_cipher(self);
        ui.add_space(16.0);

        ui.byte_io_mode_cipher(&mut self.input_format, &mut self.output_format);

        ui.add_space(8.0);
        ui.horizontal(|ui| {
            ui.subheading("Key (128 bits)");
            ui.random_bytes_button(&mut self.key);
        });
        ui.horizontal(|ui| {
            for i in 0..8 {
                if ui.u32_hex_edit(&mut self.key[i]).lost_focus() {
                    self.cipher = Snow3G::with_key_and_iv(self.key, self.iv);
                }
            }
        });

        ui.add_space(8.0);
        ui.horizontal(|ui| {
            ui.subheading("IV (128 bits)");
            ui.random_bytes_button(&mut self.iv);
        });
        ui.horizontal(|ui| {
            for i in 0..8 {
                if ui.u32_hex_edit(&mut self.iv[i]).lost_focus() {
                    self.cipher = Snow3G::with_key_and_iv(self.key, self.iv);
                }
            }
        });
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        rng.fill(&mut self.key);
        rng.fill(&mut self.iv);
        self.cipher = Snow3G::with_key_and_iv(self.key, self.iv);
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
