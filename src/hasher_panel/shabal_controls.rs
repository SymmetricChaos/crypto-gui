use hashers::shabal::{Shabal, ShabalVariant};

use crate::ui_elements::UiElements;

use super::HasherFrame;

pub struct ShabalFrame {
    hasher: Shabal,
}

impl Default for ShabalFrame {
    fn default() -> Self {
        Self {
            hasher: Default::default(),
        }
    }
}

impl ShabalFrame {}

impl HasherFrame for ShabalFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/hashers/src/shabal.rs",
        );

        ui.byte_io_mode_hasher(
            &mut self.hasher.input_format,
            &mut self.hasher.output_format,
        );

        ui.add_space(16.0);
        ui.subheading("Shabal-256 based");
        ui.horizontal(|ui| {
            ui.selectable_value(
                &mut self.hasher.variant,
                ShabalVariant::Shabal192,
                "Shabal-192",
            );
            ui.selectable_value(
                &mut self.hasher.variant,
                ShabalVariant::Shabal224,
                "Shabal-224",
            );
            ui.selectable_value(
                &mut self.hasher.variant,
                ShabalVariant::Shabal256,
                "Shabal-256",
            );

            ui.selectable_value(
                &mut self.hasher.variant,
                ShabalVariant::Shabal384,
                "Shabal-384",
            );
            ui.selectable_value(
                &mut self.hasher.variant,
                ShabalVariant::Shabal512,
                "Shabal-512",
            );
        });

        // ui.add_space(16.0);
        // ui.subheading("Discussion");
        // match self.hasher.variant {
        //     Sha2Variant::Sha224 => ui.label("SHA-224 has the same compression function as SHA-256 but a different initialization vector and an output that is truncated to 224 bits (28 bytes) to make it resistant to length extension attacks."),
        //     Sha2Variant::Sha256 => ui.label("SHA-256 is the 256 bit (32 byte) version of SHA-2. It operates on a state of eight 32 bit words and performs sixty-four compression rounds on each chunk of the message."),
        //     Sha2Variant::Sha384 => ui.label("SHA-224 has the same compression function as SHA-512 but a different initialization vector and an output that is truncated to 384 bits (48 bytes) to make it resistant to length extension attacks."),
        //     Sha2Variant::Sha512 => ui.label("SHA-512 is the 512 bit (64 byte) version of SHA-2. It operates on a state of eight 64 bit words and performs eighty compression rounds on each chunk of the message."),
        //     Sha2Variant::Sha512_224 => ui.label("SHA-512/224 has the same compression function as SHA-512 but a different initialization vector and an output that is truncated to 224 bits (28 bytes) to make it resistant to length extension attacks. This truncation length makes it a drop in replacement for SHA-224 if needed."),
        //     Sha2Variant::Sha512_256 => ui.label("SHA-512/256 has the same compression function as SHA-512 but a different initialization vector and an output that is truncated to 256 bits (32 bytes) to make it resistant to length extension attacks. This truncation length makes it a drop in replacement for SHA-256 if needed."),
        // };

        ui.label("<<<EXPLANATION OF HASH FUNCTION CODE>>>");

        ui.add_space(16.0);
    }
    crate::hash_string! {}
}
