use super::{View, ViewableCipher, _generic_components::*};
use crate::ciphers::substitution::GeneralSubstitution;
use eframe::egui::Ui;

impl ViewableCipher for GeneralSubstitution {}

impl View for GeneralSubstitution {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        randomize_reset(ui, self);
        ui.add_space(16.0);

        ui.label("Plaintext Alphabet");
        if control_string(ui, &mut self.pt_alphabet_string).changed() {
            self.set_pt_alphabet();
        }
        ui.add_space(16.0);

        ui.label("Ciphertext Alphabet");
        if control_string(ui, &mut self.ct_alphabet_string).changed() {
            self.set_pt_alphabet();
        }
        ui.add_space(16.0);
    }
}
