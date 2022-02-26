use eframe::egui::Slider;
use super::View;
use super::generic_components::*;
use crate::ciphers::Caesar;

impl View for Caesar {
    fn ui(&mut self, ui: &mut eframe::egui::Ui) {

        randomize_button(ui, self);
        ui.add_space(16.0);

        input_alphabet(ui, self);
        ui.add_space(16.0);

        ui.label("Key");
        let alpha_range = 0..=((self.alphabet.len() as i32 -1));
        ui.add(Slider::new(&mut self.shift, alpha_range));
        ui.add_space(16.0);
    }
}
