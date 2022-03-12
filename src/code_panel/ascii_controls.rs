use super::View;
use super::generic_components::*;
use crate::codes::ASCII;

impl View for ASCII {
    fn ui(&mut self, ui: &mut eframe::egui::Ui) {
        ui.label("INTERFACE GOES HERE");
    }
}
