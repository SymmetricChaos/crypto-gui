use super::View;
use super::generic_components::*;
use crate::codes::MorseITU;

impl View for MorseITU {
    fn ui(&mut self, ui: &mut eframe::egui::Ui) {
        ui.label("INTERFACE GOES HERE");
    }
}
