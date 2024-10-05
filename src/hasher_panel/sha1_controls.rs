use hashers::sha::Sha1;
use utils::{byte_formatting::ByteFormat, padding::md_strengthening_64_be};

use crate::ui_elements::UiElements;

use super::HasherFrame;

pub struct Sha1Frame {
    hasher: Sha1,
    example: String,
    example_padded: String,
}

impl Default for Sha1Frame {
    fn default() -> Self {
        Self {
            hasher: Default::default(),
            example: String::from("Woolworth Employee SSN: 078-05-1120"),
            example_padded: String::from("576f6f6c776f72746820456d706c6f7965652053534e3a203037382d30352d313132308000000000000000000000000000000000000000000000000000000118"),
        }
    }
}

impl Sha1Frame {
    fn padding(&mut self) {
        let mut bytes = self.example.as_bytes().to_vec();
        md_strengthening_64_be(&mut bytes, 64);
        self.example_padded = ByteFormat::Hex.byte_slice_to_text(bytes)
    }
}

impl HasherFrame for Sha1Frame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/hashers/src/sha/sha1.rs",
        );

        ui.byte_io_mode_hasher(
            &mut self.hasher.input_format,
            &mut self.hasher.output_format,
        );
        ui.add_space(16.0);

        ui.subheading(
            "SHA0 is nearly identical to SHA1, differing in a one bit rotation during each round.",
        );
        ui.checkbox(&mut self.hasher.rot, "SHA0");

        ui.add_space(16.0);

        ui.subheading("Demonstration of Padding");
        ui.label("Notice that that message is padded out to a multiple of 64-bytes with two special features. First the byte 0x80 (0b10000000) is always included after the message. Then zeroes are added as needed. Finally the original message length in bits is appended to the end, reaching the block size.");
        ui.add_space(2.0);
        if ui.control_string(&mut self.example).changed() {
            self.padding()
        }
        ui.add_space(8.0);
        ui.monospace(&self.example_padded);

        ui.add_space(16.0);
    }
    crate::hash_string! {}
}
