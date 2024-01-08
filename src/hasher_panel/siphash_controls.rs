use crate::ui_elements::UiElements;

use super::HasherFrame;
use egui::DragValue;
use hashers::{siphash::SipHash, traits::ClassicHasher};

pub struct SipHashFrame {
    hasher: SipHash,
    k0: u64,
    k1: u64,
}

impl Default for SipHashFrame {
    fn default() -> Self {
        Self {
            hasher: Default::default(),
            k0: 0,
            k1: 0,
        }
    }
}

impl SipHashFrame {}

impl HasherFrame for SipHashFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.add_space(16.0);

        ui.subheading("Key0");
        if ui.add(DragValue::new(&mut self.k0)).changed() {
            self.hasher.set_keys(self.k0, self.k1)
        }
        ui.add_space(8.0);
        ui.subheading("Key1");
        if ui.add(DragValue::new(&mut self.k1)).changed() {
            self.hasher.set_keys(self.k0, self.k1)
        }
        ui.add_space(16.0);
        ui.subheading("Compression Rounds");
        ui.add(DragValue::new(&mut self.hasher.compression_rounds).clamp_range(0..=8));

        ui.add_space(8.0);
        ui.subheading("Finalization Rounds");
        ui.add(DragValue::new(&mut self.hasher.finalization_rounds).clamp_range(1..=10));

        ui.add_space(16.0);
    }

    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        self.hasher.hash(bytes)
    }

    fn hash_to_string(&self, bytes: &[u8]) -> String {
        self.hasher.hash_to_string(bytes)
    }
}
