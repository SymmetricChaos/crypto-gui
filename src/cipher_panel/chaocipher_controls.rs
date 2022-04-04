use eframe::egui::{TextEdit, Ui};
use rand::prelude::StdRng;
use super::View;
use super::generic_components::*;
use crate::ciphers::Chaocipher;


impl View for Chaocipher {
    fn ui(&mut self, ui: &mut Ui, rng: &mut StdRng) {

        randomize_reset(ui, self, rng);
        ui.add_space(16.0);

        ui.label(self.left.to_string());
        ui.label(self.right.to_string());
    }
}
