use egui::{TextEdit, TextStyle};

use super::{generic_components::fill_code_columns, View, ViewableCode};
use crate::{codes::Bacon, egui_aux::subheading};

impl ViewableCode for Bacon {}

impl View for Bacon {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _errors: &mut String) {
        ui.label(subheading("False Text"));
        ui.add(TextEdit::multiline(&mut self.false_text).font(TextStyle::Monospace));
        fill_code_columns(12, 4, ui, self.chars_codes());
    }
}
