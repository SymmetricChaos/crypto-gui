use crate::ciphers::transposition::RailFence;

use super::{generic_components::*, View};

use eframe::egui::{Slider, Ui};
use rand::prelude::StdRng;

impl View for RailFence {
    fn ui(&mut self, ui: &mut Ui, rng: &mut StdRng, _errors: &mut String) {
        randomize_reset(ui, self, rng);
        ui.add_space(16.0);

        ui.label("Number of Rails");
        let alpha_range = 2..=12;
        ui.add(Slider::new(&mut self.rails, alpha_range.clone()));
        ui.add_space(16.0);
    }
}
