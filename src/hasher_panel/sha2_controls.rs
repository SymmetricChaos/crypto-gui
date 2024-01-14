use hashers::{
    sha2::{Sha2, Sha2Variant},
    traits::ClassicHasher,
};

use crate::ui_elements::UiElements;

use super::HasherFrame;

pub struct Sha2Frame {
    hasher: Sha2,
}

impl Default for Sha2Frame {
    fn default() -> Self {
        Self {
            hasher: Default::default(),
        }
    }
}

impl Sha2Frame {}

impl HasherFrame for Sha2Frame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.add_space(16.0);

        ui.subheading("SHA-256 based");
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.hasher.variant, Sha2Variant::Sha256, "SHA-256");
            ui.selectable_value(&mut self.hasher.variant, Sha2Variant::Sha224, "SHA-224");
        });

        ui.subheading("SHA-512 based");
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.hasher.variant, Sha2Variant::Sha512, "SHA-512");
            ui.selectable_value(&mut self.hasher.variant, Sha2Variant::Sha384, "SHA-384");
        });

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
