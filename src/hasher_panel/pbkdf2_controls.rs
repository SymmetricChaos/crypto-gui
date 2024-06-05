use egui::DragValue;
use hashers::{errors::HasherError, hmac::SelectHmac, pbkdf::Pbkdf2, traits::ClassicHasher};

use crate::ui_elements::UiElements;

use super::HasherFrame;

pub struct Pbkdf2Frame {
    hasher: Pbkdf2,
}

impl Default for Pbkdf2Frame {
    fn default() -> Self {
        Self {
            hasher: Default::default(),
        }
    }
}

impl HasherFrame for Pbkdf2Frame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.byte_io_mode(
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
        ui.add(DragValue::new(&mut self.hasher.iterations).clamp_range(1..=32768));

        ui.subheading("Provide Salt");
        ui.label("<<<TODO>>>");

        ui.subheading("Output Length (Bytes)");
        ui.add(DragValue::new(&mut self.hasher.output_length).clamp_range(4..=512));
    }

    fn hash_bytes_from_string(&self, text: &str) -> Result<String, HasherError> {
        self.hasher.hash_bytes_from_string(text)
    }
}
