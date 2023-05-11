use egui::Slider;

use super::{View, ViewableCode};
use crate::codes::Repetition;

impl ViewableCode for Repetition {}

impl View for Repetition {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _errors: &mut String) {
        ui.label("Block Size");
        ui.add(Slider::new(&mut self.block_size, 3..=9));
    }
}
