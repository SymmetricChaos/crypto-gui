use crate::ciphers::transposition::RailFence;

use super::{View, ViewableCipher, _generic_components::*};

use eframe::egui::{Slider, Ui};

impl ViewableCipher for RailFence {}

impl View for RailFence {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        randomize_reset(ui, self);
        ui.add_space(16.0);

        ui.label("Number of Rails");
        let alpha_range = 2..=12;
        ui.add(Slider::new(&mut self.rails, alpha_range.clone()));
        ui.add_space(16.0);
    }
}
