use eframe::egui::{RichText, TextEdit, Ui};
use rand::prelude::StdRng;

use super::View;
use super::generic_components::*;
use crate::ciphers::B64;


impl View for B64 {
    fn ui(&mut self, ui: &mut Ui, rng: &mut StdRng, _errors: &mut String) {

        randomize_reset(ui, self, rng);
        ui.add_space(16.0);

        ui.label("Polybius Key Word");
        if control_string(ui, &mut self.polybius.key_word).changed() {
            self.polybius.set_key()
        }
        ui.add_space(16.0);

        ui.label(RichText::new(format!("Grid\n{}",self.polybius)).monospace());
        ui.add_space(16.0);

        ui.label("First Columnar Key Word");
        ui.add(TextEdit::singleline(self.columnar2.control_key()));

        ui.label("Second Columnar Key Word");
        ui.add(TextEdit::singleline(self.columnar1.control_key()));
    }
}
