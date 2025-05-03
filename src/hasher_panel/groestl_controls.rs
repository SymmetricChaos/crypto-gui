use super::HasherFrame;
use crate::ui_elements::UiElements;
use hashers::{
    groestl::{Groestl1024, Groestl512},
    traits::StatefulHasher,
};
use utils::byte_formatting::ByteFormat;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GroestlVariant {
    L224,
    L256,
    L384,
    L512,
}

pub struct GroestlFrame {
    input_format: ByteFormat,
    output_format: ByteFormat,
    variant: GroestlVariant,
}

impl Default for GroestlFrame {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            variant: GroestlVariant::L256,
        }
    }
}

impl GroestlFrame {}

impl HasherFrame for GroestlFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/hashers/src/groestl.rs",
        );

        ui.byte_io_mode_hasher(&mut self.input_format, &mut self.output_format);

        ui.add_space(16.0);
        ui.subheading("Variants");
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.variant, GroestlVariant::L224, "Grøstl-224");
            ui.selectable_value(&mut self.variant, GroestlVariant::L256, "Grøstl-256");
            ui.selectable_value(&mut self.variant, GroestlVariant::L384, "Grøstl-384");
            ui.selectable_value(&mut self.variant, GroestlVariant::L512, "Grøstl-512");
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
            GroestlVariant::L224 => Groestl512::init224().hash(&bytes),
            GroestlVariant::L256 => Groestl512::init256().hash(&bytes),
            GroestlVariant::L384 => Groestl1024::init384().hash(&bytes),
            GroestlVariant::L512 => Groestl1024::init512().hash(&bytes),
        };

        Ok(self.output_format.byte_slice_to_text(&h))
    }
}
