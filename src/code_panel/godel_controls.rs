use super::View;
use super::generic_components::*;
use crate::codes::Godel;

impl View for Godel {
    fn ui(&mut self, ui: &mut eframe::egui::Ui) {
        ui.label("Interface Goes Here")
    }
}
