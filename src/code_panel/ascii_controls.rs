use eframe::{egui::{Grid}};
use super::View;
use crate::codes::{Ascii, ascii::AsciiMode::{EightBit, SevenBit}};
use crate::egui_aux::mono_strong;

const NUM_ROWS: usize = 4;

impl View for Ascii {
    fn ui(&mut self, ui: &mut eframe::egui::Ui) {
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.mode, EightBit, "8-Bit");
            ui.selectable_value(&mut self.mode, SevenBit, "7-Bit");
        });
        Grid::new("ascii_code_grid").num_columns(NUM_ROWS).show(ui, |ui| {
            let mut ctr = 0;
            for (c, code) in self.chars_codes() {
                let pair = match self.mode {
                    SevenBit => format!("{}   {}     ", c, code),
                    EightBit => format!("{}  {}     ", c, code),
                };
                mono_strong(ui,&pair, None);
                ctr += 1;
                if ctr % NUM_ROWS == 0 {
                    ui.end_row()
                }
            }
        });
    }
}
