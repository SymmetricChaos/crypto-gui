use crate::ciphers::transposition::Columnar;

use super::{generic_components::*, View};
use eframe::egui::Ui;
use rand::prelude::StdRng;

impl View for Columnar {
    fn ui(&mut self, ui: &mut Ui, rng: &mut StdRng, _errors: &mut String) {
        randomize_reset(ui, self, rng);
        ui.add_space(16.0);

        ui.label("Alphabet");
        if control_string(ui, &mut self.alphabet_string).changed() {
            self.set_alphabet()
        }

        ui.label("Key Word");
        if control_string(ui, &mut self.key_word).changed() {
            self.set_key()
        };
    }
}
