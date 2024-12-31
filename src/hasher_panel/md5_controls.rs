use crate::ui_elements::UiElements;

use super::HasherFrame;
use hashers::{md5::Md5, traits::StatefulHasher};
use utils::byte_formatting::ByteFormat;

pub struct Md5Frame {
    input_format: ByteFormat,
    output_format: ByteFormat,
}

impl Default for Md5Frame {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
        }
    }
}

impl Md5Frame {}

impl HasherFrame for Md5Frame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/hashers/src/md5.rs",
        );
        ui.add_space(8.0);

        ui.byte_io_mode_hasher(&mut self.input_format, &mut self.output_format);

        ui.add_space(8.0);

        ui.label("<<<EXPLANATION OF HASH FUNCTION CODE>>>");

        ui.add_space(16.0);
    }

    fn hash_string(&self, text: &str) -> Result<String, hashers::errors::HasherError> {
        let bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| hashers::errors::HasherError::general("byte format error"))?;
        Ok(self
            .output_format
            .byte_slice_to_text(Md5::init().hash(&bytes)))
    }
}
