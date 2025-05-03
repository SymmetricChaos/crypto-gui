use super::HasherFrame;
use crate::ui_elements::{validate_string_hex_bytes, UiElements};
use egui::DragValue;
use hashers::{hmac::HmacVariant, pbkdf2::Pbkdf2, traits::StatefulHasher};
use rand::{thread_rng, RngCore};
use utils::byte_formatting::ByteFormat;

pub struct Pbkdf2Frame {
    input_format: ByteFormat,
    output_format: ByteFormat,
    variant: HmacVariant,
    salt_string: String,
    salt: Vec<u8>,
    iterations: u32,
    hash_len: u32,
}

impl Default for Pbkdf2Frame {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            variant: HmacVariant::Sha256,
            salt_string: String::from("BEEF"),
            salt: b"BEEF".to_vec(),
            iterations: 4096,
            hash_len: 64,
        }
    }
}

impl Pbkdf2Frame {
    fn validate_salt(&mut self) {
        validate_string_hex_bytes(&mut self.salt_string, None);
        self.salt = ByteFormat::Hex
            .text_to_bytes(&self.salt_string)
            .expect("unable to parse salt input");
    }

    fn salt_control(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.control_string(&mut self.salt_string).lost_focus() {
                self.validate_salt();
            };
            if ui.button("🎲").on_hover_text("randomize").clicked() {
                let mut rng = thread_rng();
                self.salt = vec![0; 32];
                rng.fill_bytes(&mut self.salt);
                self.salt_string = ByteFormat::Hex.byte_slice_to_text(&self.salt)
            };
        });
    }
}

impl HasherFrame for Pbkdf2Frame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/hashers/src/pbkdf2.rs",
        );

        ui.add_space(8.0);
        ui.byte_io_mode_hasher(&mut self.input_format, &mut self.output_format);

        ui.add_space(8.0);
        ui.subheading("Select Inner HMAC");
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.variant, HmacVariant::Md4, "MD4");
            ui.selectable_value(&mut self.variant, HmacVariant::Md5, "MD5");
        });
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.variant, HmacVariant::Sha0, "SHA0");
            ui.selectable_value(&mut self.variant, HmacVariant::Sha1, "SHA1");
        });
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
        ui.subheading("Number of Iterations");
        ui.add(DragValue::new(&mut self.iterations).range(1..=32768));

        ui.add_space(8.0);
        ui.subheading("Output Length (Bytes)");
        ui.add(DragValue::new(&mut self.hash_len).range(4..=512));

        ui.add_space(8.0);
        ui.horizontal(|ui| {
            ui.subheading("Salt (Hexadecimal)");
        });
        self.salt_control(ui);

        ui.add_space(16.0);
    }

    fn hash_string(&self, text: &str) -> Result<String, hashers::errors::HasherError> {
        let bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| hashers::errors::HasherError::general("byte format error"))?;

        let h = Pbkdf2::init(self.variant, self.iterations, self.hash_len, &self.salt)
            .hash(&bytes);

        Ok(self.output_format.byte_slice_to_text(&h))
    }
}
