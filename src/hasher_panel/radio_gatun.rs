use super::{byte_formatting_io, HasherFrame};
use crate::ui_elements::UiElements;
use egui::DragValue;
use hashers::{errors::HasherError, radio_gatun::RadioGatun32, traits::ClassicHasher};

pub struct RadioGatunFrame {
    hasher: RadioGatun32,
}

impl Default for RadioGatunFrame {
    fn default() -> Self {
        Self {
            hasher: Default::default(),
        }
    }
}

impl RadioGatunFrame {}

impl HasherFrame for RadioGatunFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.add_space(16.0);

        byte_formatting_io(
            ui,
            &mut self.hasher.input_format,
            &mut self.hasher.output_format,
        );

        ui.add_space(8.0);
        ui.subheading("Hash Length (bytes)");
        ui.add(DragValue::new(&mut self.hasher.hash_len).clamp_range(1..=512));

        ui.add_space(16.0);
    }

    fn hash_bytes_from_string(&self, text: &str) -> Result<String, HasherError> {
        self.hasher.hash_bytes_from_string(text)
    }
}
