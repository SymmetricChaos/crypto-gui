use super::HasherFrame;
use crate::ui_elements::UiElements;
use egui::DragValue;
use hashers::{
    pbkdf1::{Pbkdf1, Pbkdf1Variant},
    traits::StatefulHasher,
};
use rand::{thread_rng, RngCore};
use strum::IntoEnumIterator;
use utils::byte_formatting::ByteFormat;

pub struct Pbkdf1Frame {
    input_format: ByteFormat,
    output_format: ByteFormat,
    variant: Pbkdf1Variant,
    salt_string: String,
    salt: Vec<u8>,
    iterations: u32,
    hash_len: u32,
}

impl Default for Pbkdf1Frame {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            variant: Pbkdf1Variant::Md5,
            salt_string: String::from("DEADBEEF"),
            salt: b"DEADBEEF".to_vec(),
            iterations: 4096,
            hash_len: 64,
        }
    }
}

impl Pbkdf1Frame {
    fn validate_salt(&mut self) {
        self.salt_string = self
            .salt_string
            .chars()
            .filter(|c| c.is_ascii_hexdigit())
            .take(16)
            .collect();
        if self.salt_string.len() % 2 != 0 {
            self.salt_string.insert(0, '0');
        }
        self.salt = ByteFormat::Hex
            .text_to_bytes(&self.salt_string)
            .expect("unable to parse salt input");
    }

    fn salt_control(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.control_string(&mut self.salt_string).lost_focus() {
                self.validate_salt();
            };
            if ui.button("🎲").on_hover_text("randomize").clicked() {
                let mut rng = thread_rng();
                self.salt = vec![0; 8];
                rng.fill_bytes(&mut self.salt);
                self.salt_string = ByteFormat::Hex.byte_slice_to_text(&self.salt)
            };
        });
    }
}

impl HasherFrame for Pbkdf1Frame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/hashers/src/pbkdf1.rs",
        );

        ui.add_space(8.0);
        ui.byte_io_mode_hasher(&mut self.input_format, &mut self.output_format);

        ui.add_space(8.0);
        ui.subheading("Select Inner Hasher");
        ui.horizontal(|ui| {
            for variant in Pbkdf1Variant::iter() {
                ui.selectable_value(&mut self.variant, variant, variant.to_string());
            }
        });

        ui.add_space(8.0);
        ui.subheading("Number of Iterations");
        ui.add(DragValue::new(&mut self.iterations).range(1..=32768));

        ui.add_space(8.0);
        ui.subheading("Output Length (Bytes)");
        ui.add(DragValue::new(&mut self.hash_len).range(4..=512));

        ui.add_space(8.0);
        ui.horizontal(|ui| {
            ui.subheading("Salt (Hexadecimal)");
        });
        self.salt_control(ui);
    }

    fn hash_string(&self, text: &str) -> Result<String, hashers::errors::HasherError> {
        let bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| hashers::errors::HasherError::general("byte format error"))?;

        let h = Pbkdf1::init(self.variant, self.iterations, self.hash_len, &self.salt).hash(&bytes);

        Ok(self.output_format.byte_slice_to_text(&h))
    }
}
