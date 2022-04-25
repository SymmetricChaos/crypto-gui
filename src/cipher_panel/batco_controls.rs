use eframe::egui::Ui;
use eframe::egui::Slider;
use rand::prelude::StdRng;
use super::View;
use super::generic_components::*;
use crate::ciphers::Batco;

impl View for Batco {
    fn ui(&mut self, ui: &mut Ui, rng: &mut StdRng, _errors: &mut String) {

        randomize_reset(ui, self, rng);
        ui.add_space(16.0);

        if ui.button("Use Seed").clicked() {
            if self.seed.is_none() {
                self.seed = Some(0)
            } else {
                self.seed = None
            }
        }

        if self.seed.is_some() {
            ui.add(Slider::new(&mut self.seed.unwrap(), 0..=u64::MAX));
        }

        ui.label(self.show_code_page());
    }
}
