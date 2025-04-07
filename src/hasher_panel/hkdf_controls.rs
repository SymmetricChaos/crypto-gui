use super::HasherFrame;
use crate::ui_elements::{validate_string_hex_bytes, UiElements};
use egui::DragValue;
use hashers::{errors::HasherError, hkdf::Hkdf, hmac::HmacVariant};
use rand::{thread_rng, RngCore};
use utils::byte_formatting::ByteFormat;

pub struct HkdfFrame {
    input_format: ByteFormat,
    output_format: ByteFormat,
    variant: HmacVariant,
    length: usize,
    salt: Vec<u8>,
    salt_string: String,
    info: Vec<u8>,
    info_string: String,
}

impl Default for HkdfFrame {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            variant: HmacVariant::Sha256,
            length: 32,
            salt: Vec::new(),
            salt_string: String::new(),
            info: Vec::new(),
            info_string: String::new(),
        }
    }
}

impl HkdfFrame {
    fn validate_salt(&mut self) {
        validate_string_hex_bytes(&mut self.salt_string, None);
        self.salt = ByteFormat::Hex
            .text_to_bytes(&self.salt_string)
            .expect("unable to parse key")
    }

    fn salt_control(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.subheading("Salt (Hexadecimal)");
            if ui.button("🎲").on_hover_text("randomize").clicked() {
                let mut rng = thread_rng();
                self.salt = vec![0; 32];
                rng.fill_bytes(&mut self.salt);
                self.salt_string = ByteFormat::Hex.byte_slice_to_text(&self.salt)
            };
        });
        ui.label("Optional salt.");
        if ui.control_string(&mut self.salt_string).lost_focus() {
            self.validate_salt();
        };
    }

    fn validate_info(&mut self) {
        validate_string_hex_bytes(&mut self.info_string, None);
        self.info = ByteFormat::Hex
            .text_to_bytes(&self.info_string)
            .expect("unable to parse info")
    }

    fn info_control(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.subheading("Info (Hexadecimal)");
            if ui.button("🎲").on_hover_text("randomize").clicked() {
                let mut rng = thread_rng();
                self.info = vec![0; 32];
                rng.fill_bytes(&mut self.info);
                self.info_string = ByteFormat::Hex.byte_slice_to_text(&self.info)
            };
        });
        ui.label("Optional context string.");
        if ui.control_string(&mut self.info_string).lost_focus() {
            self.validate_info();
        };
    }
}

impl HasherFrame for HkdfFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/hashers/src/hkdf.rs",
        );
        ui.add_space(8.0);

        ui.byte_io_mode_hasher(&mut self.input_format, &mut self.output_format);
        ui.add_space(16.0);

        ui.subheading("Algorithm");
        ui.label("Any HMAC variant can be used as the basis for HKDF.");

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
        self.salt_control(ui);

        ui.add_space(8.0);
        self.info_control(ui);

        ui.add_space(8.0);
        ui.subheading("Output Length (bytes)");
        ui.add(DragValue::new(&mut self.length).range(1..=1024));

        ui.add_space(16.0);
    }

    fn hash_string(&self, text: &str) -> Result<String, HasherError> {
        let ikm = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| hashers::errors::HasherError::general("byte format error"))?;

        let h = Hkdf::derive_key(self.variant, self.length, &ikm, &self.salt, &self.info);

        Ok(self.output_format.byte_slice_to_text(&h))
    }
}
