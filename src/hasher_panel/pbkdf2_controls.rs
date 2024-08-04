use egui::DragValue;
use hashers::{errors::HasherError, hmac::SelectHmac, pbkdf::Pbkdf2, traits::ClassicHasher};
use utils::byte_formatting::ByteFormat;

use crate::ui_elements::UiElements;

use super::HasherFrame;

pub struct Pbkdf2Frame {
    hasher: Pbkdf2,
    salt: String,
    valid_salt: bool,
}

impl Default for Pbkdf2Frame {
    fn default() -> Self {
        Self {
            hasher: Default::default(),
            salt: String::from("BEEF"),
            valid_salt: true,
        }
    }
}

impl HasherFrame for Pbkdf2Frame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.byte_io_mode_hasher(
            &mut self.hasher.input_format,
            &mut self.hasher.output_format,
        );

        ui.subheading("Select HMAC");
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.hasher.hmac, SelectHmac::Md4, "MD4");
            ui.selectable_value(&mut self.hasher.hmac, SelectHmac::Md5, "MD5");
            ui.selectable_value(&mut self.hasher.hmac, SelectHmac::Sha1, "SHA1");
            ui.selectable_value(&mut self.hasher.hmac, SelectHmac::Sha224, "SHA2-224");
            ui.selectable_value(&mut self.hasher.hmac, SelectHmac::Sha256, "SHA2-256");
            ui.selectable_value(&mut self.hasher.hmac, SelectHmac::Sha384, "SHA2-384");
            ui.selectable_value(&mut self.hasher.hmac, SelectHmac::Sha512, "SHA2-512");
        });

        ui.subheading("Select Number of Iterations");
        ui.add(DragValue::new(&mut self.hasher.iterations).range(1..=32768));

        ui.horizontal(|ui| {
            ui.subheading("Provide Salt (Hexadecimal)");
            if !self.valid_salt {
                ui.error_text("invalid salt");
            }
        });
        if ui.control_string(&mut self.salt).changed() {
            if let Ok(bytes) = ByteFormat::Hex.text_to_bytes(&self.salt) {
                self.valid_salt = true;
                self.hasher.salt = bytes;
            } else {
                self.valid_salt = false;
                self.hasher.salt.clear();
            }
        }

        ui.subheading("Output Length (Bytes)");
        ui.add(DragValue::new(&mut self.hasher.output_length).range(4..=512));
    }

    fn hash_bytes_from_string(&self, text: &str) -> Result<String, HasherError> {
        self.hasher.hash_bytes_from_string(text)
    }
}
