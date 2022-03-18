use eframe::egui::{Grid, RichText};
use super::View;
use crate::codes::ASCII;

const NUM_ROWS: usize = 4;

impl View for ASCII {
    fn ui(&mut self, ui: &mut eframe::egui::Ui) {
        Grid::new("ascii_code_grid").num_columns(NUM_ROWS).show(ui, |ui| {
            let mut ctr = 0;
            for (c, code) in self.chars_codes() {
                let pair = format!("{}  {}", c, code);
                ui.label(RichText::new(pair).monospace());
                ctr += 1;
                if ctr % NUM_ROWS == 0 {
                    ui.end_row()
                }
            }
        });
    }
}
