use super::{generic_components::fill_code_columns, View, ViewableCode};
use crate::codes::Linotype;

impl ViewableCode for Linotype {}

impl View for Linotype {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _errors: &mut String) {
        fill_code_columns(32, 4, ui, self.chars_codes());
    }
}
