use eframe::egui::Ui;
use rand::prelude::StdRng;
use super::View;
use super::generic_components::*;
use crate::ciphers::StraddlingCheckerboard;


impl View for StraddlingCheckerboard {
    fn ui(&mut self, ui: &mut Ui, rng: &mut StdRng, _errors: &mut String) {

        randomize_reset(ui, self, rng);
        ui.add_space(16.0);
        ui.label(self.cipher_page());
    }
}
