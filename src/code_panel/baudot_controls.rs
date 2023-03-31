use super::{generic_components::fill_code_columns, View, ViewableCode};
use crate::codes::Baudot;

impl ViewableCode for Baudot {}

impl View for Baudot {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _errors: &mut String) {
        fill_code_columns(32, 4, ui, self.codes_chars());
    }
}
