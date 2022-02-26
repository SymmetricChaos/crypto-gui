use eframe::egui;
use eframe::egui::Slider;
use rand::prelude::ThreadRng;
use super::View;
use super::generic_components::*;
use crate::ciphers::M94;



impl View for M94 {
    fn ui(&mut self, ui: &mut eframe::egui::Ui) {
        ui.label("Alphabet");
        ui.label("ABDCEFGHIJKLMNOPQRSTUVWXYZ");
        ui.add_space(16.0);

        ui.label("Offset");
        let alpha_range = 0..=24;
        ui.add(Slider::new(&mut self.offset, alpha_range.clone()));
        ui.add_space(16.0);

        if ui.button("Randomize Wheels").clicked() {
            let mut rng = ThreadRng::default();
            self.randomize_wheels(&mut rng);
        }

        ui.label("Wheels");
        for wheel in &self.wheels {
            ui.add(egui::Label::new(egui::RichText::from(*wheel).monospace()));
        }

        randomize_button(ui, self);
    }
}
