use super::HasherFrame;
use crate::ui_elements::UiElements;
use hashers::{
    fletcher::{Fletcher, FletcherhWidth},
    traits::StatefulHasher,
};
use utils::byte_formatting::ByteFormat;

pub struct FletcherFrame {
    input_format: ByteFormat,
    output_format: ByteFormat,
    width: FletcherhWidth,
}

impl Default for FletcherFrame {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            width: FletcherhWidth::W64,
        }
    }
}

impl FletcherFrame {}

impl HasherFrame for FletcherFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/hashers/src/fletcher.rs",
        );
        ui.add_space(16.0);
        ui.byte_io_mode_hasher(&mut self.input_format, &mut self.output_format);

        ui.selectable_value(&mut self.width, FletcherhWidth::W16, "Fletcher-16");
        ui.selectable_value(&mut self.width, FletcherhWidth::W32, "Fletcher-32");
        ui.selectable_value(&mut self.width, FletcherhWidth::W64, "Fletcher-64");

        ui.add_space(16.0);
    }

    fn hash_string(&self, text: &str) -> Result<String, hashers::errors::HasherError> {
        let bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| hashers::errors::HasherError::general("byte format error"))?;

        let h = Fletcher::init(self.width).hash(&bytes);

        Ok(self.output_format.byte_slice_to_text(&h))
    }
}
