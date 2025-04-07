use crate::ui_elements::UiElements;
use super::HasherFrame;
use hashers::{
    errors::HasherError,
    fxhash::{FxHash, FxHashVariant},
    traits::StatefulHasher,
};
use utils::byte_formatting::ByteFormat;

pub struct FxHashFrame {
    input_format: ByteFormat,
    output_format: ByteFormat,
    variant: FxHashVariant,
}

impl Default for FxHashFrame {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            variant: FxHashVariant::W64,
        }
    }
}

impl FxHashFrame {}

impl HasherFrame for FxHashFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/hashers/src/fxhash.rs",
        );
        ui.add_space(8.0);

        ui.byte_io_mode_hasher(&mut self.input_format, &mut self.output_format);
        ui.add_space(4.0);

        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.variant, FxHashVariant::W32, "32-bit");
            ui.selectable_value(&mut self.variant, FxHashVariant::W64, "64-bit");
        });

        ui.add_space(8.0);
        ui.label("<<<EXPLANATION OF HASH FUNCTION CODE>>>");

        ui.add_space(16.0);
    }

    fn hash_string(&self, text: &str) -> Result<String, HasherError> {
        let bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| hashers::errors::HasherError::general("byte format error"))?;

        let h = FxHash::init(self.variant).update_and_finalize(&bytes);

        Ok(self.output_format.byte_slice_to_text(&h))
    }
}
