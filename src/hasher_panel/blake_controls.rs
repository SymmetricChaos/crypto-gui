use crate::ui_elements::UiElements;

use super::{byte_formatting_io, HasherFrame};
use hashers::{
    blake::{blake256::Blake256, blake512::Blake512},
    errors::HasherError,
    traits::ClassicHasher,
};
use itertools::Itertools;
use rand::{thread_rng, Rng, RngCore};
use utils::byte_formatting::ByteFormat;

#[derive(Debug, PartialEq, Eq)]
enum BlakeVariant {
    B224,
    B256,
    B384,
    B512,
}

pub struct Blake2Frame {
    variant: BlakeVariant,
    hasher_224: Blake256,
    hasher_256: Blake256,
    hasher_384: Blake512,
    hasher_512: Blake512,
    salt_string: String,
}

impl Default for Blake2Frame {
    fn default() -> Self {
        Self {
            variant: BlakeVariant::B256,
            hasher_224: Blake256::blake224(),
            hasher_256: Blake256::blake256(),
            hasher_384: Blake512::blake384(),
            hasher_512: Blake512::blake512(),
            salt_string: String::new(),
        }
    }
}

impl Blake2Frame {
    fn salt_control_32(ui: &mut egui::Ui, string: &mut String, salt: &mut [u32; 4]) {
        ui.horizontal(|ui| {
            if ui.control_string(string).changed() {
                *string = string.chars().filter(|c| c.is_ascii_hexdigit()).collect();
                if ui.button("🎲").on_hover_text("randomize").clicked() {
                    let mut rng = thread_rng();
                    rng.fill(salt);
                    *string = ByteFormat::Hex.bytes_to_text(
                        &salt.iter().map(|x| x.to_be_bytes()).flatten().collect_vec(),
                    )
                }
                match ByteFormat::Hex.text_to_bytes(string) {
                    Ok(new) => *bytes = new,
                    Err(_) => {
                        ui.error_text("unable to read salt");
                    }
                };
            }
        });
    }
}

impl HasherFrame for Blake2Frame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
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
        match self.variant {
            BlakeVariant::B224 => byte_formatting_io(
                ui,
                &mut self.hasher_224.input_format,
                &mut self.hasher_224.output_format,
            ),
            BlakeVariant::B256 => byte_formatting_io(
                ui,
                &mut self.hasher_256.input_format,
                &mut self.hasher_256.output_format,
            ),
            BlakeVariant::B384 => byte_formatting_io(
                ui,
                &mut self.hasher_384.input_format,
                &mut self.hasher_384.output_format,
            ),
            BlakeVariant::B512 => byte_formatting_io(
                ui,
                &mut self.hasher_512.input_format,
                &mut self.hasher_512.output_format,
            ),
        }

        ui.add_space(16.0);
        ui.subheading("Key (provide as hexadecimal)");
        ui.label("The BLAKE functions allow a salt to be included. It consists of four integers totalling 128 bits for BLAKE-224/BLAKE-256 and 256 bits for BLAKE-382/BLAKE-512.");
        match self.variant {
            BlakeVariant::B224 => todo!(),
            BlakeVariant::B256 => todo!(),
            BlakeVariant::B384 => todo!(),
            BlakeVariant::B512 => {
                ui.label("BLAKE2s has a maximum key size of of 32 bytes (256 bits).");
                Self::key_control(ui, &mut self.salt_string, &mut self.hasher_512.salt)
            }
        };

        // ui.label("<<<EXPLANATION OF HASH FUNCTION CODE>>>");

        ui.add_space(16.0);
    }

    fn hash_bytes_from_string(&self, text: &str) -> Result<String, HasherError> {
        match self.variant {
            BlakeVariant::B224 => self.hasher_224.hash_bytes_from_string(text),
            BlakeVariant::B256 => self.hasher_256.hash_bytes_from_string(text),
            BlakeVariant::B384 => self.hasher_384.hash_bytes_from_string(text),
            BlakeVariant::B512 => self.hasher_512.hash_bytes_from_string(text),
        }
    }
}
