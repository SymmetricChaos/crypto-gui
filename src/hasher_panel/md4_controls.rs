use crate::ui_elements::UiElements;

use super::HasherFrame;
use hashers::md4::Md4;

pub struct Md4Frame {
    hasher: Md4,
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
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/hashers/src/md4.rs",
        );
        ui.add_space(8.0);

        ui.byte_io_mode_hasher(
            &mut self.hasher.input_format,
            &mut self.hasher.output_format,
        );

        ui.label("<<<EXPLANATION OF HASH FUNCTION CODE>>>");

        ui.add_space(16.0);
    }
    crate::hash_string! {}
}
