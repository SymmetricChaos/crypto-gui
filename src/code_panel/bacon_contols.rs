use super::{generic_components::fill_code_columns, View, ViewableCode};
use crate::codes::Bacon;

impl ViewableCode for Bacon {}

impl View for Bacon {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _errors: &mut String) {
        fill_code_columns(13, 2, ui, self.chars_codes());
    }
}
