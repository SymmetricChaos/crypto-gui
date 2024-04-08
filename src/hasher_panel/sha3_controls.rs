use egui::{DragValue, RichText};
use hashers::{errors::HasherError, keccak::Keccak, traits::ClassicHasher};

use crate::ui_elements::UiElements;

use super::{byte_formatting_io, HasherFrame};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Sha3Variant {
    Sha3_224,
    Sha3_256,
    Sha3_384,
    Sha3_512,
    Shake128,
    Shake256,
}

pub struct Sha3Frame {
    hasher: Keccak,
    variant: Sha3Variant,
    shake_output_len: usize,
}

impl Default for Sha3Frame {
    fn default() -> Self {
        Self {
            hasher: Default::default(),
            variant: Sha3Variant::Sha3_256,
            shake_output_len: 128,
        }
    }
}

impl Sha3Frame {
    fn set_hasher(&mut self) {
        match self.variant {
            Sha3Variant::Sha3_224 => self.hasher = Keccak::sha3_224(),
            Sha3Variant::Sha3_256 => self.hasher = Keccak::sha3_256(),
            Sha3Variant::Sha3_384 => self.hasher = Keccak::sha3_384(),
            Sha3Variant::Sha3_512 => self.hasher = Keccak::sha3_512(),
            Sha3Variant::Shake128 => self.hasher = Keccak::shake_128(self.shake_output_len),
            Sha3Variant::Shake256 => self.hasher = Keccak::shake_256(self.shake_output_len),
        }
    }
}

impl HasherFrame for Sha3Frame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.add_space(16.0);

        byte_formatting_io(
            ui,
            &mut self.hasher.input_format,
            &mut self.hasher.output_format,
        );

        ui.add_space(16.0);
        ui.subheading("SHA-3 Hash Algorithms");
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.variant, Sha3Variant::Sha3_224, "SHA3-224");
            ui.selectable_value(&mut self.variant, Sha3Variant::Sha3_256, "SHA3-256");
            ui.selectable_value(&mut self.variant, Sha3Variant::Sha3_384, "SHA3-384");
            ui.selectable_value(&mut self.variant, Sha3Variant::Sha3_512, "SHA3-512");
        });
        ui.add_space(8.0);
        ui.subheading("SHA-3 Extensible Output Functions");
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.variant, Sha3Variant::Shake128, "SHAKE128");
            ui.selectable_value(&mut self.variant, Sha3Variant::Shake256, "SHAKE256");
        });

        ui.add_space(8.0);
        ui.subheading("SHAKE Output Length");
        ui.add_enabled(
            self.variant == Sha3Variant::Shake128 || self.variant == Sha3Variant::Shake256,
            DragValue::new(&mut self.shake_output_len).clamp_range(1..=512),
        );

        ui.add_space(16.0);
        ui.subheading("Discussion");
        match self.variant {
            Sha3Variant::Sha3_224 => ui.label("SHA3-224 "),
            Sha3Variant::Sha3_256 => ui.label("SHA3-256 "),
            Sha3Variant::Sha3_384 => ui.label("SHA3-384 "),
            Sha3Variant::Sha3_512 => ui.label("SHA3-512 "),
            Sha3Variant::Shake128 => ui.label("SHAKE128 "),
            Sha3Variant::Shake256 => ui.label("SHAKE256 "),
        };

        ui.label("<<<EXPLANATION OF HASH FUNCTION CODE>>>");

        ui.add_space(16.0);
    }

    fn hash_bytes_from_string(&self, text: &str) -> Result<String, HasherError> {
        self.hasher.hash_bytes_from_string(text)
    }
}
