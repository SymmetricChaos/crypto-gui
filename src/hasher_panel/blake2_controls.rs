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
    hash_len_b: String,
    hash_len_s: String,
}

impl Default for Blake2Frame {
    fn default() -> Self {
        Self {
            variant: Blake2Variant::Big,
            hasher_b: Default::default(),
            hasher_s: Default::default(),
            key_string_b: String::new(),
            key_string_s: String::new(),
            hash_len_b: String::from("32"),
            hash_len_s: String::from("16"),
        }
    }
}

impl Blake2Frame {
    fn key_control(ui: &mut egui::Ui, string: &mut String, bytes: &mut Vec<u8>) {
        if ui.control_string(string).changed() {
            *string = string.chars().filter(|c| c.is_ascii_hexdigit()).collect();
            match ByteFormat::Hex.text_to_bytes(string) {
                Ok(new) => *bytes = new,
                Err(_) => {
                    ui.error_text("unable to read key");
                }
            };
        }
    }

    fn hash_len_control(ui: &mut egui::Ui, string: &mut String, value: &mut usize, max: usize) {
        if ui.control_string(string).changed() {
            *string = string
                .chars()
                .filter(|c| c.is_ascii_digit())
                .take(2)
                .collect();
            match usize::from_str_radix(string, 10) {
                Ok(new) => {
                    if new == 0 || new > max {
                        ui.error_text("invalid hash length_size");
                    } else {
                        *value = new
                    }
                }
                Err(_) => {
                    ui.error_text("unable to parse hash_len value");
                }
            };
        }
    }
}

impl HasherFrame for Blake2Frame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.add_space(16.0);

        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.variant, Blake2Variant::Big, "BLAKE2b");
            ui.selectable_value(&mut self.variant, Blake2Variant::Small, "BLAKE2bs");
        });

        match self.variant {
            Blake2Variant::Big => ui.label("BLAKE2b is designed for 64-bit hardware."),
            Blake2Variant::Small => ui.label("BLAKE2s is designed for 32-bit hardware."),
        };

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

        ui.subheading("Hash Length");
        ui.label("The BLAKE2 functions allow a variety of output lengths specified by how many bytes of the internal state are returned.");
        match self.variant {
            Blake2Variant::Big => {
                ui.label("BLAKE2b has a maximum output of 64 bytes (512 bits).");
                Self::hash_len_control(ui, &mut self.hash_len_b, &mut self.hasher_b.hash_len, 64);
            }
            Blake2Variant::Small => {
                ui.label("BLAKE2s has a maximum output of 32 bytes (256 bits).");
                Self::hash_len_control(ui, &mut self.hash_len_s, &mut self.hasher_s.hash_len, 32);
            }
        }

        ui.add_space(16.0);
        ui.subheading("Key (hexadecimal)");
        ui.label("The BLAKE2 functions allow a key to be included ");
        match self.variant {
            Blake2Variant::Big => {
                Self::key_control(ui, &mut self.key_string_b, &mut self.hasher_b.key)
            }
            Blake2Variant::Small => {
                Self::key_control(ui, &mut self.key_string_s, &mut self.hasher_s.key)
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
