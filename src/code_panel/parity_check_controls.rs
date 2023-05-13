use super::{View, ViewableCode};
use crate::codes::ParityBit;
use egui::Slider;

impl ViewableCode for ParityBit {}

impl View for ParityBit {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _errors: &mut String) {
        ui.label("Data Bits");
        ui.add(Slider::new(&mut self.block_size, 0..=10));
    }
}
