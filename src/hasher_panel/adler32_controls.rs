use super::HasherFrame;
use crate::ui_elements::UiElements;
use hashers::{adler::Adler32, errors::HasherError, traits::StatefulHasher};
use utils::byte_formatting::ByteFormat;

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

        ui.subheading("Example");
        ui.label("Input: \"Adler\"");
        ui.add_space(2.0);
        ui.monospace("inital         b: 0    a: 1     state 00000001");
        ui.monospace("\"A\" =  65    b: 66   a: 66    state 00420042");
        ui.monospace("\"d\" = 100    b: 232  a: 166   state 00e800a6");
        ui.monospace("\"l\" = 108    b: 506  a: 274   state 01fa0112");
        ui.monospace("\"e\" = 101    b: 881  a: 375   state 03710177");
        ui.monospace("\"r\" = 114    b: 1370 a: 489   state 055a01e9");
        ui.add_space(2.0);
        ui.monospace("Output: 055a01e9");
        ui.add_space(4.0);
        ui.label(
            "Notice that because the input is very short the upper bits of both words are zero.",
        );

        ui.add_space(16.0);
    }

    fn hash_string(&self, text: &str) -> Result<String, HasherError> {
        let bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| hashers::errors::HasherError::general("byte format error"))?;

        let h = Adler32::init().update_and_finalize(&bytes);

        Ok(self.output_format.byte_slice_to_text(&h))
    }
}
