use crate::ciphers::substitution::Caesar;

use super::{View, ViewableCipher, _generic_components::*};
use eframe::egui::{Slider, Ui};

impl ViewableCipher for Caesar {}

impl View for Caesar {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        randomize_reset(ui, self);
        ui.add_space(16.0);

        ui.label("Alphabet");
        if control_string(ui, &mut self.alphabet_string).changed() {
            self.set_alphabet()
        }
        ui.add_space(16.0);

        ui.label("Key");
        let alpha_range = 0..=(self.alphabet.len() as i32 - 1);
        ui.add(Slider::new(&mut self.shift, alpha_range));
        ui.add_space(16.0);
    }
}
