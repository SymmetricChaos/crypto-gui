use super::HasherFrame;
use crate::ui_elements::{validate_string_hex_bytes, UiElements};
use hashers::{ghash::Ghash, traits::StatefulHasher};
use rand::{thread_rng, RngCore};
use utils::byte_formatting::ByteFormat;

pub struct GhashFrame {
    input_format: ByteFormat,
    output_format: ByteFormat,
    h: u128,
    c: u128,
    ad_string: String,
    ad: Vec<u8>,
}

impl Default for GhashFrame {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            h: 0,
            c: 0,
            ad_string: String::new(),
            ad: Vec::new(),
        }
    }
}

impl GhashFrame {
    fn validate_ad(&mut self) {
        validate_string_hex_bytes(&mut self.ad_string, None);
        self.ad = ByteFormat::Hex
            .text_to_bytes(&self.ad_string)
            .expect("unable to parse ad")
    }

    fn ad_control(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.control_string(&mut self.ad_string).lost_focus() {
                self.validate_ad();
            };
            if ui.button("🎲").on_hover_text("randomize").clicked() {
                let mut rng = thread_rng();
                self.ad = vec![0; 32];
                rng.fill_bytes(&mut self.ad);
                self.ad_string = ByteFormat::Hex.byte_slice_to_text(&self.ad)
            };
        });
    }
}

impl HasherFrame for GhashFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/hashers/src/ghash.rs",
        );
        ui.add_space(8.0);

        ui.byte_io_mode_hasher(&mut self.input_format, &mut self.output_format);
        ui.add_space(4.0);

        ui.subheading("Additional Data");
        ui.label("Any number of bytes of the input can be provided as Additional Data which is processed first.");
        self.ad_control(ui);
        ui.add_space(4.0);

        ui.horizontal(|ui| {
            ui.subheading("H Value");
            ui.random_num_button(&mut self.h);
        });
        ui.label("H is the point at which the GHASH polynomial is evaluated.");
        ui.u128_hex_edit(&mut self.h);

        ui.add_space(4.0);
        ui.horizontal(|ui| {
            ui.subheading("Constant Value");
            ui.random_num_button(&mut self.c);
        });
        ui.label("The constant value is the x^0 coefficient of the GHASH polynomial. It is simply XORed into hash state before it is returned.");
        ui.u128_hex_edit(&mut self.c);

        ui.add_space(16.0);
    }

    fn hash_string(&self, text: &str) -> Result<String, hashers::errors::HasherError> {
        let bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| hashers::errors::HasherError::general("byte format error"))?;

        let h = Ghash::init(&self.h.to_be_bytes(), &self.c.to_be_bytes(), &self.ad).hash(&bytes);

        Ok(self.output_format.byte_slice_to_text(&h))
    }
}
