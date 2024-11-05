use crate::ui_elements::UiElements;

use super::HasherFrame;
use egui::DragValue;
use hashers::scrypt::Scrypt;

pub struct ScryptFrame {
    hasher: Scrypt,
    salt: Vec<u8>,
    salt_string: String,
}

impl Default for ScryptFrame {
    fn default() -> Self {
        Self {
            hasher: Default::default(),
            salt: Vec::new(),
            salt_string: String::new(),
        }
    }
}

impl ScryptFrame {}

impl HasherFrame for ScryptFrame {
    fn ui(&mut self, ui: &mut egui::Ui, errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/hashers/src/scrypt",
        );

        ui.byte_io_mode_hasher(
            &mut self.hasher.input_format,
            &mut self.hasher.output_format,
        );
        ui.add_space(16.0);

        ui.subheading("Cost Factor");
        ui.add(DragValue::new(&mut self.hasher.cost));
        ui.subheading("Block Size Factor");
        ui.add(DragValue::new(&mut self.hasher.blocksize_factor));
        ui.subheading("Parallelism");
        ui.add(DragValue::new(&mut self.hasher.parallelism));
        ui.subheading("Output Length");
        ui.add(DragValue::new(&mut self.hasher.key_len));

        todo!()
    }

    crate::hash_string! {}
}
