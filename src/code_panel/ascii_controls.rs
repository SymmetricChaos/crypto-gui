use eframe::egui::{Grid, RichText};
use super::View;
use crate::codes::ASCII;

impl View for ASCII {
    fn ui(&mut self, ui: &mut eframe::egui::Ui) {
        Grid::new("ascii_grid").num_columns(2).show(ui, |ui| {
            for (c, code) in self.chars_codes() {
                ui.label(RichText::new(c.to_string()).monospace());
                ui.label(RichText::new(code).monospace());
                ui.end_row();
            }
        });
    }
}
