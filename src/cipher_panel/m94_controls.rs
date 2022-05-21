use super::{generic_components::*, View};
use crate::ciphers::{Cipher, M94};
use eframe::egui::{self, Slider, Ui};
use rand::prelude::StdRng;

impl View for M94 {
    fn ui(&mut self, ui: &mut Ui, rng: &mut StdRng, _errors: &mut String) {
        randomize_reset(ui, self, rng);
        ui.add_space(16.0);

        ui.label("Alphabet");
        ui.label("ABDCEFGHIJKLMNOPQRSTUVWXYZ");
        ui.add_space(16.0);

        ui.label("Offset");
        let alpha_range = 0..=24;
        ui.add(Slider::new(&mut self.offset, alpha_range.clone()));
        ui.add_space(16.0);

        if ui.button("Randomize Wheels").clicked() {
            self.randomize(rng);
        }

        ui.label("Wheels");
        for wheel in &self.wheels {
            ui.add(egui::Label::new(egui::RichText::from(*wheel).monospace()));
        }
    }
}
