use crate::ui_elements::{validate_string_hex_bytes, UiElements};

use super::HasherFrame;
use hashers::{
    errors::HasherError,
    hmac::{Hmac, HmacVariant},
    traits::StatefulHasher,
};
use rand::{thread_rng, RngCore};
use utils::byte_formatting::ByteFormat;

pub struct HmacFrame {
    input_format: ByteFormat,
    output_format: ByteFormat,
    variant: HmacVariant,
    key: Vec<u8>,
    key_string: String,
}

impl Default for HmacFrame {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            variant: HmacVariant::Sha256,
            key: Vec::new(),
            key_string: String::new(),
        }
    }
}

impl HmacFrame {
    fn validate_key(&mut self) {
        validate_string_hex_bytes(&mut self.key_string, None);
        self.key = ByteFormat::Hex
            .text_to_bytes(&self.key_string)
            .expect("unable to parse key")
    }

    fn key_control(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.control_string(&mut self.key_string).lost_focus() {
                self.validate_key();
            };
            if ui.button("🎲").on_hover_text("randomize").clicked() {
                let mut rng = thread_rng();
                self.key = vec![0; 32];
                rng.fill_bytes(&mut self.key);
                self.key_string = ByteFormat::Hex.byte_slice_to_text(&self.key)
            };
        });
    }
}

impl HasherFrame for HmacFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/hashers/src/hmac.rs",
        );
        ui.add_space(8.0);

        ui.byte_io_mode_hasher(&mut self.input_format, &mut self.output_format);
        ui.add_space(16.0);

        ui.subheading("Algorithm");
        ui.label("HMAC accepts a hash function, a key, and a message. In the case that the key is larger than the block size of the hasher it is hashed and that hash is used as the key instead.\n1) Each byte of the key is XORed with the padding byte 0x5c and the padding bytes continue up to the block size of the hasher.\n2) The message is appended to the key and that entire sequence of bytes is hashed.\n3) Each byte of the key is XORed with the padding byte 0x36 and the padding bytes continue up to the block size of the hasher.\n4) The previously hashed result is appended to this padded key and that entire sequence of bytes is hashed.\nHMAC = H((key ⊕ outer_pad) || H( (key ⊕ inner_pad) || message )))");

        ui.subheading("Discontinued Hashers");
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.variant, HmacVariant::Md4, "MD4");
            ui.selectable_value(&mut self.variant, HmacVariant::Md5, "MD5");
        });
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.variant, HmacVariant::Sha0, "SHA0");
            ui.selectable_value(&mut self.variant, HmacVariant::Sha1, "SHA1");
        });

        ui.add_space(8.0);
        ui.subheading("NIST Approved Hashers");
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.variant, HmacVariant::Sha224, "SHA224");
            ui.selectable_value(&mut self.variant, HmacVariant::Sha256, "SHA256");
            ui.selectable_value(&mut self.variant, HmacVariant::Sha384, "SHA384");
            ui.selectable_value(&mut self.variant, HmacVariant::Sha512, "SHA512");
        });
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.variant, HmacVariant::Sha512_224, "SHA512-224");
            ui.selectable_value(&mut self.variant, HmacVariant::Sha512_256, "SHA512-256");
        });
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.variant, HmacVariant::Sha3_224, "SHA3-224");
            ui.selectable_value(&mut self.variant, HmacVariant::Sha3_256, "SHA3-256");
            ui.selectable_value(&mut self.variant, HmacVariant::Sha3_384, "SHA3-384");
            ui.selectable_value(&mut self.variant, HmacVariant::Sha3_512, "SHA3-512");
        });

        ui.add_space(8.0);
        self.key_control(ui);

        ui.add_space(16.0);
    }

    fn hash_string(&self, text: &str) -> Result<String, HasherError> {
        let bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| hashers::errors::HasherError::general("byte format error"))?;

        let h = Hmac::init(self.variant, &self.key).update_and_finalize(&bytes);

        Ok(self.output_format.byte_slice_to_text(&h))
    }
}
