use super::HasherFrame;
use crate::ui_elements::UiElements;
use egui::DragValue;
use hashers::pbkdf1::{Pbkdf1, Pbkdf1Variant};
use strum::IntoEnumIterator;
use utils::byte_formatting::ByteFormat;

pub struct Pbkdf1Frame {
    hasher: Pbkdf1,
    salt: String,
    valid_salt: bool,
}

impl Default for Pbkdf1Frame {
    fn default() -> Self {
        Self {
            hasher: Default::default(),
            salt: String::from("DEADBEEF"),
            valid_salt: true,
        }
    }
}

impl HasherFrame for Pbkdf1Frame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.byte_io_mode_hasher(
            &mut self.hasher.input_format,
            &mut self.hasher.output_format,
        );

        ui.subheading("Select Inner Hasher");
        ui.horizontal(|ui| {
            for variant in Pbkdf1Variant::iter() {
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
            match self.hasher.set_salt_from_str(ByteFormat::Hex, &self.salt) {
                Ok(_) => self.valid_salt = true,
                Err(_) => {
                    self.valid_salt = false;
                    self.hasher.salt = [0; 8];
                }
            }
        }

        ui.subheading("Output Length (Bytes)");
        ui.add(DragValue::new(&mut self.hasher.hash_len).range(4..=512));
    }

    crate::hash_string! {}
}
