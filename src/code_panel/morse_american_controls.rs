use eframe::egui::Grid;

use super::View;
use crate::{codes::{MorseAmerican, morse_american::MorseMode::{Binary, DitDah}}, egui_aux::mono_strong};

const NUM_ROWS: usize = 3;

impl View for MorseAmerican {
    fn ui(&mut self, ui: &mut eframe::egui::Ui) {
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.mode, DitDah, "DitDah");
            ui.selectable_value(&mut self.mode, Binary, "Binary");
        });
        Grid::new("morse_american_code_grid").num_columns(NUM_ROWS).show(ui, |ui| {
            let mut ctr = 0;
            for (c, code) in self.chars_codes() {
                let pair = format!("{}  {}", c, code);
                mono_strong(ui,&pair, Some(16.0));
                ctr += 1;
                if ctr % NUM_ROWS == 0 {
                    ui.end_row()
                }
            }
        });
    }
}
