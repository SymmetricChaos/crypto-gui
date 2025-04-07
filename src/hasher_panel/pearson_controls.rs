use super::HasherFrame;
use crate::ui_elements::UiElements;
use egui::{FontId, RichText};
use hashers::{
    pearson::{Pearson, PEARSON_ARRAY},
    traits::StatefulHasher,
};
use rand::{seq::SliceRandom, thread_rng};
use utils::byte_formatting::ByteFormat;

pub struct PearsonFrame {
    input_format: ByteFormat,
    output_format: ByteFormat,
    array: [u8; 256],
}

impl Default for PearsonFrame {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            array: PEARSON_ARRAY,
        }
    }
}

impl PearsonFrame {}

impl HasherFrame for PearsonFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/hashers/src/pearson.rs",
        );

        ui.byte_io_mode_hasher(&mut self.input_format, &mut self.output_format);

        ui.add_space(16.0);
        ui.horizontal(|ui| {
            ui.subheading("Array of Bytes");
            if ui.button("🎲").on_hover_text("randomize").clicked() {
                self.array.shuffle(&mut thread_rng());
            }
        });
        egui::Grid::new("pearson_array")
            .num_columns(16)
            .striped(true)
            .show(ui, |ui| {
                for (n, b) in self.array.into_iter().enumerate() {
                    if n % 16 == 0 && n != 0 {
                        ui.end_row()
                    }
                    ui.label(RichText::from(format!("{:02X}", b)).font(FontId::monospace(15.0)));
                }
            });
        if ui.button("Reset Array").clicked() {
            self.array = PEARSON_ARRAY;
        }
        ui.add_space(16.0);
    }

    fn hash_string(&self, text: &str) -> Result<String, hashers::errors::HasherError> {
        let bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| hashers::errors::HasherError::general("byte format error"))?;

        let h = Pearson::init(self.array).update_and_finalize(&bytes);

        Ok(self.output_format.byte_slice_to_text(&h))
    }
}
