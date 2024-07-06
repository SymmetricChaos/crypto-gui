use hashers::{errors::HasherError, sha0::Sha0, traits::ClassicHasher};
use utils::byte_formatting::ByteFormat;

use crate::ui_elements::UiElements;

use super::HasherFrame;

pub struct Sha0Frame {
    hasher: Sha0,
    example: String,
    example_padded: String,
}

impl Default for Sha0Frame {
    fn default() -> Self {
        Self {
            hasher: Default::default(),
            example: String::from("Woolworth Employee SSN: 078-05-1120"),
            example_padded: String::from("576f6f6c776f72746820456d706c6f7965652053534e3a203037382d30352d313132308000000000000000000000000000000000000000000000000000000118"),
        }
    }
}

impl Sha0Frame {
    fn padding(&mut self) {
        let mut bytes = self.example.as_bytes().to_vec();

        let b_len = (bytes.len().wrapping_mul(8)) as u64;

        // Step 1.Padding
        // push a byte with a leading 1 to the bytes
        bytes.push(0x80);
        // push zeros until the length in bits is 448 mod 512
        // equivalently until the length in bytes is 56 mod 64
        while (bytes.len() % 64) != 56 {
            bytes.push(0)
        }

        // Step 2. Append length
        for b in b_len.to_be_bytes() {
            bytes.push(b)
        }

        self.example_padded = ByteFormat::Hex.byte_slice_to_text(bytes)
    }
}

impl HasherFrame for Sha0Frame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.add_space(16.0);

        ui.byte_io_mode_hasher(
            &mut self.hasher.input_format,
            &mut self.hasher.output_format,
        );

        ui.add_space(16.0);

        ui.subheading("Demonstration of Padding");
        ui.label("Notice that that message is padded out to a multiple of 64-bytes with two special features. First the byte 0x80 (0b10000000) is always included after the message and the message length in bits is appended to the end.");
        ui.add_space(2.0);
        if ui.control_string(&mut self.example).changed() {
            self.padding()
        }
        ui.add_space(4.0);
        ui.mono_strong(&self.example_padded);

        ui.add_space(16.0);
    }

    fn hash_bytes_from_string(&self, text: &str) -> Result<String, HasherError> {
        self.hasher.hash_bytes_from_string(text)
    }
}
