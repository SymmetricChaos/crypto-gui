use super::HasherFrame;
use crate::ui_elements::UiElements;
use hashers::{
    skein::{skein1024::Skein1024, skein256::Skein256, skein512::Skein512},
    traits::StatefulHasher,
};
use utils::byte_formatting::ByteFormat;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SkeinVariant {
    S256_128,
    S256_160,
    S256_224,
    S256_256,
    S512_128,
    S512_160,
    S512_224,
    S512_256,
    S512_384,
    S512_512,
    S1024_384,
    S1024_512,
    S1024_1024,
}

pub struct SkeinFrame {
    input_format: ByteFormat,
    output_format: ByteFormat,
    variant: SkeinVariant,
}

impl Default for SkeinFrame {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            variant: SkeinVariant::S256_256,
        }
    }
}

impl HasherFrame for SkeinFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/hashers/src/skein",
        );

        ui.byte_io_mode_hasher(&mut self.input_format, &mut self.output_format);

        ui.add_space(16.0);
        ui.subheading("Variants");
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.variant, SkeinVariant::S256_128, "Skein-256-128");
            ui.selectable_value(&mut self.variant, SkeinVariant::S256_160, "Skein-256-160");
            ui.selectable_value(&mut self.variant, SkeinVariant::S256_224, "Skein-256-224");
            ui.selectable_value(&mut self.variant, SkeinVariant::S256_256, "Skein-256-256");
        });
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.variant, SkeinVariant::S512_128, "Skein-512-128");
            ui.selectable_value(&mut self.variant, SkeinVariant::S512_160, "Skein-512-160");
            ui.selectable_value(&mut self.variant, SkeinVariant::S512_224, "Skein-512-224");
            ui.selectable_value(&mut self.variant, SkeinVariant::S512_256, "Skein-512-256");
            ui.selectable_value(&mut self.variant, SkeinVariant::S512_384, "Skein-512-384");
            ui.selectable_value(&mut self.variant, SkeinVariant::S512_512, "Skein-512-512");
        });
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.variant, SkeinVariant::S1024_384, "Skein-1024-384");
            ui.selectable_value(&mut self.variant, SkeinVariant::S1024_512, "Skein-1024-512");
            ui.selectable_value(
                &mut self.variant,
                SkeinVariant::S1024_1024,
                "Skein-1024-1024",
            );
        });
    }

    fn hash_string(&self, text: &str) -> Result<String, hashers::errors::HasherError> {
        let bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| hashers::errors::HasherError::general("byte format error"))?;

        let h = match self.variant {
            SkeinVariant::S256_128 => Skein256::init_128().hash(&bytes),
            SkeinVariant::S256_160 => Skein256::init_160().hash(&bytes),
            SkeinVariant::S256_224 => Skein256::init_224().hash(&bytes),
            SkeinVariant::S256_256 => Skein256::init_256().hash(&bytes),
            SkeinVariant::S512_128 => Skein512::init_128().hash(&bytes),
            SkeinVariant::S512_160 => Skein512::init_160().hash(&bytes),
            SkeinVariant::S512_224 => Skein512::init_224().hash(&bytes),
            SkeinVariant::S512_256 => Skein512::init_256().hash(&bytes),
            SkeinVariant::S512_384 => Skein512::init_384().hash(&bytes),
            SkeinVariant::S512_512 => Skein512::init_512().hash(&bytes),
            SkeinVariant::S1024_384 => Skein1024::init_384().hash(&bytes),
            SkeinVariant::S1024_512 => Skein1024::init_512().hash(&bytes),
            SkeinVariant::S1024_1024 => Skein1024::init_1024().hash(&bytes),
        };

        Ok(self.output_format.byte_slice_to_text(&h))
    }
}
