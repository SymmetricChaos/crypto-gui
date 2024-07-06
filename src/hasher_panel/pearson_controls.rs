use crate::ui_elements::UiElements;

use super::HasherFrame;
use egui::{FontId, RichText};
use hashers::{errors::HasherError, pearson::Pearson, traits::ClassicHasher};
use rand::{seq::SliceRandom, thread_rng};

pub struct PearsonFrame {
    hasher: Pearson,
}

impl Default for PearsonFrame {
    fn default() -> Self {
        Self {
            hasher: Default::default(),
        }
    }
}

impl PearsonFrame {}

impl HasherFrame for PearsonFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.add_space(16.0);

        ui.byte_io_mode_hasher(
            &mut self.hasher.input_format,
            &mut self.hasher.output_format,
        );

        ui.add_space(16.0);
        ui.horizontal(|ui| {
            ui.subheading("Array of Bytes");
            if ui.button("🎲").on_hover_text("randomize").clicked() {
                self.hasher.array.shuffle(&mut thread_rng());
            }
        });
        egui::Grid::new("pearson_array")
            .num_columns(16)
            .striped(true)
            .show(ui, |ui| {
                for (n, b) in self.hasher.array.into_iter().enumerate() {
                    if n % 16 == 0 && n != 0 {
                        ui.end_row()
                    }
                    ui.label(RichText::from(format!("{:02X}", b)).font(FontId::monospace(15.0)));
                }
            });
        if ui.button("Reset Array").clicked() {
            self.hasher = Default::default();
        }
        ui.add_space(16.0);
    }

    fn hash_bytes_from_string(&self, text: &str) -> Result<String, HasherError> {
        self.hasher.hash_bytes_from_string(text)
    }
}
