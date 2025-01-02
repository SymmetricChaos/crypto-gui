use super::HasherFrame;
use crate::ui_elements::UiElements;
use egui::DragValue;
use hashers::{siphash::SipHash, traits::StatefulHasher};
use rand::{thread_rng, Rng};
use utils::byte_formatting::ByteFormat;

pub struct SipHashFrame {
    input_format: ByteFormat,
    output_format: ByteFormat,
    key: [u64; 2],
    compression_rounds: usize,
    finalization_rounds: usize,
}

impl Default for SipHashFrame {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            key: [0; 2],
            compression_rounds: 2,
            finalization_rounds: 4,
        }
    }
}

impl HasherFrame for SipHashFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/hashers/src/siphash.rs",
        );

        ui.add_space(8.0);
        ui.byte_io_mode_hasher(&mut self.input_format, &mut self.output_format);

        ui.add_space(16.0);
        ui.horizontal(|ui| {
            ui.subheading("Key");
            if ui.button("🎲").on_hover_text("randomize").clicked() {
                let mut rng = thread_rng();
                self.key[0] = rng.gen();
                self.key[1] = rng.gen();
            };
        });
        ui.u64_hex_edit(&mut self.key[0]);
        ui.u64_hex_edit(&mut self.key[1]);

        ui.add_space(16.0);
        ui.subheading("Compression Rounds");
        ui.add(DragValue::new(&mut self.compression_rounds).range(0..=8));

        ui.add_space(8.0);
        ui.subheading("Finalization Rounds");
        ui.add(DragValue::new(&mut self.finalization_rounds).range(1..=10));

        ui.add_space(16.0);
    }

    fn hash_string(&self, text: &str) -> Result<String, hashers::errors::HasherError> {
        let bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| hashers::errors::HasherError::general("byte format error"))?;

        Ok(self.output_format.byte_slice_to_text(
            SipHash::init(self.key, self.compression_rounds, self.finalization_rounds).hash(&bytes),
        ))
    }
}
