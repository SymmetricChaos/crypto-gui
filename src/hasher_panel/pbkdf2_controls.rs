use egui::DragValue;
use hashers::{hmac::HmacVariant, pbkdf2::Pbkdf2};
use strum::IntoEnumIterator;
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
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/hashers/src/pbkdf2.rs",
        );

        ui.byte_io_mode_hasher(
            &mut self.hasher.input_format,
            &mut self.hasher.output_format,
        );

        ui.subheading("Select Inner HMAC");
        ui.horizontal(|ui| {
            for variant in HmacVariant::iter() {
                ui.selectable_value(&mut self.hasher.variant, variant, variant.to_string());
            }
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
        ui.add(DragValue::new(&mut self.hasher.hash_len).range(4..=512));
    }
    crate::hash_string! {}
}
