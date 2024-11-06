use crate::ui_elements::UiElements;

use super::HasherFrame;
use egui::DragValue;
use hashers::scrypt::Scrypt;
use strum::IntoEnumIterator;
use utils::byte_formatting::ByteFormat;

pub struct ScryptFrame {
    hasher: Scrypt,
    salt: Vec<u8>,
    salt_string: String,
    salt_format: ByteFormat,
    salt_valid: bool,
}

impl Default for ScryptFrame {
    fn default() -> Self {
        Self {
            hasher: Default::default(),
            salt: Vec::new(),
            salt_string: String::new(),
            salt_format: ByteFormat::Utf8,
            salt_valid: true,
        }
    }
}

impl ScryptFrame {
    fn set_salt(&mut self) {
        if let Ok(salt) = self.salt_format.text_to_bytes(&self.salt_string) {
            self.salt = salt;
            self.salt_valid = true;
        } else {
            self.salt.clear();
            self.salt_valid = false;
        };
    }
}

impl HasherFrame for ScryptFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/hashers/src/scrypt",
        );
        ui.add_space(8.0);

        ui.byte_io_mode_hasher(
            &mut self.hasher.input_format,
            &mut self.hasher.output_format,
        );
        ui.add_space(8.0);

        ui.subheading("Output Length (bytes)");
        ui.add(DragValue::new(&mut self.hasher.key_len));
        ui.add_space(8.0);

        ui.subheading("Cost Factor");
        ui.label("Adjust the number of iterations performed.");
        ui.add(DragValue::new(&mut self.hasher.cost));
        ui.add_space(4.0);
        ui.subheading("Block Size Factor");
        ui.label("Adjust the amount of memory needed.");
        ui.add(DragValue::new(&mut self.hasher.blocksize_factor));
        ui.add_space(4.0);
        ui.subheading("Parallelism");
        ui.label("Adjust the number of independent threads that can be used.");
        ui.add(DragValue::new(&mut self.hasher.parallelism));
        ui.add_space(8.0);

        ui.subheading("Salt");
        ui.label("Arbitray additional data incorporated into the function.");
        ui.horizontal(|ui| {
            for variant in ByteFormat::iter() {
                if ui
                    .selectable_value(&mut self.salt_format, variant, variant.to_string())
                    .changed()
                {
                    self.set_salt();
                }
            }
        });
        if ui.control_string(&mut self.salt_string).changed() {
            self.set_salt();
        }
        if !self.salt_valid {
            ui.error_text("SALT INVALID, CURRENTLY SET TO EMPTY");
        }
    }

    crate::hash_string! {}
}
