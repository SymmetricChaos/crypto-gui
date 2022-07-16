use crate::ciphers::polyalphabetic::Alberti;

use super::{View, ViewableCipher, _generic_components::*};
use eframe::egui::{Slider, Ui};

impl ViewableCipher for Alberti {}

impl View for Alberti {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        randomize_reset(ui, self);
        ui.add_space(16.0);

        ui.label("Fixed Alphabet");
        if control_string(ui, &mut self.fixed_alphabet_string).changed() {
            self.set_fixed_alphabet()
        }

        ui.label("Moving Alphabet");
        if control_string(ui, &mut self.moving_alphabet_string).changed() {
            self.set_moving_alphabet()
        }

        ui.label(mono(&self));

        ui.label("Index");
        let alpha_range = 0..=(self.alphabet_len() - 1);
        ui.add(Slider::new(&mut self.start_index, alpha_range.clone()));
        ui.add_space(16.0);
    }
}
