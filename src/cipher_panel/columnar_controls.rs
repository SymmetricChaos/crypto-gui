use crate::ciphers::transposition::Columnar;

use super::{View, ViewableCipher, _generic_components::*};
use eframe::egui::Ui;

impl ViewableCipher for Columnar {}

impl View for Columnar {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        randomize_reset(ui, self);
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
