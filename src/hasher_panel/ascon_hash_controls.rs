use hashers::{
    ascon::{hash::Ascon, Variant},
    errors::HasherError,
};
use strum::IntoEnumIterator;
use utils::byte_formatting::ByteFormat;

use super::HasherFrame;
use crate::ui_elements::UiElements;

pub struct AsconHashFrame {
    input_format: ByteFormat,
    output_format: ByteFormat,
    variant: Variant,
    hash_len: usize,
}

impl Default for AsconHashFrame {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            variant: Variant::Hash,
            hash_len: 32,
        }
    }
}

impl HasherFrame for AsconHashFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/hashers/src/ascon/hash.rs",
        );
        ui.add_space(8.0);

        ui.byte_io_mode_hasher(&mut self.input_format, &mut self.output_format);
        ui.add_space(4.0);

        for variant in Variant::iter() {
            ui.selectable_value(&mut self.variant, variant, variant.to_string());
        }
        ui.add_space(4.0);

        ui.subheading("Hash Length");
        match self.variant {
            Variant::Hash => {
                ui.label("Ascon-Hash can return a hash of any length from 16 bytes to 32 bytes (128 bits to 256 bits). There are 12 rounds for all steps.");
                ui.add(egui::DragValue::new(&mut self.hash_len).range(16..=32));
            }
            Variant::Hasha => {
                ui.label("Ascon-Hasha can return a hash of any length from 16 bytes to 32 bytes (128 bits to 256 bits). There are 12 initialization round and 8 rounds for all other steps.");
                ui.add(egui::DragValue::new(&mut self.hash_len).range(16..=32));
            }
            Variant::Xof => {
                ui.label("Ascon-XOF can return an output of any length but here is limited to 1024 bytes (4096 bits). There are 12 rounds for all steps.");
                ui.add(egui::DragValue::new(&mut self.hash_len).range(1..=1024));
            }
            Variant::Xofa => {
                ui.label("Ascon-XOFa can return an output of any length but here is limited to 1024 bytes (4096 bits). There are 12 initialization round and 8 rounds for all other steps.");
                ui.add(egui::DragValue::new(&mut self.hash_len).range(1..=1024));
            }
            _ => {
                ui.error_text("VARIANT NOT COVERED");
            }
        }

        ui.add_space(16.0);
    }

    fn hash_string(&self, text: &str) -> Result<String, HasherError> {
        let bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| hashers::errors::HasherError::general("byte format error"))?;

        let h = match self.variant {
            Variant::Hash => Ascon::hash(&bytes),
            Variant::Hasha => Ascon::hasha(&bytes),
            Variant::Xof => Ascon::xof(&bytes, self.hash_len),
            Variant::Xofa => Ascon::xofa(&bytes, self.hash_len),
            _ => panic!("VARIANT NOT COVERED"),
        };

        Ok(self.output_format.byte_slice_to_text(&h))
    }
}
