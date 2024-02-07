use super::HasherFrame;
use hashers::{blake2b::Blake2b, errors::HasherError, traits::ClassicHasher};

pub struct Blake2Frame {
    hasher: Blake2b,
}

impl Default for Blake2Frame {
    fn default() -> Self {
        Self {
            hasher: Default::default(),
        }
    }
}

impl Blake2Frame {}

impl HasherFrame for Blake2Frame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.add_space(16.0);

        ui.label("<<<EXPLANATION OF HASH FUNCTION CODE>>>");

        ui.add_space(16.0);
    }

    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        self.hasher.hash(bytes)
    }

    fn hash_bytes_from_string(&self, text: &str) -> Result<String, HasherError> {
        self.hasher.hash_bytes_from_string(text)
    }
}
