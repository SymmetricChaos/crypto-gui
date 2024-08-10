use crate::ui_elements::UiElements;

use super::HasherFrame;
use hashers::{errors::HasherError, md2::Md2, traits::ClassicHasher};

pub struct Md2Frame {
    hasher: Md2,
}

impl Default for Md2Frame {
    fn default() -> Self {
        Self {
            hasher: Default::default(),
        }
    }
}

impl Md2Frame {}

impl HasherFrame for Md2Frame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.add_space(16.0);

        ui.byte_io_mode_hasher(
            &mut self.hasher.input_format,
            &mut self.hasher.output_format,
        );

        ui.add_space(16.0);
        ui.subheading("Example Initialization");
        ui.monospace(
            "Text (UTF-8):  Hồ Chí Minh\nBytes:         48 e1 bb 93 43 68 c3 ad 4d 69 6e 68\nWith Padding:  48 e1 bb 93 20 43 68 c3 ad 20 4d 69 6e 68 02 02\nChecksum:      94 a4 ca 3e 82 dc 85 07 f9 cb 87 4b d4 41 91 ad",
        );
        ui.label(
            "The actual input to the hash function is the padded text followed by the checksum. ",
        );

        ui.add_space(16.0);
    }

    fn hash_bytes_from_string(&self, text: &str) -> Result<String, HasherError> {
        self.hasher.hash_bytes_from_string(text)
    }
}
