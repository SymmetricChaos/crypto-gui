use crate::ui_elements::UiElements;

use super::HasherFrame;
use hashers::md5::Md5;

pub struct Md5Frame {
    hasher: Md5,
}

impl Default for Md5Frame {
    fn default() -> Self {
        Self {
            hasher: Default::default(),
        }
    }
}

impl Md5Frame {}

impl HasherFrame for Md5Frame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.add_space(16.0);

        ui.byte_io_mode_hasher(
            &mut self.hasher.input_format,
            &mut self.hasher.output_format,
        );

        ui.label("<<<EXPLANATION OF HASH FUNCTION CODE>>>");

        ui.add_space(16.0);
    }
    crate::hash_string! {}
}
