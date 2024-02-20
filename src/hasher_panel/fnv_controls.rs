use crate::ui_elements::UiElements;

use super::{byte_formatting_io, HasherFrame};
use hashers::{
    errors::HasherError,
    fnv::{Fnv, PrimeSize, O128, O256, O32, O64, P128, P256, P32, P64},
    traits::ClassicHasher,
};

pub struct FnvFrame {
    hasher: Fnv,
}

impl Default for FnvFrame {
    fn default() -> Self {
        Self {
            hasher: Default::default(),
        }
    }
}

impl FnvFrame {}

impl HasherFrame for FnvFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.add_space(16.0);
        byte_formatting_io(
            ui,
            &mut self.hasher.input_format,
            &mut self.hasher.output_format,
        );

        ui.add_space(16.0);
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.hasher.size, PrimeSize::P32, "32-bit");
            ui.selectable_value(&mut self.hasher.size, PrimeSize::P64, "64-bit");
            ui.selectable_value(&mut self.hasher.size, PrimeSize::P128, "128-bit");
        });

        ui.add_space(16.0);
        ui.label("In the original FNV algorithm the multiplication was performed before the XOR but better results were found when using the XOR first.");
        ui.checkbox(
            &mut self.hasher.alternate,
            "Use Alternate Mode (recommended)",
        );

        ui.subheading("Hash Size");
        ui.label("Three FNV variants are provided here but the original FNV paper defines constants for 512 and 1024 versions of the algorithm. However these run much more slowly and add no real utility.");
        match self.hasher.size {
            PrimeSize::P32 => ui.label(format!(
                "FNV-32 uses the prime {} as the multiplier and is initialized with a value of {}",
                P32, O32,
            )),
            PrimeSize::P64 => ui.label(format!(
                "FNV-64 uses the prime {} as the multiplier and is initialized with a value of {}",
                P64, O64,
            )),
            PrimeSize::P128 => ui.label(format!(
                "FNV-128 uses the prime {} as the multiplier and is initialized with a value of {}",
                P128, O128,
            )),
            PrimeSize::P256 => ui.label(format!(
                "FNV-256 uses the prime {} as the multiplier and is initialized with a value of {}",
                P256, O256,
            )),
        };

        ui.add_space(16.0);
    }

    fn hash_bytes_from_string(&self, text: &str) -> Result<String, HasherError> {
        self.hasher.hash_bytes_from_string(text)
    }
}
