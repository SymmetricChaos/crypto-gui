use eframe::egui::{TextEdit, Grid};

use super::View;
use crate::codes::Godel;

impl View for Godel {
    fn ui(&mut self, ui: &mut eframe::egui::Ui) {
        ui.add(TextEdit::singleline(self.control_alphabet()));
        Grid::new("godel_grid").num_columns(2).show(ui, |ui| {
            for (code, c) in self.chars_codes() {
                ui.label(code);
                ui.label(c.to_string());
                ui.end_row();
            }
        });

    }
}
