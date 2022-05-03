use eframe::egui::Ui;
use rand::prelude::StdRng;
use super::View;
use super::generic_components::*;
use crate::ciphers::Chaocipher;
use crate::egui_aux::mono;


impl View for Chaocipher {
    fn ui(&mut self, ui: &mut Ui, rng: &mut StdRng, _errors: &mut String) {

        randomize_reset(ui, self, rng);
        ui.add_space(16.0);

        mono(ui, &self.left.to_string(), None);
        mono(ui, &self.right.to_string(), None);
    }
}
