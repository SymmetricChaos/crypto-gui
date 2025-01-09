use crate::ui_elements::UiElements;

use super::HasherFrame;
use hashers::{
    murmurhash3::{Murmur3_128, Murmur3_32},
    traits::StatefulHasher,
};
use utils::byte_formatting::ByteFormat;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Murmur3Selector {
    M32,
    M128,
}

pub struct Murmur3Frame {
    input_format: ByteFormat,
    output_format: ByteFormat,
    selector: Murmur3Selector,
    seed: u32,
}

impl Default for Murmur3Frame {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            selector: Murmur3Selector::M32,
            seed: 0,
        }
    }
}

impl Murmur3Frame {}

impl HasherFrame for Murmur3Frame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/hashers/src/murmurhash3.rs",
        );

        ui.add_space(8.0);
        ui.byte_io_mode_hasher(&mut self.input_format, &mut self.output_format);

        ui.add_space(8.0);
        ui.selectable_value(&mut self.selector, Murmur3Selector::M32, "Murmur3_32");
        ui.selectable_value(&mut self.selector, Murmur3Selector::M128, "Murmur3_128");

        ui.add_space(8.0);
        ui.subheading("Seed");
        ui.u32_hex_edit(&mut self.seed);

        ui.add_space(16.0);

        ui.label("<<<EXPLANATION OF HASH FUNCTION CODE>>>");

        ui.add_space(16.0);
    }

    fn hash_string(&self, text: &str) -> Result<String, hashers::errors::HasherError> {
        let bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| hashers::errors::HasherError::general("byte format error"))?;

        let h = match self.selector {
            Murmur3Selector::M32 => Murmur3_32::init(&self.seed.to_be_bytes()).update_and_finalize(&bytes),
            Murmur3Selector::M128 => Murmur3_128::init(&self.seed.to_be_bytes()).update_and_finalize(&bytes),
        };

        Ok(self.output_format.byte_slice_to_text(&h))
    }
}
