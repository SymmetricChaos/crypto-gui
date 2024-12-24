use crate::ui_elements::UiElements;

use super::HasherFrame;
use hashers::{
    blake::{blake256::Blake256, blake512::Blake512, Blake224, Blake384},
    errors::HasherError,
    traits::StatefulHasher,
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
    input_format: ByteFormat,
    output_format: ByteFormat,
    salt_32: [u32; 4],
    salt_64: [u64; 4],
    valid_salt: bool,
}

impl Default for BlakeFrame {
    fn default() -> Self {
        Self {
            variant: BlakeVariant::B256,
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            salt_32: [0; 4],
            salt_64: [0; 4],
            valid_salt: true,
        }
    }
}

impl BlakeFrame {
    fn salt_control_32(&mut self, ui: &mut egui::Ui) {
        ui.u32_hex_edit(&mut self.salt_32[0]);
        ui.u32_hex_edit(&mut self.salt_32[1]);
        ui.u32_hex_edit(&mut self.salt_32[2]);
        ui.u32_hex_edit(&mut self.salt_32[3]);
    }

    fn salt_control_64(&mut self, ui: &mut egui::Ui) {
        ui.u64_hex_edit(&mut self.salt_64[0]);
        ui.u64_hex_edit(&mut self.salt_64[1]);
        ui.u64_hex_edit(&mut self.salt_64[2]);
        ui.u64_hex_edit(&mut self.salt_64[3]);
    }
}

impl HasherFrame for BlakeFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/hashers/src/blake",
        );
        ui.add_space(8.0);

        ui.byte_io_mode_hasher(&mut self.input_format, &mut self.output_format);

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
                    self.salt_control_32(ui);
                }
                BlakeVariant::B256 => {
                    ui.label("BLAKE-256 has a salt with four 32-bit words (256 bits).");
                    self.salt_control_32(ui);
                }
                BlakeVariant::B384 => {
                    ui.label("BLAKE-384 has a salt with four 64-bit words (512 bits).");
                    self.salt_control_64(ui);
                }
                BlakeVariant::B512 => {
                    ui.label("BLAKE-512 has a salt with four 64-bit words (512 bits).");
                    self.salt_control_64(ui);
                }
            };
        });

        // ui.label("<<<EXPLANATION OF HASH FUNCTION CODE>>>");

        ui.add_space(16.0);
    }

    fn hash_string(&self, text: &str) -> Result<String, HasherError> {
        if !self.valid_salt {
            return Err(hashers::errors::HasherError::key("invalid salt"));
        }

        let bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| hashers::errors::HasherError::general("byte format error"))?;

        let h = match self.variant {
            BlakeVariant::B224 => Blake224::init_mac(self.salt_32).hash(&bytes),
            BlakeVariant::B256 => Blake256::init_mac(self.salt_32).hash(&bytes),
            BlakeVariant::B384 => Blake384::init_mac(self.salt_64).hash(&bytes),
            BlakeVariant::B512 => Blake512::init_mac(self.salt_64).hash(&bytes),
        };

        Ok(self.output_format.byte_slice_to_text(&h))
    }
}
