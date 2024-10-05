use crate::ui_elements::UiElements;

use super::HasherFrame;
use hashers::{
    blake::{blake256::Blake256, blake512::Blake512},
    errors::HasherError,
    traits::ClassicHasher,
};
use rand::{thread_rng, Rng};
use utils::byte_formatting::ByteFormat;

#[derive(Debug, PartialEq, Eq)]
enum BlakeVariant {
    B224,
    B256,
    B384,
    B512,
}

pub struct BlakeFrame {
    variant: BlakeVariant,
    hasher_224: Blake256,
    hasher_256: Blake256,
    hasher_384: Blake512,
    hasher_512: Blake512,
    salt_224: String,
    salt_256: String,
    salt_384: String,
    salt_512: String,
    salt_error: String,
}

impl Default for BlakeFrame {
    fn default() -> Self {
        Self {
            variant: BlakeVariant::B256,
            hasher_224: Blake256::blake224(),
            hasher_256: Blake256::blake256(),
            hasher_384: Blake512::blake384(),
            hasher_512: Blake512::blake512(),
            salt_224: String::new(),
            salt_256: String::new(),
            salt_384: String::new(),
            salt_512: String::new(),
            salt_error: String::new(),
        }
    }
}

impl BlakeFrame {
    fn salt_control_32(
        ui: &mut egui::Ui,
        string: &mut String,
        salt: &mut [u32; 4],
        error: &mut String,
    ) {
        ui.horizontal(|ui| {
            if ui.control_string(string).changed() {
                *string = string.chars().filter(|c| c.is_ascii_hexdigit()).collect();
                if ui.button("🎲").on_hover_text("randomize").clicked() {
                    let mut rng = thread_rng();
                    rng.fill(salt);
                    *string = ByteFormat::Hex.u32_slice_to_text_be(&salt);
                }
                match ByteFormat::Hex.text_to_u32_be(string) {
                    Ok(mut new) => {
                        while new.len() < 4 {
                            new.push(0)
                        }
                        new.truncate(4);
                        *salt = new.try_into().expect("input too long");
                        error.clear();
                    }
                    Err(_) => *error = String::from("unable to parse salt"),
                };
            }
        });
    }

    fn salt_control_64(
        ui: &mut egui::Ui,
        string: &mut String,
        salt: &mut [u64; 4],
        error: &mut String,
    ) {
        ui.horizontal(|ui| {
            if ui.control_string(string).changed() {
                *string = string.chars().filter(|c| c.is_ascii_hexdigit()).collect();
                if ui.button("🎲").on_hover_text("randomize").clicked() {
                    let mut rng = thread_rng();
                    rng.fill(salt);
                    *string = ByteFormat::Hex.u64_slice_to_text_be(&salt);
                }
                match ByteFormat::Hex.text_to_u64_be(string) {
                    Ok(mut new) => {
                        while new.len() < 4 {
                            new.push(0)
                        }
                        new.truncate(4);
                        *salt = new.try_into().expect("input too long");
                        error.clear();
                    }

                    Err(_) => *error = String::from("unable to parse salt"),
                };
            }
        });
    }
}

impl HasherFrame for BlakeFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/hashers/src/blake",
        );
        ui.add_space(8.0);

        match self.variant {
            BlakeVariant::B224 => ui.byte_io_mode_hasher(
                &mut self.hasher_224.input_format,
                &mut self.hasher_224.output_format,
            ),
            BlakeVariant::B256 => ui.byte_io_mode_hasher(
                &mut self.hasher_256.input_format,
                &mut self.hasher_256.output_format,
            ),
            BlakeVariant::B384 => ui.byte_io_mode_hasher(
                &mut self.hasher_384.input_format,
                &mut self.hasher_384.output_format,
            ),
            BlakeVariant::B512 => ui.byte_io_mode_hasher(
                &mut self.hasher_512.input_format,
                &mut self.hasher_512.output_format,
            ),
        }

        ui.add_space(16.0);

        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.variant, BlakeVariant::B224, "BLAKE-224");
            ui.selectable_value(&mut self.variant, BlakeVariant::B256, "BLAKE-256");
            ui.selectable_value(&mut self.variant, BlakeVariant::B384, "BLAKE-384");
            ui.selectable_value(&mut self.variant, BlakeVariant::B512, "BLAKE-512");
        });

        match self.variant {
            BlakeVariant::B224 => {
                ui.label("BLAKE-224 is the truncated version for 32-bit hardware.")
            }
            BlakeVariant::B256 => {
                ui.label("BLAKE-256 is the full length version for 32-bit hardware.")
            }
            BlakeVariant::B384 => {
                ui.label("BLAKE-384 is the truncated version for 64-bit hardware.")
            }
            BlakeVariant::B512 => {
                ui.label("BLAKE-512 is the full length version for 64-bit hardware.")
            }
        };

        ui.add_space(16.0);
        ui.subheading("Salt (provide as hexadecimal)");
        ui.horizontal(|ui| {
            match self.variant {
                BlakeVariant::B224 => {
                    ui.label("BLAKE-224 has a salt with four 32-bit words (256 bits).");
                    Self::salt_control_32(
                        ui,
                        &mut self.salt_224,
                        &mut self.hasher_224.salt,
                        &mut self.salt_error,
                    )
                }
                BlakeVariant::B256 => {
                    ui.label("BLAKE-256 has a salt with four 32-bit words (256 bits).");
                    Self::salt_control_32(
                        ui,
                        &mut self.salt_256,
                        &mut self.hasher_256.salt,
                        &mut self.salt_error,
                    )
                }
                BlakeVariant::B384 => {
                    ui.label("BLAKE-384 has a salt with four 64-bit words (512 bits).");
                    Self::salt_control_64(
                        ui,
                        &mut self.salt_384,
                        &mut self.hasher_384.salt,
                        &mut self.salt_error,
                    )
                }
                BlakeVariant::B512 => {
                    ui.label("BLAKE-512 has a salt with four 64-bit words (512 bits).");
                    Self::salt_control_64(
                        ui,
                        &mut self.salt_512,
                        &mut self.hasher_512.salt,
                        &mut self.salt_error,
                    )
                }
            };
            ui.error_text(&self.salt_error);
        });

        // ui.label("<<<EXPLANATION OF HASH FUNCTION CODE>>>");

        ui.add_space(16.0);
    }

    fn hash_string(&self, text: &str) -> Result<String, HasherError> {
        match self.variant {
            BlakeVariant::B224 => self.hasher_224.hash_bytes_from_string(text),
            BlakeVariant::B256 => self.hasher_256.hash_bytes_from_string(text),
            BlakeVariant::B384 => self.hasher_384.hash_bytes_from_string(text),
            BlakeVariant::B512 => self.hasher_512.hash_bytes_from_string(text),
        }
    }
}
