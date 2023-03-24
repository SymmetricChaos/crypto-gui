use super::{generic_components::fill_code_columns, View, ViewableCode};
use crate::codes::{ascii::DisplayMode, Ascii};

impl ViewableCode for Ascii {}

impl View for Ascii {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _errors: &mut String) {
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.mode, DisplayMode::EightBitBinary, "8-Bit");
            ui.selectable_value(&mut self.mode, DisplayMode::SevenBitBinary, "7-Bit");
            ui.selectable_value(&mut self.mode, DisplayMode::Octal, "Octal");
            ui.selectable_value(&mut self.mode, DisplayMode::Decimal, "Decimal");
            ui.selectable_value(&mut self.mode, DisplayMode::Hex, "Hexadecimal");
        });

        fill_code_columns(32, 4, ui, self.chars_codes_display());
    }
}
