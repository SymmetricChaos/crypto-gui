use super::{generic_components::fill_code_columns, View, ViewableCode};
use crate::codes::PgpWords;

impl ViewableCode for PgpWords {}

impl View for PgpWords {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _errors: &mut String) {
        fill_code_columns(64, 4, ui, Box::new(self.chars_codes()));
    }
}
