use super::{generic_components::fill_code_columns, View, ViewableCode};
use crate::codes::{baudot::BaudotVersion, Baudot};

impl ViewableCode for Baudot {}

impl View for Baudot {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _errors: &mut String) {
        ui.selectable_value(&mut self.version, BaudotVersion::Ita1, "ITA1 (Baudot)");
        ui.selectable_value(&mut self.version, BaudotVersion::Ita2, "ITA2 (Murray)");
        ui.selectable_value(&mut self.version, BaudotVersion::UsTty, "US TTY");
        ui.add_space(16.0);
        fill_code_columns(32, 4, ui, self.codes_chars());
    }
}
