use super::HasherFrame;
use crate::ui_elements::UiElements;
use hashers::{
    ripemd::{
        ripemd0::RipeMd0, ripemd128::RipeMd128, ripemd160::RipeMd160, ripemd256::RipeMd256,
        ripemd320::RipeMd320, RipeMdVariant,
    },
    traits::StatefulHasher,
};
use strum::IntoEnumIterator;
use utils::byte_formatting::ByteFormat;

pub struct RipeMdFrame {
    input_format: ByteFormat,
    output_format: ByteFormat,
    variant: RipeMdVariant,
}

impl Default for RipeMdFrame {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            variant: RipeMdVariant::Md256,
        }
    }
}

impl RipeMdFrame {}

impl HasherFrame for RipeMdFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/hashers/src/ripemd",
        );

        ui.byte_io_mode_hasher(&mut self.input_format, &mut self.output_format);
        ui.add_space(16.0);

        ui.subheading("RIPEMD Variants");
        ui.horizontal(|ui| {
            for variant in RipeMdVariant::iter() {
                ui.selectable_value(&mut self.variant, variant, variant.to_string());
            }
        });
    }

    fn hash_string(&self, text: &str) -> Result<String, hashers::errors::HasherError> {
        let bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| hashers::errors::HasherError::general("byte format error"))?;

        let h = match self.variant {
            RipeMdVariant::Md0 => RipeMd0::init().update_and_finalize(&bytes),
            RipeMdVariant::Md128 => RipeMd128::init().update_and_finalize(&bytes),
            RipeMdVariant::Md160 => RipeMd160::init().update_and_finalize(&bytes),
            RipeMdVariant::Md256 => RipeMd256::init().update_and_finalize(&bytes),
            RipeMdVariant::Md320 => RipeMd320::init().update_and_finalize(&bytes),
        };

        Ok(self.output_format.byte_slice_to_text(&h))
    }
}
