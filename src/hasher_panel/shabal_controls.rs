use super::HasherFrame;
use crate::ui_elements::UiElements;
use hashers::{
    shabal::{Shabal192, Shabal224, Shabal256, Shabal384, Shabal512, ShabalVariant},
    traits::StatefulHasher,
};
use utils::byte_formatting::ByteFormat;

pub struct ShabalFrame {
    input_format: ByteFormat,
    output_format: ByteFormat,
    variant: ShabalVariant,
}

impl Default for ShabalFrame {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            variant: ShabalVariant::Shabal256,
        }
    }
}

impl ShabalFrame {}

impl HasherFrame for ShabalFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/hashers/src/shabal.rs",
        );

        ui.byte_io_mode_hasher(&mut self.input_format, &mut self.output_format);

        ui.add_space(16.0);
        ui.subheading("Variants");
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.variant, ShabalVariant::Shabal192, "Shabal-192");
            ui.selectable_value(&mut self.variant, ShabalVariant::Shabal224, "Shabal-224");
            ui.selectable_value(&mut self.variant, ShabalVariant::Shabal256, "Shabal-256");
            ui.selectable_value(&mut self.variant, ShabalVariant::Shabal384, "Shabal-384");
            ui.selectable_value(&mut self.variant, ShabalVariant::Shabal512, "Shabal-512");
        });

        ui.add_space(16.0);
        ui.label("<<<EXPLANATION OF HASH FUNCTION CODE>>>");

        ui.add_space(16.0);
    }

    fn hash_string(&self, text: &str) -> Result<String, hashers::errors::HasherError> {
        let bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| hashers::errors::HasherError::general("byte format error"))?;

        let h = match self.variant {
            ShabalVariant::Shabal192 => Shabal192::init().hash(&bytes),
            ShabalVariant::Shabal224 => Shabal224::init().hash(&bytes),
            ShabalVariant::Shabal256 => Shabal256::init().hash(&bytes),
            ShabalVariant::Shabal384 => Shabal384::init().hash(&bytes),
            ShabalVariant::Shabal512 => Shabal512::init().hash(&bytes),
        };

        Ok(self.output_format.byte_slice_to_text(&h))
    }
}
