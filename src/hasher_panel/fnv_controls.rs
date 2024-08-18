use crate::ui_elements::UiElements;

use super::HasherFrame;
use hashers::fnv::{Fnv, FnvSize, O128, O32, O64, P128, P32, P64};

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
        ui.byte_io_mode_hasher(
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
        ui.label("Zero basis mode initializes the hash with all zeroes and when used this way the hasher is called FNV-0. The usual initialization value for FNV-1 and (FNV-1a) was created by hashing the ASCII string \"chongo <Landon Curt Noll> /\\../\\\" in zero basis mode.");
        ui.checkbox(
            &mut self.hasher.zero_basis,
            "Use Zero Basis Mode (not recommended)",
        );

        ui.add_space(16.0);
        ui.subheading("Hash Size");
        ui.label("The FNV primes are of a specific form, close to a power of 256, which the developers found to be highly effective at dispersing the bits of the input throughout the state. Four FNV variants are provided here but the original FNV paper also defines constants for 512 and 1024 versions of the algorithm. However these run more slowly, take more space to store, and offer no practical increase in collision resistance.");
        ui.add_space(4.0);
        match self.hasher.size {
            FnvSize::P32 => ui.mono_strong(format!(
                "The 32-bit FNV prime is {} and the initialization value is {}",
                P32, O32,
            )),
            FnvSize::P64 => ui.mono_strong(format!(
                "The 64-bit FNV prime is {} and the initialization value is {}",
                P64, O64,
            )),
            FnvSize::P128 => ui.mono_strong(format!(
                "The 128-bit FNV prime is {} and the initialization value is {}",
                P128, O128,
            )),
            // The P256 constants don't format nicely so they are hard-coded here.
            FnvSize::P256 => ui.mono_strong(
                "The 256-bit FNV prime is 374144419156711147060143317175368453031918731002211 and the initialization value is 100029257958052580907070968620625704837092796014241193945225284501741471925557", 
            ),
        };

        ui.add_space(16.0);
    }
    crate::hash_string! {}
}
