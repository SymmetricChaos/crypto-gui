use super::HasherFrame;
use crate::ui_elements::UiElements;
use hashers::{
    cityhash::{cityhash32::CityHash32, cityhash64::CityHash64},
    errors::HasherError,
    traits::StatefulHasher,
};
use utils::byte_formatting::ByteFormat;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CityHashVariant {
    V32,
    V64,
}

pub struct CityHashFrame {
    input_format: ByteFormat,
    output_format: ByteFormat,
    variant: CityHashVariant,
}

impl Default for CityHashFrame {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            variant: CityHashVariant::V32,
        }
    }
}

impl CityHashFrame {}

impl HasherFrame for CityHashFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/hashers/src/cityhash",
        );

        ui.add_space(8.0);
        ui.byte_io_mode_hasher(&mut self.input_format, &mut self.output_format);

        ui.add_space(16.0);
        ui.subheading("Variant");
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.variant, CityHashVariant::V32, "32-bit");
            ui.selectable_value(&mut self.variant, CityHashVariant::V64, "64-bit");
        });
        ui.add_space(8.0);

        ui.add_space(16.0);
        ui.subheading("Discussion");
        match self.variant {
            CityHashVariant::V32 => ui.label(""),
            CityHashVariant::V64 => ui.label(""),
        };

        ui.add_space(16.0);
    }

    fn hash_string(&self, text: &str) -> Result<String, HasherError> {
        let bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| hashers::errors::HasherError::general("byte format error"))?;

        let h = match self.variant {
            CityHashVariant::V32 => CityHash32::init().hash(&bytes),
            CityHashVariant::V64 => CityHash64::init(None).hash(&bytes),
        };

        Ok(self.output_format.byte_slice_to_text(&h))
    }
}
