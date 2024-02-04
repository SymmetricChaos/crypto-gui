use hashers::{
    errors::HasherError,
    sha2::{Sha2, Sha2Variant},
    traits::ClassicHasher,
};

use crate::ui_elements::UiElements;

use super::HasherFrame;

pub struct Sha2Frame {
    hasher: Sha2,
}

impl Default for Sha2Frame {
    fn default() -> Self {
        Self {
            hasher: Default::default(),
        }
    }
}

impl Sha2Frame {}

impl HasherFrame for Sha2Frame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.add_space(16.0);

        ui.subheading("SHA-256 based");
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.hasher.variant, Sha2Variant::Sha256, "SHA-256");
            ui.selectable_value(&mut self.hasher.variant, Sha2Variant::Sha224, "SHA-224");
        });
        ui.add_space(8.0);
        ui.subheading("SHA-512 based");
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.hasher.variant, Sha2Variant::Sha512, "SHA-512");
            ui.selectable_value(&mut self.hasher.variant, Sha2Variant::Sha384, "SHA-384");
            ui.selectable_value(
                &mut self.hasher.variant,
                Sha2Variant::Sha512_224,
                "SHA-512/224",
            );
            ui.selectable_value(
                &mut self.hasher.variant,
                Sha2Variant::Sha512_256,
                "SHA-512/245",
            );
        });

        ui.add_space(16.0);
        ui.subheading("Discussion");
        match self.hasher.variant {
            Sha2Variant::Sha224 => ui.label("SHA-224 has the same compression function as SHA-256 but a different initialization vector and an output that is truncated to 224 bits (28 bytes) to make it resistant to length extension attacks."),
            Sha2Variant::Sha256 => ui.label("SHA-256 is the 256 bit (32 byte) version of SHA-2. It operates on a state of eight 32 bit words and performs sixty-four compression rounds on each chunk of the message."),
            Sha2Variant::Sha384 => ui.label("SHA-224 has the same compression function as SHA-512 but a different initialization vector and an output that is truncated to 384 bits (48 bytes) to make it resistant to length extension attacks."),
            Sha2Variant::Sha512 => ui.label("SHA-512 is the 512 bit (64 byte) version of SHA-2. It operates on a state of eight 64 bit words and performs eighty compression rounds on each chunk of the message."),
            Sha2Variant::Sha512_224 => ui.label("SHA-512/224 has the same compression function as SHA-512 but a different initialization vector and an output that is truncated to 224 bits (28 bytes) to make it resistant to length extension attacks. This truncation length makes it a drop in replacement for SHA-224 if needed."),
            Sha2Variant::Sha512_256 => ui.label("SHA-512/256 has the same compression function as SHA-512 but a different initialization vector and an output that is truncated to 256 bits (32 bytes) to make it resistant to length extension attacks. This truncation length makes it a drop in replacement for SHA-256 if needed."),
        };

        ui.label("<<<EXPLANATION OF HASH FUNCTION CODE>>>");

        ui.add_space(16.0);
    }

    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        self.hasher.hash(bytes)
    }

    fn hash_bytes_from_string(&self, text: &str) -> Result<String, HasherError> {
        self.hasher.hash_bytes_from_string(text)
    }
}
