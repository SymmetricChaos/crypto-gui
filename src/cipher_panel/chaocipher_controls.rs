use super::{View, ViewableCipher, _generic_components::*};
use crate::ciphers::polyalphabetic::Chaocipher;
use eframe::egui::Ui;

impl ViewableCipher for Chaocipher {}

impl View for Chaocipher {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        randomize_reset(ui, self);
        ui.add_space(16.0);

        if control_string(ui, &mut self.left_string).changed() {
            self.set_left()
        }

        if control_string(ui, &mut self.right_string).changed() {
            self.set_right()
        }
    }
}
