use eframe::egui::Slider;
use eframe::egui::Ui;
use rand::prelude::StdRng;
use super::View;
use super::generic_components::*;
use crate::ciphers::Scytale;


impl View for Scytale {
    fn ui(&mut self, ui: &mut Ui, rng: &mut StdRng, _errors: &mut String) {
        randomize_reset(ui, self, rng);
        ui.add_space(16.0);

        ui.label("Wraps");
        let alpha_range = 2..=12;
        ui.add(Slider::new(&mut self.key, alpha_range.clone()));
        ui.add_space(16.0);

    }
}