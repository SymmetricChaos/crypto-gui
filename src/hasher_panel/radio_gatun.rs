use super::HasherFrame;
use crate::ui_elements::UiElements;
use egui::DragValue;
use hashers::{
    errors::HasherError,
    radio_gatun::{RadioGatun32, RadioGatun64},
    traits::StatefulHasher,
};
use utils::byte_formatting::ByteFormat;

pub struct RadioGatunFrame {
    input_format: ByteFormat,
    output_format: ByteFormat,
    wide: bool,
    hash_len: u32,
}

impl Default for RadioGatunFrame {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            wide: false,
            hash_len: 32,
        }
    }
}

impl RadioGatunFrame {
    fn byte_formatting(&mut self, ui: &mut egui::Ui) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/hashers/src/radio_gatun.rs",
        );

        ui.byte_io_mode_hasher(&mut self.input_format, &mut self.output_format);
    }
}

impl HasherFrame for RadioGatunFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.add_space(16.0);

        ui.checkbox(&mut self.wide, "Use 64-Bit Version");
        ui.add_space(8.0);

        self.byte_formatting(ui);
        ui.add_space(8.0);
        ui.subheading("Hash Length (Bytes)");
        ui.add(DragValue::new(&mut self.hash_len).range(1..=512));

        ui.add_space(16.0);
    }

    fn hash_string(&self, text: &str) -> Result<String, HasherError> {
        let bytes: Vec<u8> = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| hashers::errors::HasherError::general("byte format error"))?;

        let h = if self.wide {
            RadioGatun64::init(self.hash_len as u64).update_and_finalize(&bytes)
        } else {
            RadioGatun32::init(self.hash_len).update_and_finalize(&bytes)
        };

        Ok(self.output_format.byte_slice_to_text(&h))
    }
}
