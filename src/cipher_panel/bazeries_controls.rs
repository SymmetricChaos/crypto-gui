use eframe::egui::{self, Slider, Ui};
use rand::prelude::StdRng;
use super::View;
use super::generic_components::*;
use crate::ciphers::Bazeries;


impl View for Bazeries {
    fn ui(&mut self, ui: &mut Ui, rng: &mut StdRng) {

        randomize_reset(ui, self, rng);
        ui.add_space(16.0);

        ui.label("Alphabet");
        input_alphabet(ui, &mut self.alphabet);
        ui.add_space(16.0);

        ui.label("Offset");
        let alpha_range = 0..=(self.alphabet.chars().count());
        ui.add(Slider::new(&mut self.offset, alpha_range.clone()));
        ui.add_space(16.0);

        ui.label("Wheels");
        for wheel in &self.wheels {
            ui.add(egui::Label::new(egui::RichText::from(wheel).monospace()));
        }

        ui.horizontal(|ui| {
            if ui.button("+").clicked() {
                self.add_wheel(rng)
            }
            if ui.button("-").clicked() {
                self.del_wheel()
            }
        });

    }
}