use super::generic_components::fill_code_columns;
use super::View;
use crate::codes::{
    ascii::AsciiMode::{EightBit, SevenBit},
    Ascii,
};

impl View for Ascii {
    fn ui(&mut self, ui: &mut eframe::egui::Ui) {
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.mode, EightBit, "8-Bit");
            ui.selectable_value(&mut self.mode, SevenBit, "7-Bit");
        });
        fill_code_columns(32, 4, ui, self.chars_codes());
    }
}
