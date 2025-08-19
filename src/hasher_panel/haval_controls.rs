use super::HasherFrame;
use crate::ui_elements::UiElements;
use egui::Slider;
use hashers::{errors::HasherError, haval::Haval, traits::StatefulHasher};
use utils::byte_formatting::ByteFormat;

pub struct HavalFrame {
    input_format: ByteFormat,
    output_format: ByteFormat,
    rounds: u32,
    hash_len: u32,
}

impl Default for HavalFrame {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            rounds: 5,
            hash_len: 32,
        }
    }
}


impl HasherFrame for HavalFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/hashers/src/haval.rs",
        );
        ui.add_space(8.0);

        ui.byte_io_mode_hasher(&mut self.input_format, &mut self.output_format);
        ui.add_space(4.0);

        ui.subheading("Rounds");
        ui.add(Slider::new(&mut self.rounds, 3..=5));

        ui.subheading("Hash Length");
        ui.selectable_value(&mut self.hash_len, 16, "128 bits");
        ui.selectable_value(&mut self.hash_len, 20, "160 bits");
        ui.selectable_value(&mut self.hash_len, 24, "192 bits");
        ui.selectable_value(&mut self.hash_len, 28, "224 bits");
        ui.selectable_value(&mut self.hash_len, 32, "256 bits");

        ui.add_space(16.0);
    }

    fn hash_string(&self, text: &str) -> Result<String, HasherError> {
        let bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| hashers::errors::HasherError::general("byte format error"))?;

        let h = Haval::init(self.hash_len, self.rounds).hash(&bytes);

        Ok(self.output_format.byte_slice_to_text(&h))
    }
}
