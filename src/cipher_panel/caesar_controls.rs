use eframe::egui::Slider;
use eframe::egui::Ui;
use rand::prelude::StdRng;
use super::View;
use super::generic_components::*;
use crate::ciphers::Caesar;

impl View for Caesar {
    fn ui(&mut self, ui: &mut Ui, rng: &mut StdRng) {

        randomize_reset(ui, self, rng);
        ui.add_space(16.0);

        input_alphabet(ui, self.control_alphabet());
        ui.add_space(16.0);

        ui.label("Key");
        let alpha_range = 0..=((self.alphabet.len() as i32 -1));
        ui.add(Slider::new(&mut self.shift, alpha_range));
        ui.add_space(16.0);
    }
}
