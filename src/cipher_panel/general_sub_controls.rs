use super::{generic_components::*, View};
use crate::ciphers::GeneralSubstitution;
use eframe::egui::Ui;
use rand::prelude::StdRng;

impl View for GeneralSubstitution {
    fn ui(&mut self, ui: &mut Ui, rng: &mut StdRng, _errors: &mut String) {
        randomize_reset(ui, self, rng);
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
