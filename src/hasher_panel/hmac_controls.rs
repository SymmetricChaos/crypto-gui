use crate::ui_elements::UiElements;

use super::HasherFrame;
use hashers::{
    errors::HasherError,
    hmac::Hmac,
    md4::Md4,
    md5::Md5,
    sha2::{
        sha256::{Sha2_224, Sha2_256},
        sha512::{Sha2_384, Sha2_512},
    },
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
    Sha2_384,
    Sha2_512,
}

impl HmacHasher {
    pub fn block_size(&self) -> usize {
        match self {
            HmacHasher::Md4 => 64,
            HmacHasher::Md5 => 64,
            HmacHasher::Sha2_224 => 64,
            HmacHasher::Sha2_256 => 64,
            HmacHasher::Sha2_384 => 128,
            HmacHasher::Sha2_512 => 128,
        }
    }

    pub fn hasher(&self) -> Box<dyn ClassicHasher> {
        match self {
            HmacHasher::Md4 => Box::new(Md4::default()),
            HmacHasher::Md5 => Box::new(Md5::default()),
            HmacHasher::Sha2_224 => Box::new(Sha2_224::default()),
            HmacHasher::Sha2_256 => Box::new(Sha2_256::default()),
            HmacHasher::Sha2_384 => Box::new(Sha2_384::default()),
            HmacHasher::Sha2_512 => Box::new(Sha2_512::default()),
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
        ui.subheading("Key");
        ui.label("Any number of bytes.");
        ui.horizontal(|ui| {
            if ui.control_string(&mut self.key_string).changed() {
                if self.hasher.key_from_str(&self.key_string).is_err() {
                    self.valid_key = false
                } else {
                    self.valid_key = true
                }
            };
            if ui.button("🎲").on_hover_text("randomize").clicked() {
                let mut rng = thread_rng();
                self.hasher.key = vec![0; self.hasher.block_size / 4];
                rng.fill_bytes(&mut self.hasher.key);
                self.key_string = self
                    .hasher
                    .key_format
                    .byte_slice_to_text(&mut self.hasher.key)
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

        ui.subheading("Algorithm");
        ui.label("HMAC accepts a hasher, a key, and a message. In the case that the key is larger than the block size of the hasher it is hashed and that hash is used as the key instead.\n1) Each byte of the key is XORed with the padding byte 0x5c and the padding bytes continue up to the block size of the hasher.\n2) The message is appended to the key and that entire sequence of bytes is hashed.\n3) Each byte of the key is XORed with the padding byte 0x36 and the padding bytes continue up to the block size of the hasher.\n4) The previously hashed result is appended to this padded key and that entire sequence of bytes is hashed.\nHMAC = H((key ⊕ outer_pad) || H( (key ⊕ inner_pad) || message )))");

        ui.byte_io_mode(
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
                || ui
                    .selectable_value(&mut self.inner_hasher, HmacHasher::Sha2_384, "SHA384")
                    .clicked()
                || ui
                    .selectable_value(&mut self.inner_hasher, HmacHasher::Sha2_512, "SHA512")
                    .clicked()
            {
                self.hasher.block_size = self.inner_hasher.block_size();
                self.hasher.hasher = self.inner_hasher.hasher();
            }
        });

        ui.add_space(16.0);
        ui.collapsing("Key Format", |ui| {
            ui.label("Key can be given as text, hexadecimal, or Base64.");
            ui.horizontal(|ui| {
                ui.selectable_value(
                    &mut self.hasher.key_format,
                    ByteFormat::Utf8,
                    "Text (UTF-8)",
                );
                ui.selectable_value(&mut self.hasher.key_format, ByteFormat::Hex, "Hexadecimal");
                ui.selectable_value(&mut self.hasher.key_format, ByteFormat::Base64, "Base64");
            });
        });
        ui.add_space(8.0);
        self.key_control(ui);

        ui.add_space(16.0);
    }

    fn hash_bytes_from_string(&self, text: &str) -> Result<String, HasherError> {
        self.hasher.hash_bytes_from_string(text)
    }
}
