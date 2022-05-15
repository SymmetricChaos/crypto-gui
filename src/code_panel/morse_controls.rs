use super::{generic_components::fill_code_columns, View};
use crate::codes::MorseITU;

impl View for MorseITU {
    fn ui(&mut self, ui: &mut eframe::egui::Ui) {
        // ui.horizontal(|ui| {
        //     ui.selectable_value(&mut self.mode, DitDah, "DitDah");
        //     ui.selectable_value(&mut self.mode, Binary, "Binary");
        // });
        fill_code_columns(20, 3, ui, self.chars_codes());
    }
}
