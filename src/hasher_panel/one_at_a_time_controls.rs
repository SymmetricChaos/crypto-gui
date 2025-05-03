use super::HasherFrame;
use crate::ui_elements::UiElements;
use hashers::{one_at_a_time::OneAtATime, traits::StatefulHasher};
use std::num::Wrapping;
use utils::byte_formatting::ByteFormat;

pub struct OaatFrame {
    input_format: ByteFormat,
    output_format: ByteFormat,
    example_string: String,
    example_bytes: Vec<u8>,
    example_hash: Wrapping<u32>,
}

impl Default for OaatFrame {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            example_string: String::from("Hồ Chí Min"),
            example_bytes: vec![
                0x48, 0xe1, 0xbb, 0x93, 0x43, 0x68, 0xc3, 0xad, 0x4d, 0x69, 0x6e, 0x68,
            ],
            example_hash: Wrapping(0u32),
        }
    }
}

impl HasherFrame for OaatFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/hashers/src/one_at_a_time.rs",
        );
        ui.add_space(8.0);
        ui.byte_io_mode_hasher(&mut self.input_format, &mut self.output_format);
        ui.add_space(16.0);
        ui.separator();
        ui.add_space(16.0);
        ui.subheading("Interactive Example");
        ui.horizontal(|ui| {
            if ui.button("Next Byte").clicked() {
                if self.example_bytes.len() > 0 {
                    let byte = self.example_bytes.remove(0);
                    self.example_hash += byte as u32;
                    self.example_hash += self.example_hash << 10;
                    self.example_hash ^= self.example_hash >> 6;
                }
            }

            if ui.button("Finalize").clicked() {
                self.example_hash += self.example_hash << 3;
                self.example_hash ^= self.example_hash >> 11;
                self.example_hash += self.example_hash << 15;
            }

            if ui.button("Reset").clicked() {
                self.example_string = String::from("Hồ Chí Min");
                self.example_bytes = vec![
                    0x48, 0xe1, 0xbb, 0x93, 0x43, 0x68, 0xc3, 0xad, 0x4d, 0x69, 0x6e, 0x68,
                ];
                self.example_hash = Wrapping(0);
            }
        });
        ui.add_space(4.0);
        ui.label("Provide some text which will be converted to bytes. Then click \"Next Byte\" to take one byte at a time and mix it into the hash. When all bytes are taken click \"Finalize\" to perform a last mixing step.");

        ui.add_space(8.0);
        if ui.control_string(&mut self.example_string).changed() {
            self.example_bytes = ByteFormat::Utf8
                .text_to_bytes(&self.example_string)
                .expect("invalid UTF-8");
        }
        ui.add_space(4.0);
        ui.label(format!(
            "Unused Bytes: {}",
            ByteFormat::Hex.byte_slice_to_text(&self.example_bytes)
        ));
        ui.add_space(4.0);

        ui.label(format!("Hash: {:08x?}", self.example_hash));
        ui.add_space(16.0);
    }

    fn hash_string(&self, text: &str) -> Result<String, hashers::errors::HasherError> {
        let bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| hashers::errors::HasherError::general("byte format error"))?;

        Ok(self
            .output_format
            .byte_slice_to_text(&OneAtATime::init().hash(&bytes)))
    }
}
