use crate::ui_elements::UiElements;

use super::HasherFrame;
use hashers::murmurhash3::{Murmur3_128, Murmur3_32};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Murmur3Selector {
    M32,
    M128,
}

pub struct Murmur3Frame {
    hasher32: Murmur3_32,
    hasher128: Murmur3_128,
    selector: Murmur3Selector,
}

impl Default for Murmur3Frame {
    fn default() -> Self {
        Self {
            hasher32: Default::default(),
            hasher128: Default::default(),
            selector: Murmur3Selector::M32,
        }
    }
}

impl Murmur3Frame {}

impl HasherFrame for Murmur3Frame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/hashers/src/murmurhash3.rs",
        );
        ui.add_space(8.0);

        ui.selectable_value(&mut self.selector, Murmur3Selector::M32, "Murmur3_32");
        ui.selectable_value(&mut self.selector, Murmur3Selector::M128, "Murmur3_128");

        ui.add_space(8.0);

        match self.selector {
            Murmur3Selector::M32 => {
                ui.byte_io_mode_hasher(
                    &mut self.hasher32.input_format,
                    &mut self.hasher32.output_format,
                );
                ui.add_space(8.0);
                ui.subheading("Seed");
                ui.u32_hex_edit(&mut self.hasher32.seed);
            }
            Murmur3Selector::M128 => {
                ui.byte_io_mode_hasher(
                    &mut self.hasher128.input_format,
                    &mut self.hasher128.output_format,
                );
                ui.add_space(8.0);
                ui.subheading("Seed");
                ui.u32_hex_edit(&mut self.hasher128.seed);
            }
        }

        ui.add_space(16.0);

        ui.label("<<<EXPLANATION OF HASH FUNCTION CODE>>>");

        ui.add_space(16.0);
    }

    fn hash_string(&self, text: &str) -> Result<String, hashers::errors::HasherError> {
        match self.selector {
            Murmur3Selector::M32 => {
                hashers::traits::ClassicHasher::hash_bytes_from_string(&self.hasher32, text)
            }
            Murmur3Selector::M128 => {
                hashers::traits::ClassicHasher::hash_bytes_from_string(&self.hasher128, text)
            }
        }
    }
}
