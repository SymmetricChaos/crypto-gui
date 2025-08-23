use super::CipherFrame;
use crate::ui_elements::UiElements;
use ciphers::digital::stream_ciphers::rabbit::Rabbit;
use rand::{thread_rng, Rng};
use utils::byte_formatting::ByteFormat;

pub struct RabbitFrame {
    input_format: ByteFormat,
    output_format: ByteFormat,
    cipher: Rabbit,
    key: [u32; 4],
    iv: [u32; 2],
    use_iv: bool,
}

impl Default for RabbitFrame {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            cipher: Default::default(),
            key: [0; 4],
            iv: [0; 2],
            use_iv: false,
        }
    }
}

impl RabbitFrame {
    fn set_rng(&mut self) {
        if self.use_iv {
            self.cipher = Rabbit::with_key_and_iv_u32(self.key, self.iv)
        } else {
            self.cipher = Rabbit::with_key_u32(self.key)
        }
    }
}

impl CipherFrame for RabbitFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/ciphers/src/digital/stream_ciphers/rabbit.rs",
        );

        ui.byte_io_mode_cipher(&mut self.input_format, &mut self.output_format);

        ui.add_space(8.0);
        ui.subheading("Key");
        for i in 0..4 {
            if ui.u32_hex_edit(&mut self.key[i]).lost_focus() {
                self.set_rng();
            }
        }

        ui.add_space(8.0);
        ui.checkbox(&mut self.use_iv, "Use IV");
        ui.add_space(2.0);
        if self.use_iv {
            ui.subheading("Initialization Vector");
            for i in 0..2 {
                if ui.u32_hex_edit(&mut self.iv[i]).lost_focus() {
                    self.set_rng();
                }
            }
        }
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        rng.fill(&mut self.key);
    }

    fn reset(&mut self) {
        *self = Self::default()
    }

    fn encrypt_string(&self, text: &str) -> Result<String, utils::errors::GeneralError> {
        let mut bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|e| utils::errors::GeneralError::input(e.to_string()))?;
        self.cipher.encrypt_bytes(&mut bytes);
        Ok(self.output_format.byte_slice_to_text(&bytes))
    }

    fn decrypt_string(&self, text: &str) -> Result<String, utils::errors::GeneralError> {
        self.encrypt_string(text)
    }
}
