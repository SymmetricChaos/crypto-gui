use hashers::{sha1::Sha1, traits::ClassicHasher};

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