use super::{generic_components::*, View};
use crate::ciphers::polyalphabetic::Chaocipher;
use eframe::egui::Ui;
use rand::prelude::StdRng;

impl View for Chaocipher {
    fn ui(&mut self, ui: &mut Ui, rng: &mut StdRng, _errors: &mut String) {
        randomize_reset(ui, self, rng);
        ui.add_space(16.0);

        if control_string(ui, &mut self.left_string).changed() {
            self.set_left()
        }

        if control_string(ui, &mut self.right_string).changed() {
            self.set_right()
        }
    }
}
