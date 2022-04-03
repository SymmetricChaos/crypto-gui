use super::{View, generic_components::fill_code_columns};
use crate::{codes::{MorseAmerican, morse_american::MorseMode::{Binary, DitDah}}};


impl View for MorseAmerican {
    fn ui(&mut self, ui: &mut eframe::egui::Ui) {
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.mode, DitDah, "DitDah");
            ui.selectable_value(&mut self.mode, Binary, "Binary");
        });
        fill_code_columns(20, 3, ui, self.chars_codes());
    }
}
