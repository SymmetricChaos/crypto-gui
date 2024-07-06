use hashers::{errors::HasherError, sha1::Sha1, traits::ClassicHasher};

use crate::ui_elements::UiElements;

use super::HasherFrame;

pub struct Sha1Frame {
    hasher: Sha1,
}

impl Default for Sha1Frame {
    fn default() -> Self {
        Self {
            hasher: Default::default(),
        }
    }
}

impl Sha1Frame {}

impl HasherFrame for Sha1Frame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.add_space(16.0);

        ui.byte_io_mode_hasher(
            &mut self.hasher.input_format,
            &mut self.hasher.output_format,
        );

        ui.label("<<<EXPLANATION OF HASH FUNCTION CODE>>>");

        ui.add_space(16.0);
    }

    fn hash_bytes_from_string(&self, text: &str) -> Result<String, HasherError> {
        self.hasher.hash_bytes_from_string(text)
    }
}
