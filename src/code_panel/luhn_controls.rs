use egui::Slider;

use super::{View, ViewableCode};
use crate::codes::LuhnAlgorithm;

impl ViewableCode for LuhnAlgorithm {}

impl View for LuhnAlgorithm {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _errors: &mut String) {
        ui.group(|ui| {
            ui.label("Modulus");
            ui.add(Slider::new(&mut self.modulus, 2..=36));
            if self.modulus % 2 != 0 {
                ui.label("Modulus must be even.");
            }
        });
    }
}
