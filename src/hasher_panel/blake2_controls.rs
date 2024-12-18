use crate::ui_elements::UiElements;

use super::HasherFrame;
use egui::DragValue;
use hashers::{
    blake::{Blake2b, Blake2bLong, Blake2s},
    errors::HasherError,
    traits::ClassicHasher,
};
use rand::{thread_rng, RngCore};
use utils::byte_formatting::ByteFormat;

#[derive(Debug, PartialEq, Eq)]
enum Blake2Variant {
    Big,
    Small,
    BigLong,
}

pub struct Blake2Frame {
    variant: Blake2Variant,
    hasher_b_long: Blake2bLong,
    hasher_b: Blake2b,
    hasher_s: Blake2s,
    key_string_b: String,
    valid_key_b: bool,
    key_string_b_long: String,
    valid_key_b_long: bool,
    key_string_s: String,
    valid_key_s: bool,
}

impl Default for Blake2Frame {
    fn default() -> Self {
        Self {
            variant: Blake2Variant::Big,
            hasher_b_long: Default::default(),
            hasher_b: Default::default(),
            hasher_s: Default::default(),
            key_string_b: String::new(),
            valid_key_b: true,
            key_string_b_long: String::new(),
            valid_key_b_long: true,
            key_string_s: String::new(),
            valid_key_s: true,
        }
    }
}

impl Blake2Frame {
    fn key_control_b(&mut self, ui: &mut egui::Ui) {
        let string = &mut self.key_string_b;
        ui.horizontal(|ui| {
            if ui.control_string(string).changed() {
                *string = string
                    .chars()
                    .filter(|c| c.is_ascii_hexdigit())
                    .take(128) // 128 characters is 64 bytes
                    .collect();
                self.valid_key_b = string.len() % 2 != 0;
                if self.valid_key_b {
                    if let Ok(new) = ByteFormat::Hex.text_to_bytes(string) {
                        match new.try_into() {
                            Ok(key) => self.hasher_b.key = key,
                            Err(_) => unreachable!(),
                        }
                    } else {
                        unreachable!("unable to parse input");
                    }
                }
            };
            if ui.button("🎲").on_hover_text("randomize").clicked() {
                let mut rng = thread_rng();
                rng.fill_bytes(&mut self.hasher_b.key);
                *string = ByteFormat::Hex.byte_slice_to_text(&self.hasher_b.key)
            };
        });

        if !self.valid_key_b {
            ui.error_text("invalid key");
        } else {
            ui.error_text("");
        }
    }

    fn key_control_b_long(&mut self, ui: &mut egui::Ui) {
        let string = &mut self.key_string_b_long;
        ui.horizontal(|ui| {
            if ui.control_string(string).changed() {
                *string = string
                    .chars()
                    .filter(|c| c.is_ascii_hexdigit())
                    .take(128) // 128 characters is 64 bytes
                    .collect();
                self.valid_key_b_long = string.len() % 2 != 0;
                if self.valid_key_b_long {
                    if let Ok(new) = ByteFormat::Hex.text_to_bytes(string) {
                        match new.try_into() {
                            Ok(key) => self.hasher_b_long.key = key,
                            Err(_) => unreachable!(),
                        }
                    } else {
                        unreachable!("unable to parse input");
                    }
                }
            };
            if ui.button("🎲").on_hover_text("randomize").clicked() {
                let mut rng = thread_rng();
                rng.fill_bytes(&mut self.hasher_b_long.key);
                *string = ByteFormat::Hex.byte_slice_to_text(&self.hasher_b_long.key)
            };
        });

        if !self.valid_key_b_long {
            ui.error_text("invalid key");
        } else {
            ui.error_text("");
        }
    }

    fn key_control_s(&mut self, ui: &mut egui::Ui) {
        let string = &mut self.key_string_s;
        ui.horizontal(|ui| {
            if ui.control_string(string).changed() {
                *string = string
                    .chars()
                    .filter(|c| c.is_ascii_hexdigit())
                    .take(64) // 64 characters is 32 bytes
                    .collect();
                self.valid_key_s = string.len() % 2 != 0;
                if self.valid_key_s {
                    if let Ok(new) = ByteFormat::Hex.text_to_bytes(string) {
                        match new.try_into() {
                            Ok(key) => self.hasher_s.key = key,
                            Err(_) => unreachable!(),
                        }
                    } else {
                        unreachable!("unable to parse input");
                    }
                }
            };
            if ui.button("🎲").on_hover_text("randomize").clicked() {
                let mut rng = thread_rng();
                rng.fill_bytes(&mut self.hasher_s.key);
                *string = ByteFormat::Hex.byte_slice_to_text(&self.hasher_s.key)
            };
        });

        if !self.valid_key_s {
            ui.error_text("invalid key");
        } else {
            ui.error_text("");
        }
    }
}

impl HasherFrame for Blake2Frame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/hashers/src/blake",
        );
        ui.add_space(8.0);

        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.variant, Blake2Variant::Big, "BLAKE2b");
            ui.selectable_value(&mut self.variant, Blake2Variant::BigLong, "BLAKE2b-Long");
            ui.selectable_value(&mut self.variant, Blake2Variant::Small, "BLAKE2bs");
        });

        match self.variant {
            Blake2Variant::BigLong => ui.label("BLAKE2b-Long is designed for 64-bit hardware."),
            Blake2Variant::Big => ui.label("BLAKE2b is designed for 64-bit hardware."),
            Blake2Variant::Small => ui.label("BLAKE2s is designed for 32-bit hardware."),
        };

        ui.add_space(16.0);
        match self.variant {
            Blake2Variant::Big => ui.byte_io_mode_hasher(
                &mut self.hasher_b.input_format,
                &mut self.hasher_b.output_format,
            ),
            Blake2Variant::BigLong => ui.byte_io_mode_hasher(
                &mut self.hasher_b_long.input_format,
                &mut self.hasher_b_long.output_format,
            ),
            Blake2Variant::Small => ui.byte_io_mode_hasher(
                &mut self.hasher_s.input_format,
                &mut self.hasher_s.output_format,
            ),
        }

        ui.subheading("Hash Length");
        ui.label("The BLAKE2 functions allow a variety of output lengths specified by how many bytes of the internal state are returned.");
        match self.variant {
            Blake2Variant::Big => {
                ui.label("BLAKE2b has a maximum output of 64 bytes (512 bits).");
                ui.add(DragValue::new(&mut self.hasher_b.hash_len).range(1..=64));
            }
            Blake2Variant::BigLong => {
                ui.label("BLAKE2b-Long has no maximum output length but here is limited to 1024 bytes (8192 bits, 1 kilobyte). For output lengths of 64 bytes or less it is identical to BLAKE2b.");
                ui.add(DragValue::new(&mut self.hasher_b_long.hash_len).range(1..=1024));
            }
            Blake2Variant::Small => {
                ui.label("BLAKE2s has a maximum output of 32 bytes (256 bits).");
                ui.add(DragValue::new(&mut self.hasher_s.hash_len).range(1..=32));
            }
        }

        ui.add_space(16.0);
        ui.subheading("Key (provide as hexadecimal)");
        ui.label("The BLAKE2 functions allow a key to be included.");
        match self.variant {
            Blake2Variant::Big => {
                ui.label("BLAKE2b has a maximum key size of of 64 bytes (512 bits).");
                self.key_control_b(ui);
            }
            Blake2Variant::BigLong => {
                ui.label("BLAKE2b-Long has a maximum key size of of 64 bytes (512 bits).");
                self.key_control_b_long(ui);
            }
            Blake2Variant::Small => {
                ui.label("BLAKE2s has a maximum key size of of 32 bytes (256 bits).");
                self.key_control_s(ui);
            }
        };

        // ui.label("<<<EXPLANATION OF HASH FUNCTION CODE>>>");

        ui.add_space(16.0);
    }

    fn hash_string(&self, text: &str) -> Result<String, HasherError> {
        match self.variant {
            Blake2Variant::Big => self.hasher_b.hash_bytes_from_string(text),
            Blake2Variant::Small => self.hasher_s.hash_bytes_from_string(text),
            Blake2Variant::BigLong => self.hasher_b_long.hash_bytes_from_string(text),
        }
    }
}
