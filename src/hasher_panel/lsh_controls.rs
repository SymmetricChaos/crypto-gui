use super::HasherFrame;
use crate::ui_elements::UiElements;
use hashers::{
    lsh::{lsh256::Lsh256, lsh512::Lsh512},
    traits::StatefulHasher,
};
use utils::byte_formatting::ByteFormat;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Variant {
    LSH256_224,
    LSH256_256,
    LSH512_224,
    LSH512_256,
    LSH512_384,
    LSH512_512,
}

pub struct LshFrame {
    variant: Variant,
    input_format: ByteFormat,
    output_format: ByteFormat,
}

impl Default for LshFrame {
    fn default() -> Self {
        Self {
            variant: Variant::LSH256_256,
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
        }
    }
}

impl HasherFrame for LshFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/hashers/src/lsh.rs",
        );
        ui.add_space(8.0);

        ui.byte_io_mode_hasher(&mut self.input_format, &mut self.output_format);

        ui.add_space(8.0);

        ui.subheading("LSH-256");
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.variant, Variant::LSH256_224, "LSH-256-224");
            ui.selectable_value(&mut self.variant, Variant::LSH256_256, "LSH-256-256");
        });
        ui.add_space(4.0);
        ui.subheading("LSH-512");
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.variant, Variant::LSH512_224, "LSH-512-224");
            ui.selectable_value(&mut self.variant, Variant::LSH512_256, "LSH-512-256");
            ui.selectable_value(&mut self.variant, Variant::LSH512_384, "LSH-512-384");
            ui.selectable_value(&mut self.variant, Variant::LSH512_512, "LSH-512-512");
        });
        ui.add_space(16.0);
    }

    fn hash_string(&self, text: &str) -> Result<String, hashers::errors::HasherError> {
        let bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| hashers::errors::HasherError::general("byte format error"))?;
        Ok(self.output_format.byte_slice_to_text(match self.variant {
            Variant::LSH256_224 => Lsh256::init_224().hash(&bytes),
            Variant::LSH256_256 => Lsh256::init_256().hash(&bytes),
            Variant::LSH512_224 => Lsh512::init_224().hash(&bytes),
            Variant::LSH512_256 => Lsh512::init_256().hash(&bytes),
            Variant::LSH512_384 => Lsh512::init_384().hash(&bytes),
            Variant::LSH512_512 => Lsh512::init_512().hash(&bytes),
        }))
    }
}
