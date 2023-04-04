use super::{generic_components::fill_code_columns, View, ViewableCode};
use crate::{cipher_panel::_generic_components::control_string, codes::Needle};

impl ViewableCode for Needle {}

impl View for Needle {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _errors: &mut String) {
        ui.label("Alphabet");
        if control_string(ui, &mut self.alphabet).changed() {
            self.set_map()
        }
        ui.add_space(16.0);
        fill_code_columns(5, 4, ui, self.chars_codes());
    }
}
