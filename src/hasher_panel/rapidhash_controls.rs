use super::HasherFrame;
use crate::ui_elements::UiElements;
use hashers::{
    rapidhash::{
        rapidhash::RapidHashV3, rapidhash_micro::RapidHashMicroV3, rapidhash_nano::RapidHashNanoV3,
    },
    traits::StatefulHasher,
};
use utils::byte_formatting::ByteFormat;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Variant {
    RapidHash,
    Micro,
    Nano,
}

pub struct RapidhashFrame {
    input_format: ByteFormat,
    output_format: ByteFormat,
    variant: Variant,
    seed: u64,
}

impl Default for RapidhashFrame {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            variant: Variant::RapidHash,
            seed: 0,
        }
    }
}

impl HasherFrame for RapidhashFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/hashers/src/rapidhash",
        );
        ui.add_space(8.0);

        ui.byte_io_mode_hasher(&mut self.input_format, &mut self.output_format);
        ui.add_space(16.0);

        ui.subheading("Seed");
        ui.label("Rapidhash accepts a 64-bit seed value to provide resistance to hash flooding.");
        ui.u64_hex_edit(&mut self.seed);
        ui.add_space(8.0);

        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.variant, Variant::RapidHash, "Rapidhash");
            ui.selectable_value(&mut self.variant, Variant::Micro, "RapidhashMicro");
            ui.selectable_value(&mut self.variant, Variant::Nano, "RapidhashNano");
        });
        ui.add_space(8.0);

        match self.variant {
            Variant::RapidHash => {
                ui.label("The general purpose version of Rapidhash uses a 112-byte block size in order to process long inputs quickly.");
            }
            Variant::Micro => {
                ui.label("RapidhashMicro uses a 80-byte block size is order to reduce the number of instructions in the compiled code while still being able to handle longer inputs at a good speed. It is described as being aimed at high performance computing.");
            }
            Variant::Nano => {
                ui.label("RapidhashNano uses a 48-byte block size is order to reduce the number of instructions in the compiled code even more than RapidhashMicro. The small block size makes large inputs slow to hash.");
            }
        };
        ui.label("All variants of Rapidhash produce identical outputs for inputs of 16 bytes or less because they shortcut to the same simple hashing scheme.");
        ui.add_space(16.0);
    }

    fn hash_string(&self, text: &str) -> Result<String, hashers::errors::HasherError> {
        let bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| hashers::errors::HasherError::general("byte format error"))?;

        let h = match self.variant {
            Variant::RapidHash => RapidHashV3::with_seed(self.seed).hash(&bytes),
            Variant::Micro => RapidHashMicroV3::with_seed(self.seed).hash(&bytes),
            Variant::Nano => RapidHashNanoV3::with_seed(self.seed).hash(&bytes),
        };

        Ok(self.output_format.byte_slice_to_text(&h))
    }
}
