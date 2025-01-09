use super::HasherFrame;
use crate::ui_elements::UiElements;
use hashers::{poly1305::Poly1305, traits::StatefulHasher};
use rand::{thread_rng, RngCore};
use utils::byte_formatting::ByteFormat;

pub struct Poly1305Frame {
    input_format: ByteFormat,
    output_format: ByteFormat,
    key_r_string: String,
    key_r: [u8; 16],
    key_s_string: String,
    key_s: [u8; 16],
}

impl Default for Poly1305Frame {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            key_r_string: String::new(),
            key_r: [0; 16],
            key_s_string: String::new(),
            key_s: [0; 16],
        }
    }
}

impl Poly1305Frame {
    fn key_r_control(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("🎲").on_hover_text("randomize").clicked() {
                let mut rng = thread_rng();
                rng.fill_bytes(&mut self.key_r);
                self.key_r_string = ByteFormat::Hex.byte_slice_to_text(&self.key_r);
            };
            if ui.control_string(&mut self.key_r_string).lost_focus() {
                self.key_r_string = self
                    .key_r_string
                    .chars()
                    .filter(|c| c.is_ascii_hexdigit())
                    .take(32)
                    .collect();

                while self.key_r_string.len() != 32 {
                    self.key_r_string.insert(0, '0');
                }

                self.key_r = ByteFormat::Hex
                    .text_to_bytes(&self.key_r_string)
                    .expect("invalid hex bytes")
                    .try_into()
                    .expect("invalid hex bytes");
            }
        });
    }

    fn key_s_control(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("🎲").on_hover_text("randomize").clicked() {
                let mut rng = thread_rng();
                rng.fill_bytes(&mut self.key_s);
                self.key_s_string = ByteFormat::Hex.byte_slice_to_text(&self.key_s);
            };
            if ui.control_string(&mut self.key_s_string).lost_focus() {
                self.key_s_string = self
                    .key_s_string
                    .chars()
                    .filter(|c| c.is_ascii_hexdigit())
                    .take(32)
                    .collect();

                while self.key_s_string.len() != 32 {
                    self.key_s_string.insert(0, '0');
                }

                self.key_s = ByteFormat::Hex
                    .text_to_bytes(&self.key_s_string)
                    .expect("invalid hex bytes")
                    .try_into()
                    .expect("invalid hex bytes");
            }
        });
    }
}

impl HasherFrame for Poly1305Frame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/hashers/src/poly3015.rs",
        );

        ui.byte_io_mode_hasher(&mut self.input_format, &mut self.output_format);

        ui.add_space(16.0);
        ui.subheading("Key (r)");
        ui.label("The point at which the polynomial is evaluated.");
        self.key_r_control(ui);

        ui.add_space(8.0);
        ui.subheading("Key (s)");
        ui.label("A constant that is added after the polynomial is evaluated.");
        self.key_s_control(ui);

        ui.add_space(16.0);
    }

    fn hash_string(&self, text: &str) -> Result<String, hashers::errors::HasherError> {
        let bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| hashers::errors::HasherError::general("byte format error"))?;

        let h = Poly1305::init(&self.key_r, &self.key_s).update_and_finalize(&bytes);

        Ok(self.output_format.byte_slice_to_text(&h))
    }
}
