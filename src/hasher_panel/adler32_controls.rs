use hashers::adler::Adler32;

use super::HasherFrame;

pub struct Adler32Frame {
    hasher: Adler32,
}

impl Default for Adler32Frame {
    fn default() -> Self {
        Self {
            hasher: Default::default(),
        }
    }
}

impl Adler32Frame {}

impl HasherFrame for Adler32Frame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/hashers/src/adler32.rs",
        );
        ui.add_space(8.0);

        ui.label("<<<EXPLANATION OF HASH FUNCTION CODE>>>");

        ui.add_space(16.0);
    }

    crate::hash_string! {}
}
