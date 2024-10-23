use super::HasherFrame;
use crate::ui_elements::UiElements;
use hashers::sm3::Sm3;

pub struct Sm3Frame {
    hasher: Sm3,
}

impl Default for Sm3Frame {
    fn default() -> Self {
        Self {
            hasher: Default::default(),
        }
    }
}

impl HasherFrame for Sm3Frame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/hashers/src/sm3.rs",
        );

        ui.byte_io_mode_hasher(
            &mut self.hasher.input_format,
            &mut self.hasher.output_format,
        );

        ui.add_space(16.0);
    }
    crate::hash_string! {}
}
