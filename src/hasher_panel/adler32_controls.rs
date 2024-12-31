use hashers::{adler::Adler32, errors::HasherError, traits::StatefulHasher};
use utils::byte_formatting::ByteFormat;

use crate::ui_elements::UiElements;

use super::HasherFrame;

pub struct Adler32Frame {
    input_format: ByteFormat,
    output_format: ByteFormat,
}

impl Default for Adler32Frame {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
        }
    }
}

impl Adler32Frame {}

impl HasherFrame for Adler32Frame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/hashers/src/adler32.rs",
        );
        ui.add_space(8.0);

        ui.byte_io_mode_hasher(&mut self.input_format, &mut self.output_format);

        ui.add_space(8.0);

        ui.label("<<<EXPLANATION OF HASH FUNCTION CODE>>>");

        ui.add_space(16.0);
    }

    fn hash_string(&self, text: &str) -> Result<String, HasherError> {
        let bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| hashers::errors::HasherError::general("byte format error"))?;

        let h = Adler32::init().hash(&bytes);

        Ok(self.output_format.byte_slice_to_text(&h))
    }
}
