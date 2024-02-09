use super::HasherFrame;
use hashers::{
    blake::{Blake2b, Blake2s},
    errors::HasherError,
    traits::ClassicHasher,
};

enum Blake2Variant {
    Big,
    Small,
}

pub struct Blake2Frame {
    variant: Blake2Variant,
    hasher_b: Blake2b,
    hasher_s: Blake2s,
}

impl Default for Blake2Frame {
    fn default() -> Self {
        Self {
            variant: Blake2Variant::Big,
            hasher_b: Default::default(),
            hasher_s: Default::default(),
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

    fn hash_bytes_from_string(&self, text: &str) -> Result<String, HasherError> {
        match self.variant {
            Blake2Variant::Big => self.hasher_b.hash_bytes_from_string(text),
            Blake2Variant::Small => self.hasher_s.hash_bytes_from_string(text),
        }
    }
}
