use crate::ui_elements::UiElements;

use super::{byte_formatting_io, HasherFrame};
use hashers::{
    blake::{Blake2b, Blake2s},
    errors::HasherError,
    traits::ClassicHasher,
};
use utils::byte_formatting::ByteFormat;

#[derive(Debug, PartialEq, Eq)]
enum Blake2Variant {
    Big,
    Small,
}

pub struct Blake2Frame {
    variant: Blake2Variant,
    hasher_b: Blake2b,
    hasher_s: Blake2s,
    key_string_b: String,
    key_string_s: String,
}

impl Default for Blake2Frame {
    fn default() -> Self {
        Self {
            variant: Blake2Variant::Big,
            hasher_b: Default::default(),
            hasher_s: Default::default(),
            key_string_b: String::new(),
            key_string_s: String::new(),
        }
    }
}

impl Blake2Frame {}

impl HasherFrame for Blake2Frame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.add_space(16.0);

        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.variant, Blake2Variant::Big, "BLAKE2b");
            ui.selectable_value(&mut self.variant, Blake2Variant::Small, "BLAKE2bs");
        });

        ui.add_space(16.0);
        match self.variant {
            Blake2Variant::Big => byte_formatting_io(
                ui,
                &mut self.hasher_b.input_format,
                &mut self.hasher_b.output_format,
            ),
            Blake2Variant::Small => byte_formatting_io(
                ui,
                &mut self.hasher_s.input_format,
                &mut self.hasher_s.output_format,
            ),
        }

        ui.add_space(16.0);
        ui.subheading("Key (hexadecimal)");
        match self.variant {
            Blake2Variant::Big => {
                if ui.control_string(&mut self.key_string_b).changed() {
                    match ByteFormat::Hex.text_to_bytes(&self.key_string_b) {
                        Ok(bytes) => self.hasher_b.key = bytes,
                        Err(_) => {
                            ui.error_text("unable to read key");
                        }
                    };
                }
            }
            Blake2Variant::Small => {
                if ui.control_string(&mut self.key_string_s).changed() {
                    match ByteFormat::Hex.text_to_bytes(&self.key_string_s) {
                        Ok(bytes) => self.hasher_s.key = bytes,
                        Err(_) => {
                            ui.error_text("unable to read key");
                        }
                    };
                }
            }
        };

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
