use eframe::egui::Grid;

use super::View;
use crate::codes::MorseITU;

impl View for MorseITU {
    fn ui(&mut self, ui: &mut eframe::egui::Ui) {
        Grid::new("morse_grid").num_columns(2).show(ui, |ui| {
            for (c, code) in self.chars_codes() {
                ui.label(c.to_string());
                ui.label(code);
                ui.end_row();
            }
        });
    }
}
