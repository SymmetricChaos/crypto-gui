use super::HasherFrame;
use crate::ui_elements::{validate_string_hex_bytes, UiElements};
use egui::DragValue;
use hashers::{
    blake::{Blake2b, Blake2s},
    errors::HasherError,
    traits::StatefulHasher,
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
    input_format: ByteFormat,
    output_format: ByteFormat,
    key: Vec<u8>,
    key_string: String,
    hash_len: u64,
}

impl Default for Blake2Frame {
    fn default() -> Self {
        Self {
            variant: Blake2Variant::Big,
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            key: Vec::new(),
            key_string: String::new(),
            hash_len: 32,
        }
    }
}

impl Blake2Frame {
    fn validate_key(&mut self, length: usize) {
        validate_string_hex_bytes(&mut self.key_string, Some(length));
        self.key = ByteFormat::Hex
            .text_to_bytes(&self.key_string)
            .expect("unable to parse key")
    }

    fn key_control(&mut self, ui: &mut egui::Ui, length: usize) {
        ui.horizontal(|ui| {
            if ui.control_string(&mut self.key_string).lost_focus() {
                self.validate_key(length);
            };
            if ui.button("🎲").on_hover_text("randomize").clicked() {
                let mut rng = thread_rng();
                self.key.clear();
                self.key.shrink_to(256);
                rng.fill_bytes(&mut self.key);
                self.key_string = ByteFormat::Hex.byte_slice_to_text(&self.key)
            };
        });
    }
}

impl HasherFrame for Blake2Frame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/hashers/src/blake",
        );
        ui.add_space(8.0);

        ui.byte_io_mode_hasher(&mut self.input_format, &mut self.output_format);

        ui.horizontal(|ui| {
            if ui
                .selectable_value(&mut self.variant, Blake2Variant::Big, "BLAKE2b")
                .clicked()
            {
                self.hash_len = self.hash_len.clamp(1, 64);
                self.validate_key(64);
            };
            if ui
                .selectable_value(&mut self.variant, Blake2Variant::BigLong, "BLAKE2b-Long")
                .clicked()
            {
                self.hash_len = self.hash_len.clamp(1, 1024);
                self.validate_key(64);
            };
            if ui
                .selectable_value(&mut self.variant, Blake2Variant::Small, "BLAKE2bs")
                .clicked()
            {
                self.hash_len = self.hash_len.clamp(1, 32);
                self.validate_key(32);
            }
        });
        ui.add_space(8.0);

        match self.variant {
            Blake2Variant::BigLong => ui.label("BLAKE2b-Long is designed for 64-bit hardware."),
            Blake2Variant::Big => ui.label("BLAKE2b is designed for 64-bit hardware."),
            Blake2Variant::Small => ui.label("BLAKE2s is designed for 32-bit hardware."),
        };

        ui.add_space(8.0);

        ui.subheading("Hash Length");
        ui.label("The BLAKE2 functions allow a variety of output lengths specified by how many bytes of the internal state are returned.");
        match self.variant {
            Blake2Variant::Big => {
                ui.label("BLAKE2b has a maximum output of 64 bytes (512 bits).");
                ui.add(DragValue::new(&mut self.hash_len).range(1..=64));
            }
            Blake2Variant::BigLong => {
                ui.label("BLAKE2b-Long has no maximum output length but here is limited to 1024 bytes (8192 bits, 1 kilobyte). For output lengths of 64 bytes or less it is identical to BLAKE2b.");
                ui.add(DragValue::new(&mut self.hash_len).range(1..=1024));
            }
            Blake2Variant::Small => {
                ui.label("BLAKE2s has a maximum output of 32 bytes (256 bits).");
                ui.add(DragValue::new(&mut self.hash_len).range(1..=32));
            }
        }

        ui.add_space(16.0);
        ui.subheading("Key (Hexadecimal)");
        ui.label("The BLAKE2 functions allow a key to be included to they function as a MAC.");
        match self.variant {
            Blake2Variant::Big => {
                ui.label("BLAKE2b has a maximum key size of of 64 bytes (512 bits).");
                self.key_control(ui, 64);
            }
            Blake2Variant::BigLong => {
                ui.label("BLAKE2b-Long has a maximum key size of of 64 bytes (512 bits).");
                self.key_control(ui, 64);
            }
            Blake2Variant::Small => {
                ui.label("BLAKE2s has a maximum key size of of 32 bytes (256 bits).");
                self.key_control(ui, 32);
            }
        };

        // ui.label("<<<EXPLANATION OF HASH FUNCTION CODE>>>");

        ui.add_space(16.0);
    }

    fn hash_string(&self, text: &str) -> Result<String, HasherError> {
        let bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| hashers::errors::HasherError::general("byte format error"))?;

        let h = match self.variant {
            Blake2Variant::Big => {
                Blake2b::init(&self.key, self.hash_len).update_and_finalize(&bytes)
            }
            Blake2Variant::Small => {
                Blake2s::init(&self.key, self.hash_len as u32).update_and_finalize(&bytes)
            }
            Blake2Variant::BigLong => {
                Blake2b::init(&self.key, self.hash_len).update_and_finalize(&bytes)
            }
        };

        Ok(self.output_format.byte_slice_to_text(&h))
    }
}
