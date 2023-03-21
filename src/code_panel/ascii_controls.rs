use super::{generic_components::fill_code_columns, View, ViewableCode};
use crate::codes::{
    ascii::AsciiMode::{EightBit, SevenBit},
    Ascii,
};

impl ViewableCode for Ascii {}

impl View for Ascii {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _errors: &mut String) {
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.mode, EightBit, "8-Bit");
            ui.selectable_value(&mut self.mode, SevenBit, "7-Bit");
        });

        fill_code_columns(32, 4, ui, Box::new(self.chars_codes()));
    }
}
