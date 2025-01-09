use hashers::{
    errors::HasherError,
    sha::{sha2::Sha2Variant, Sha224, Sha256, Sha384, Sha512, Sha512_224, Sha512_256},
    traits::StatefulHasher,
};
use utils::byte_formatting::ByteFormat;

use crate::ui_elements::UiElements;

use super::HasherFrame;

pub struct Sha2Frame {
    input_format: ByteFormat,
    output_format: ByteFormat,
    variant: Sha2Variant,
}

impl Default for Sha2Frame {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            variant: Sha2Variant::Sha256,
        }
    }
}

impl Sha2Frame {}

impl HasherFrame for Sha2Frame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/hashers/src/sha/sha2.rs",
        );

        ui.add_space(8.0);
        ui.byte_io_mode_hasher(&mut self.input_format, &mut self.output_format);

        ui.add_space(16.0);
        ui.subheading("SHA-256 based");
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.variant, Sha2Variant::Sha256, "SHA-256");
            ui.selectable_value(&mut self.variant, Sha2Variant::Sha224, "SHA-224");
        });
        ui.add_space(8.0);
        ui.subheading("SHA-512 based");
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.variant, Sha2Variant::Sha512, "SHA-512");
            ui.selectable_value(&mut self.variant, Sha2Variant::Sha384, "SHA-384");
            ui.selectable_value(&mut self.variant, Sha2Variant::Sha512_224, "SHA-512/224");
            ui.selectable_value(&mut self.variant, Sha2Variant::Sha512_256, "SHA-512/256");
        });

        ui.add_space(16.0);
        ui.subheading("Discussion");
        match self.variant {
            Sha2Variant::Sha224 => ui.label("SHA-224 has the same compression function as SHA-256 but a different initialization vector and an output that is truncated to 224 bits (28 bytes) to make it resistant to length extension attacks."),
            Sha2Variant::Sha256 => ui.label("SHA-256 is the 256 bit (32 byte) version of SHA-2. It operates on a state of eight 32 bit words and performs sixty-four compression rounds on each chunk of the message."),
            Sha2Variant::Sha384 => ui.label("SHA-224 has the same compression function as SHA-512 but a different initialization vector and an output that is truncated to 384 bits (48 bytes) to make it resistant to length extension attacks."),
            Sha2Variant::Sha512 => ui.label("SHA-512 is the 512 bit (64 byte) version of SHA-2. It operates on a state of eight 64 bit words and performs eighty compression rounds on each chunk of the message."),
            Sha2Variant::Sha512_224 => ui.label("SHA-512/224 has the same compression function as SHA-512 but a different initialization vector and an output that is truncated to 224 bits (28 bytes) to make it resistant to length extension attacks. This truncation length makes it a drop in replacement for SHA-224 if needed."),
            Sha2Variant::Sha512_256 => ui.label("SHA-512/256 has the same compression function as SHA-512 but a different initialization vector and an output that is truncated to 256 bits (32 bytes) to make it resistant to length extension attacks. This truncation length makes it a drop in replacement for SHA-256 if needed."),
        };

        ui.add_space(16.0);
    }

    fn hash_string(&self, text: &str) -> Result<String, HasherError> {
        let bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| hashers::errors::HasherError::general("byte format error"))?;

        let h = match self.variant {
            Sha2Variant::Sha224 => Sha224::init().update_and_finalize(&bytes),
            Sha2Variant::Sha256 => Sha256::init().update_and_finalize(&bytes),
            Sha2Variant::Sha384 => Sha384::init().update_and_finalize(&bytes),
            Sha2Variant::Sha512 => Sha512::init().update_and_finalize(&bytes),
            Sha2Variant::Sha512_224 => Sha512_224::init().update_and_finalize(&bytes),
            Sha2Variant::Sha512_256 => Sha512_256::init().update_and_finalize(&bytes),
        };

        Ok(self.output_format.byte_slice_to_text(&h))
    }
}
