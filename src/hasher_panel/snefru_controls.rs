use super::HasherFrame;
use crate::ui_elements::UiElements;
use egui::DragValue;
use hashers::{
    snefru::{Snefru, SnefruOutputSize},
    traits::StatefulHasher,
};
use utils::byte_formatting::ByteFormat;

pub struct SnefruFrame {
    input_format: ByteFormat,
    output_format: ByteFormat,
    security_level: u32,
    variant: SnefruOutputSize,
}

impl Default for SnefruFrame {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            security_level: 8,
            variant: SnefruOutputSize::W128,
        }
    }
}

impl HasherFrame for SnefruFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/hashers/src/snefru.rs",
        );

        ui.byte_io_mode_hasher(&mut self.input_format, &mut self.output_format);
        ui.add_space(16.0);

        ui.subheading("Security Level");
        ui.add(DragValue::new(&mut self.security_level).range(2..=16));
        ui.add_space(16.0);

        ui.subheading("Output Size");
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.variant, SnefruOutputSize::W128, "Snefru-128");
            ui.selectable_value(&mut self.variant, SnefruOutputSize::W256, "Snefru-256");
        });
        ui.add_space(16.0);
    }

    fn hash_string(&self, text: &str) -> Result<String, hashers::errors::HasherError> {
        let bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| hashers::errors::HasherError::general("byte format error"))?;
        Ok(self.output_format.byte_slice_to_text(
            Snefru::init(self.security_level, self.variant).update_and_finalize(&bytes),
        ))
    }
}
