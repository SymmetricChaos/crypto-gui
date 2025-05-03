use super::HasherFrame;
use crate::ui_elements::UiElements;
use hashers::{sm3::Sm3, traits::StatefulHasher};
use utils::byte_formatting::ByteFormat;

pub struct Sm3Frame {
    input_format: ByteFormat,
    output_format: ByteFormat,
}

impl Default for Sm3Frame {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
        }
    }
}

impl HasherFrame for Sm3Frame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/hashers/src/sm3.rs",
        );

        ui.byte_io_mode_hasher(&mut self.input_format, &mut self.output_format);

        ui.add_space(16.0);
    }

    fn hash_string(&self, text: &str) -> Result<String, hashers::errors::HasherError> {
        let bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| hashers::errors::HasherError::general("byte format error"))?;
        Ok(self
            .output_format
            .byte_slice_to_text(Sm3::init().hash(&bytes)))
    }
}
