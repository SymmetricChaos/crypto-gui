use crate::ui_elements::UiElements;

use super::{byte_formatting_io, HasherFrame};
use hashers::{
    errors::HasherError,
    hmac::Hmac,
    md4::Md4,
    md5::Md5,
    sha2::sha256::{Sha2_224, Sha2_256},
    traits::ClassicHasher,
};
use rand::{thread_rng, RngCore};
use utils::byte_formatting::ByteFormat;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HmacHasher {
    Md4,
    Md5,
    Sha2_224,
    Sha2_256,
}

impl HmacHasher {
    pub fn block_size(&self) -> usize {
        match self {
            HmacHasher::Md4 => 64,
            HmacHasher::Md5 => 64,
            HmacHasher::Sha2_224 => 64,
            HmacHasher::Sha2_256 => 64,
        }
    }

    pub fn hasher(&self) -> Box<dyn ClassicHasher> {
        match self {
            HmacHasher::Md4 => Box::new(Md4::default()),
            HmacHasher::Md5 => Box::new(Md5::default()),
            HmacHasher::Sha2_224 => Box::new(Sha2_224::default()),
            HmacHasher::Sha2_256 => Box::new(Sha2_256::default()),
        }
    }
}

pub struct HmacFrame {
    hasher: Hmac,
    inner_hasher: HmacHasher,
    key_string: String,
    valid_key: bool,
}

impl Default for HmacFrame {
    fn default() -> Self {
        Self {
            hasher: Default::default(),
            inner_hasher: HmacHasher::Sha2_256,
            key_string: String::new(),
            valid_key: false,
        }
    }
}

impl HmacFrame {
    fn key_control(&mut self, ui: &mut egui::Ui) {
        // let string = &mut self.key_string;
        ui.subheading("Key");
        ui.label("Any number of bytes as hexadecimal digits.");
        ui.horizontal(|ui| {
            if ui.control_string(&mut self.key_string).changed() {
                self.key_string = self
                    .key_string
                    .chars()
                    .filter(|c| c.is_ascii_hexdigit())
                    .collect();
                if self.hasher.key_from_str(&self.key_string).is_err() {
                    self.valid_key = false
                } else {
                    self.valid_key = true
                }
            };
            if ui.button("🎲").on_hover_text("randomize").clicked() {
                let mut rng = thread_rng();
                self.hasher.key = vec![0; self.hasher.block_size];
                rng.fill_bytes(&mut self.hasher.key);
                self.key_string = ByteFormat::Hex.byte_slice_to_text(&mut self.hasher.key)
            }
        });

        if self.valid_key {
            ui.error_text("");
        } else {
            ui.error_text("invalid key");
        }
    }
}

impl HasherFrame for HmacFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.add_space(16.0);

        byte_formatting_io(
            ui,
            &mut self.hasher.input_format,
            &mut self.hasher.output_format,
        );
        ui.add_space(16.0);

        ui.horizontal(|ui| {
            if ui
                .selectable_value(&mut self.inner_hasher, HmacHasher::Md4, "MD4")
                .clicked()
                || ui
                    .selectable_value(&mut self.inner_hasher, HmacHasher::Md5, "MD5")
                    .clicked()
                || ui
                    .selectable_value(&mut self.inner_hasher, HmacHasher::Sha2_224, "SHA224")
                    .clicked()
                || ui
                    .selectable_value(&mut self.inner_hasher, HmacHasher::Sha2_256, "SHA256")
                    .clicked()
            {
                self.hasher.block_size = self.inner_hasher.block_size();
                self.hasher.hasher = self.inner_hasher.hasher();
            }
        });

        ui.add_space(16.0);

        self.key_control(ui);

        ui.add_space(16.0);
    }

    fn hash_bytes_from_string(&self, text: &str) -> Result<String, HasherError> {
        self.hasher.hash_bytes_from_string(text)
    }
}
