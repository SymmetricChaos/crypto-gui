use super::HasherFrame;
use crate::ui_elements::UiElements;
use hashers::{mgf1::Mgf1, sha::sha2::Sha2Variant, traits::StatefulHasher};
use utils::byte_formatting::ByteFormat;

pub struct Mgf1Frame {
    input_format: ByteFormat,
    output_format: ByteFormat,
    variant: Sha2Variant,
    hash_len: u32,
}

impl Default for Mgf1Frame {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            variant: Sha2Variant::Sha256,
            hash_len: 64,
        }
    }
}

impl Mgf1Frame {}

impl HasherFrame for Mgf1Frame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/hashers/src/mgdf1.rs",
        );

        ui.byte_io_mode_hasher(&mut self.input_format, &mut self.output_format);

        ui.add_space(16.0);
        ui.subheading("Hash Function");
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.variant, Sha2Variant::Sha256, "SHA-256");
            ui.selectable_value(&mut self.variant, Sha2Variant::Sha224, "SHA-224");
        });
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.variant, Sha2Variant::Sha512, "SHA-512");
            ui.selectable_value(&mut self.variant, Sha2Variant::Sha384, "SHA-384");
            ui.selectable_value(&mut self.variant, Sha2Variant::Sha512_224, "SHA-512/224");
            ui.selectable_value(&mut self.variant, Sha2Variant::Sha512_256, "SHA-512/256");
        });

        ui.subheading("Output Length (Bytes)");
        ui.label("While MGF1 can output several gigabytes it is limited to 1024 bytes here.");
        ui.add(egui::DragValue::new(&mut self.hash_len).range(1..=1024));

        ui.add_space(16.0);
    }

    fn hash_string(&self, text: &str) -> Result<String, hashers::errors::HasherError> {
        let bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| hashers::errors::HasherError::general("byte format error"))?;
        Ok(self
            .output_format
            .byte_slice_to_text(Mgf1::init(self.hash_len, self.variant).update_and_finalize(&bytes)))
    }
}
