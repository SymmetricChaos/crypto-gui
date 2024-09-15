use hashers::ascon::ascon_hash::{AsconHash, Variant};
use strum::IntoEnumIterator;

use super::HasherFrame;
use crate::ui_elements::UiElements;

pub struct AsconFrame {
    hasher: AsconHash,
}

impl Default for AsconFrame {
    fn default() -> Self {
        Self {
            hasher: AsconHash::default(),
        }
    }
}

impl HasherFrame for AsconFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.add_space(16.0);

        ui.byte_io_mode_hasher(
            &mut self.hasher.input_format,
            &mut self.hasher.output_format,
        );
        ui.add_space(4.0);

        for variant in Variant::iter() {
            ui.selectable_value(&mut self.hasher.variant, variant, variant.to_string());
        }
        ui.add_space(4.0);

        ui.subheading("Hash Length");
        match self.hasher.variant {
            Variant::AsconHash => {
                ui.label("Ascon-Hash can return a hash of any length from 16 bytes to 32 bytes (128 bits to 256 bits). There are 12 rounds for all steps.");
                ui.add(egui::DragValue::new(&mut self.hasher.hash_len).range(16..=32));
            }
            Variant::AsconHasha => {
                ui.label("Ascon-Hasha can return a hash of any length from 16 bytes to 32 bytes (128 bits to 256 bits). There are 12 initialization round and 8 rounds for all other steps.");
                ui.add(egui::DragValue::new(&mut self.hasher.hash_len).range(16..=32));
            }
            Variant::AsconXof => {
                ui.label("Ascon-XOF can return an output of any length but here is limited to 512 bytes (4096 bits). There are 12 rounds for all steps.");
                ui.add(egui::DragValue::new(&mut self.hasher.hash_len).range(1..=512));
            }
            Variant::AsconXofa => {
                ui.label("Ascon-XOFa can return an output of any length but here is limited to 512 bytes (4096 bits). There are 12 initialization round and 8 rounds for all other steps.");
                ui.add(egui::DragValue::new(&mut self.hasher.hash_len).range(1..=512));
            }
        }

        ui.add_space(16.0);
    }

    crate::hash_string! {}
}
