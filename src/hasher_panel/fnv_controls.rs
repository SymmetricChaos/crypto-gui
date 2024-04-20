use crate::ui_elements::UiElements;

use super::HasherFrame;
use hashers::{
    errors::HasherError,
    fnv::{Fnv, FnvSize, O128, O256, O32, O64, P128, P256, P32, P64},
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
        ui.byte_io_mode(
            &mut self.hasher.input_format,
            &mut self.hasher.output_format,
        );

        ui.add_space(16.0);
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.hasher.size, FnvSize::P32, "32-bit");
            ui.selectable_value(&mut self.hasher.size, FnvSize::P64, "64-bit");
            ui.selectable_value(&mut self.hasher.size, FnvSize::P128, "128-bit");
            ui.selectable_value(&mut self.hasher.size, FnvSize::P256, "256-bit");
        });

        ui.add_space(16.0);
        ui.label("In the original FNV-1 algorithm the multiplication was performed before the XOR but better results were found when using the XOR first. The algorithms with this alternate order are known as FNV-1a.");
        ui.checkbox(
            &mut self.hasher.alternate,
            "Use Alternate Mode (recommended)",
        );

        ui.add_space(16.0);
        ui.label("The initialization value is for FNV-1 and (FNV-1a) was created by hashing the ASCII string \"chongo <Landon Curt Noll> /\\../\\\" with the zero basis version FNV-1, that is starting with a value of zero. These zero basis version are called FNV-0.");
        ui.checkbox(
            &mut self.hasher.alternate,
            "Use Zero Basis Mode (not recommended)",
        );

        ui.subheading("Hash Size");
        ui.label("The FNV primes are of a specific form, close to a power of 256, which the developers found to be highly effective at dispersing the bits of the input throughout the state. Four FNV variants are provided here but the original FNV paper also defines constants for 512 and 1024 versions of the algorithm. However these run much more slowly, take more space to store, and offer no practical increase in collision resistance.");
        match self.hasher.size {
            FnvSize::P32 => ui.label(format!(
                "The 32-bit FNV-1 prime is {} and the initialization value is {}",
                P32, O32,
            )),
            FnvSize::P64 => ui.label(format!(
                "The 64-bit FNV-1 prime is {} and the initialization value is {}",
                P64, O64,
            )),
            FnvSize::P128 => ui.label(format!(
                "The 128-bit FNV-1 prime is {} and the initialization value is {}",
                P128, O128,
            )),
            FnvSize::P256 => ui.label(format!(
                "The 256-bit FNV-1 prime is {} and the initialization value is {}",
                P256, O256,
            )),
        };

        ui.add_space(16.0);
    }

    fn hash_bytes_from_string(&self, text: &str) -> Result<String, HasherError> {
        self.hasher.hash_bytes_from_string(text)
    }
}
