use eframe::egui::Grid;

use super::View;
use super::generic_components::*;
use crate::codes::ASCII;

impl View for ASCII {
    fn ui(&mut self, ui: &mut eframe::egui::Ui) {
        Grid::new("ascii_grid").num_columns(2).show(ui, |ui| {
            for (c, code) in self.chars_codes() {
                ui.label(c.to_string());
                ui.label(code);
                ui.end_row();
            }
        });
    }
}
