use super::CipherFrame;
use crate::ui_elements::UiElements;
use ciphers::digital::stream_ciphers::a51::A51;
use rand::{thread_rng, Rng};
use utils::byte_formatting::ByteFormat;

pub struct A51Frame {
    input_format: ByteFormat,
    output_format: ByteFormat,
    cipher: A51,
    key: u64,
    frame_number: u32,
}

impl Default for A51Frame {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            cipher: Default::default(),
            key: 0,
            frame_number: 0,
        }
    }
}

impl CipherFrame for A51Frame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/ciphers/src/digital/stream_ciphers/a51.rs",
        );
        ui.add_space(8.0);

        ui.randomize_reset_cipher(self);
        ui.add_space(16.0);

        ui.byte_io_mode_cipher(&mut self.input_format, &mut self.output_format);
        ui.add_space(16.0);

        ui.subheading("LFSRs (Starting States)");
        ui.monospace(format!("{:019b}", self.cipher.rng.lfsrs[0].register));
        ui.monospace("^^^  ^             ");
        ui.add_space(4.0);
        ui.monospace(format!("{:022b}", self.cipher.rng.lfsrs[1].register));
        ui.monospace("^^                    ");
        ui.add_space(4.0);
        ui.monospace(format!("{:023b}", self.cipher.rng.lfsrs[2].register));
        ui.monospace("^^^            ^       ");
        ui.add_space(16.0);

        ui.subheading("Key (Taken in Big-endian Order)");
        if ui.u64_hex_edit(&mut self.key).lost_focus() {
            self.cipher
                .rng
                .ksa(self.key.to_be_bytes(), self.frame_number)
        }
        ui.add_space(8.0);

        ui.subheading("Frame Number (Limited to 22 Bits)");
        if ui.u32_hex_edit(&mut self.frame_number).lost_focus() {
            self.frame_number &= 0x3fffff; // mask off the high bits
            self.cipher
                .rng
                .ksa(self.key.to_be_bytes(), self.frame_number)
        }
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        self.key = rng.gen();
        self.frame_number = rng.gen();
        self.frame_number &= 0x3fffff; // mask off the high bits
        self.cipher
            .rng
            .ksa(self.key.to_be_bytes(), self.frame_number)
    }

    fn reset(&mut self) {
        *self = Self::default()
    }

    fn encrypt_string(&self, text: &str) -> Result<String, ciphers::CipherError> {
        let mut bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| ciphers::CipherError::general("byte format error"))?;

        self.cipher.encrypt_bytes(&mut bytes);

        Ok(self.output_format.byte_slice_to_text(&bytes))
    }

    fn decrypt_string(&self, text: &str) -> Result<String, ciphers::CipherError> {
        let mut bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| ciphers::CipherError::general("byte format error"))?;

        self.cipher.decrypt_bytes(&mut bytes);

        Ok(self.output_format.byte_slice_to_text(&bytes))
    }
}
