use super::HasherFrame;
use crate::ui_elements::UiElements;
use egui::DragValue;
use hashers::{
    errors::HasherError,
    radio_gatun::{RadioGatun32, RadioGatun64},
    traits::ClassicHasher,
};

pub struct RadioGatunFrame {
    hasher32: RadioGatun32,
    hasher64: RadioGatun64,
    wide: bool,
}

impl Default for RadioGatunFrame {
    fn default() -> Self {
        Self {
            hasher32: Default::default(),
            hasher64: Default::default(),
            wide: false,
        }
    }
}

impl RadioGatunFrame {
    fn byte_formatting(&mut self, ui: &mut egui::Ui) {
        ui.byte_io_mode_hasher(
            &mut self.hasher32.input_format,
            &mut self.hasher32.output_format,
        );
        self.hasher64.input_format = self.hasher32.input_format;
        self.hasher64.output_format = self.hasher32.output_format;
    }
}

impl HasherFrame for RadioGatunFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.add_space(16.0);

        ui.checkbox(&mut self.wide, "Use 64-Bit Version");
        ui.add_space(8.0);

        self.byte_formatting(ui);
        ui.add_space(8.0);
        ui.subheading("Hash Length (bytes)");
        if ui
            .add(DragValue::new(&mut self.hasher32.hash_len).range(1..=512))
            .changed()
        {
            self.hasher64.hash_len = self.hasher32.hash_len as u64;
        }

        ui.add_space(16.0);
    }

    fn hash_string(&self, text: &str) -> Result<String, HasherError> {
        if self.wide {
            self.hasher64.hash_bytes_from_string(text)
        } else {
            self.hasher32.hash_bytes_from_string(text)
        }
    }
}
