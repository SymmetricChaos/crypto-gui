use super::HasherFrame;
use crate::ui_elements::{validate_string_hex_bytes, UiElements};
use egui::DragValue;
use hashers::{scrypt::Scrypt, traits::StatefulHasher};
use rand::{thread_rng, RngCore};
use utils::byte_formatting::ByteFormat;

pub struct ScryptFrame {
    input_format: ByteFormat,
    output_format: ByteFormat,
    salt: Vec<u8>,
    salt_string: String,
    cost: u32,
    blocksize_factor: u32,
    parallelism: u32,
    key_len: u32,
}

impl Default for ScryptFrame {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            salt: Vec::new(),
            salt_string: String::new(),
            cost: 0,
            blocksize_factor: 0,
            parallelism: 0,
            key_len: 0,
        }
    }
}

impl ScryptFrame {
    fn validate_salt(&mut self) {
        validate_string_hex_bytes(&mut self.salt_string, None);
        self.salt = ByteFormat::Hex
            .text_to_bytes(&self.salt_string)
            .expect("unable to parse salt input");
    }

    fn salt_control(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.control_string(&mut self.salt_string).lost_focus() {
                self.validate_salt();
            };
            if ui.button("🎲").on_hover_text("randomize").clicked() {
                let mut rng = thread_rng();
                self.salt = vec![0; 32];
                rng.fill_bytes(&mut self.salt);
                self.salt_string = ByteFormat::Hex.byte_slice_to_text(&self.salt)
            };
        });
    }
}

impl HasherFrame for ScryptFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/hashers/src/scrypt",
        );
        ui.add_space(8.0);

        ui.byte_io_mode_hasher(&mut self.input_format, &mut self.output_format);
        ui.add_space(8.0);

        ui.subheading("Output Length (bytes)");
        ui.add(DragValue::new(&mut self.key_len));
        ui.add_space(8.0);
        ui.subheading("Cost Factor");
        ui.label("Number of iterations performed.");
        ui.add(DragValue::new(&mut self.cost));
        ui.add_space(4.0);
        ui.subheading("Block Size Factor");
        ui.label("Amount of memory needed.");
        ui.add(DragValue::new(&mut self.blocksize_factor));
        ui.add_space(4.0);
        ui.subheading("Parallelism");
        ui.label("Nnumber of independent threads that can be used.");
        ui.add(DragValue::new(&mut self.parallelism));
        ui.add_space(8.0);

        ui.subheading("Salt");
        ui.label("Arbitray additional data incorporated into the function.");
        self.salt_control(ui);
    }

    fn hash_string(&self, text: &str) -> Result<String, hashers::errors::HasherError> {
        let bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| hashers::errors::HasherError::general("byte format error"))?;

        let h = Scrypt::init(
            self.cost,
            self.blocksize_factor,
            self.parallelism,
            self.key_len,
            &self.salt,
        )
        .hash(&bytes);

        Ok(self.output_format.byte_slice_to_text(&h))
    }
}
