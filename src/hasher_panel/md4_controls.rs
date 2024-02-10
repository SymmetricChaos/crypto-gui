use super::{byte_formatting_io, HasherFrame};
use hashers::{errors::HasherError, md4::Md5, traits::ClassicHasher};

pub struct Md4Frame {
    hasher: Md5,
}

impl Default for Md4Frame {
    fn default() -> Self {
        Self {
            hasher: Default::default(),
        }
    }
}

impl Md4Frame {}

impl HasherFrame for Md4Frame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.add_space(16.0);

        byte_formatting_io(
            ui,
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
