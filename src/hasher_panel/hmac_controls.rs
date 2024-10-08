use crate::ui_elements::UiElements;

use super::HasherFrame;
use hashers::hmac::{Hmac, HmacVariant};
use rand::{thread_rng, RngCore};
use strum::IntoEnumIterator;
use utils::byte_formatting::ByteFormat;

pub struct HmacFrame {
    hasher: Hmac,
    select_hasher: HmacVariant,
    key_string: String,
    key_format: ByteFormat,
    valid_key: bool,
}

impl Default for HmacFrame {
    fn default() -> Self {
        Self {
            hasher: Hmac::default(),
            select_hasher: HmacVariant::Sha256,
            key_string: String::new(),
            key_format: ByteFormat::Utf8,
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
                if self
                    .hasher
                    .set_key_from_str(ByteFormat::Hex, &self.key_string)
                    .is_err()
                {
                    self.valid_key = false
                } else {
                    self.valid_key = true
                }
            };
            if ui.button("🎲").on_hover_text("randomize").clicked() {
                let mut rng = thread_rng();
                self.hasher.key = vec![0; self.hasher.variant.block_size() / 4];
                rng.fill_bytes(&mut self.hasher.key);
                self.key_string = self.key_format.byte_slice_to_text(&mut self.hasher.key)
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
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/hashers/src/hmac.rs",
        );
        ui.add_space(8.0);

        ui.subheading("Algorithm");
        ui.label("HMAC accepts a hash function, a key, and a message. In the case that the key is larger than the block size of the hasher it is hashed and that hash is used as the key instead.\n1) Each byte of the key is XORed with the padding byte 0x5c and the padding bytes continue up to the block size of the hasher.\n2) The message is appended to the key and that entire sequence of bytes is hashed.\n3) Each byte of the key is XORed with the padding byte 0x36 and the padding bytes continue up to the block size of the hasher.\n4) The previously hashed result is appended to this padded key and that entire sequence of bytes is hashed.\nHMAC = H((key ⊕ outer_pad) || H( (key ⊕ inner_pad) || message )))");

        ui.byte_io_mode_hasher(
            &mut self.hasher.input_format,
            &mut self.hasher.output_format,
        );
        ui.add_space(16.0);

        ui.horizontal(|ui| {
            for variant in HmacVariant::iter() {
                ui.selectable_value(&mut self.select_hasher, variant, variant.to_string());
            }
        });

        ui.add_space(16.0);
        ui.collapsing("Key Format", |ui| {
            ui.label("Key can be given as text, hexadecimal, or Base64.");
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.key_format, ByteFormat::Utf8, "Text (UTF-8)");
                ui.selectable_value(&mut self.key_format, ByteFormat::Hex, "Hexadecimal");
                ui.selectable_value(&mut self.key_format, ByteFormat::Base64, "Base64");
            });
        });
        ui.add_space(8.0);
        self.key_control(ui);

        ui.add_space(16.0);
    }

    crate::hash_string! {}
}
