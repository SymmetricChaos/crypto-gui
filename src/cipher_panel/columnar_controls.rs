use eframe::egui::{TextEdit};
use super::View;
use super::generic_components::*;
use crate::ciphers::Columnar;


impl View for Columnar {
    fn ui(&mut self, ui: &mut eframe::egui::Ui) {

        randomize_button(ui, self);
        ui.add_space(16.0);

        ui.add_space(16.0);
        input_alphabet(ui, self);
        ui.add_space(16.0);

        ui.label("Key Word");
        ui.add(TextEdit::singleline(self.control_key()));

        randomize_button(ui, self);
    }
}
