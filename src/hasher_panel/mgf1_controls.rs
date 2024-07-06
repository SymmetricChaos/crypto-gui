use crate::ui_elements::UiElements;

use super::HasherFrame;
use hashers::{errors::HasherError, mgf1::Mgf1, sha2::Sha2Variant, traits::ClassicHasher};

pub struct Mgf1Frame {
    hasher: Mgf1,
}

impl Default for Mgf1Frame {
    fn default() -> Self {
        Self {
            hasher: Default::default(),
        }
    }
}

impl Mgf1Frame {}

impl HasherFrame for Mgf1Frame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.add_space(16.0);

        ui.byte_io_mode_hasher(
            &mut self.hasher.input_format,
            &mut self.hasher.output_format,
        );

        ui.add_space(16.0);
        ui.subheading("Hash Function");
        ui.horizontal(|ui| {
            ui.selectable_value(
                &mut self.hasher.hasher.variant,
                Sha2Variant::Sha256,
                "SHA-256",
            );
            ui.selectable_value(
                &mut self.hasher.hasher.variant,
                Sha2Variant::Sha224,
                "SHA-224",
            );
        });
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            ui.selectable_value(
                &mut self.hasher.hasher.variant,
                Sha2Variant::Sha512,
                "SHA-512",
            );
            ui.selectable_value(
                &mut self.hasher.hasher.variant,
                Sha2Variant::Sha384,
                "SHA-384",
            );
            ui.selectable_value(
                &mut self.hasher.hasher.variant,
                Sha2Variant::Sha512_224,
                "SHA-512/224",
            );
            ui.selectable_value(
                &mut self.hasher.hasher.variant,
                Sha2Variant::Sha512_256,
                "SHA-512/256",
            );
        });

        ui.subheading("Output Length (number of bytes)");
        ui.label("While MGF1 can output several gigabytes its limited 1024 bytes here.");
        ui.add(egui::DragValue::new(&mut self.hasher.output_length).clamp_range(1..=1024));

        ui.add_space(16.0);

        ui.add_space(16.0);
    }

    fn hash_bytes_from_string(&self, text: &str) -> Result<String, HasherError> {
        self.hasher.hash_bytes_from_string(text)
    }
}
