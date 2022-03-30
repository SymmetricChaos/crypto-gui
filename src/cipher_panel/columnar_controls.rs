use eframe::egui::{TextEdit, Ui};
use rand::prelude::StdRng;
use super::View;
use super::generic_components::*;
use crate::ciphers::Columnar;


impl View for Columnar {
    fn ui(&mut self, ui: &mut Ui, rng: &mut StdRng) {

        randomize_reset(ui, self, rng);
        ui.add_space(16.0);

        ui.add_space(16.0);
        input_alphabet(ui, self);
        ui.add_space(16.0);

        ui.label("Key Word");
        ui.add(TextEdit::singleline(self.control_key()));
    }
}
