use super::{generic_components::fill_code_columns, View, ViewableCode};
use crate::codes::{binary_to_text::BinaryToTextMode, PgpWords};

impl ViewableCode for PgpWords {}

impl View for PgpWords {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _errors: &mut String) {
        ui.add_space(10.0);
        ui.label("Encoding Mode");
        ui.selectable_value(&mut self.mode, BinaryToTextMode::Hex, "Hex")
            .on_hover_text("interpret input as hexcode");
        ui.selectable_value(&mut self.mode, BinaryToTextMode::Utf8, "Text")
            .on_hover_text("convert text to raw bytes");
        ui.add_space(10.0);
        fill_code_columns(64, 4, ui, Box::new(self.chars_codes()));
    }
}
