use super::HasherFrame;
use hashers::{md4::Md5, traits::ClassicHasher};

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

        ui.label("<<<EXPLANATION OF HASH FUNCTION CODE>>>");

        ui.add_space(16.0);
    }

    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        self.hasher.hash(bytes)
    }

    fn hash_to_string(&self, bytes: &[u8]) -> String {
        self.hasher.hash_to_string(bytes)
    }
}
