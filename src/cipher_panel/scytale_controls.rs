use crate::ciphers::transposition::Scytale;

use super::{View, ViewableCipher, _generic_components::*};

use eframe::egui::{Slider, Ui};

impl ViewableCipher for Scytale {}

impl View for Scytale {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        randomize_reset(ui, self);
        ui.add_space(16.0);

        ui.label("Wraps");
        let alpha_range = 2..=12;
        ui.add(Slider::new(&mut self.key, alpha_range.clone()));
        ui.add_space(16.0);
    }
}
