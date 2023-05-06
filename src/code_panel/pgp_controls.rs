use super::{
    generic_components::{binary_to_text_input_mode, fill_code_columns},
    View, ViewableCode,
};
use crate::codes::PgpWords;

impl ViewableCode for PgpWords {}

impl View for PgpWords {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _errors: &mut String) {
        ui.add_space(16.0);
        binary_to_text_input_mode(ui, &mut self.mode);
        ui.add_space(16.0);
        fill_code_columns(64, 4, ui, Box::new(self.chars_codes()));
    }
}
