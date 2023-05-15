use egui::Slider;

use super::{View, ViewableCode};
use crate::codes::MofNCode;

impl ViewableCode for MofNCode {}

impl View for MofNCode {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _errors: &mut String) {
        ui.group(|ui| {
            ui.label("Weight");
            ui.add(Slider::new(&mut self.weight, 1..=self.length));
            ui.label("Length");
            ui.add(Slider::new(&mut self.length, 0..=10));
        });
    }
}
