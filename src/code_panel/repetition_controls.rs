use egui::Slider;

use super::{View, ViewableCode};
use crate::codes::Repetition;

impl ViewableCode for Repetition {}

impl View for Repetition {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _errors: &mut String) {
        ui.label("Block Size");
        ui.add(Slider::new(&mut self.block_size, 3..=9));

        ui.add_space(16.0);

        if self.block_size % 2 == 0 {
            ui.label(format!(
                "Correct {}-bit errors\nDetect {}-bit errors",
                self.block_size / 2 - 1,
                self.block_size / 2
            ));
        } else {
            ui.label(format!("Correct {}-bit errors", self.block_size / 2,));
        }
    }
}
