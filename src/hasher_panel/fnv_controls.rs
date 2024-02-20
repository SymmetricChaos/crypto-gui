use super::{byte_formatting_io, HasherFrame};
use hashers::{
    errors::HasherError,
    fnv::{Fnv, PrimeSize},
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

        ui.label("<<<EXPLANATION OF HASH FUNCTION CODE>>>");

        ui.add_space(16.0);
    }

    fn hash_bytes_from_string(&self, text: &str) -> Result<String, HasherError> {
        self.hasher.hash_bytes_from_string(text)
    }
}
