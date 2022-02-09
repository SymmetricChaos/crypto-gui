use eframe::egui;
use eframe::egui::Slider;
use super::View;
use super::generic_components::*;
use crate::ciphers::M94;



impl View for M94 {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, input: &mut String, output: &mut String, errors: &mut String) {
        ui.label("Alphabet");
        ui.label("ABDCEFGHIJKLMNOPQRSTUVWXYZ");
        ui.add_space(16.0);

        ui.label("Offset");
        let alpha_range = 0..=24;
        ui.add(Slider::new(&mut self.offset, alpha_range.clone()));
        ui.add_space(16.0);

        ui.label("Wheels");
        for wheel in &self.wheels {
            ui.add(egui::Label::new(egui::RichText::from(*wheel).monospace()));
        }


        encrypt_decrypt(ui, self, input, output, errors);
        ui.add_space(16.0);
        randomize_button(ui, self);
    }
}
