use eframe::egui::Ui;

impl ViewableCipher for PolybiusCube {}

use super::{View, ViewableCipher, _generic_components::*};
use crate::{ciphers::polybius::PolybiusCube, egui_aux::mono};

impl View for PolybiusCube {
    fn ui(&mut self, ui: &mut Ui, errors: &mut String) {
        randomize_reset(ui, self);
        ui.add_space(16.0);

        ui.add_space(16.0);
        ui.label("Alphabet");
        if control_string(ui, &mut self.alphabet_string).changed() {
            match self.set_alphabet() {
                Ok(_) => (),
                Err(e) => *errors = e.to_string(),
            }
        }

        ui.add_space(16.0);
        ui.label("Key Word");
        if control_string(ui, &mut self.key_word).changed() {
            self.set_key()
        }

        ui.add_space(16.0);
        ui.label("Labels");
        if control_string(ui, &mut self.labels_string).changed() {
            self.set_labels();
        }

        ui.add_space(16.0);
        ui.label("Grid");
        let grids = self.show_grids();
        ui.horizontal(|ui| {
            mono(ui, &grids[0], None);
            mono(ui, &grids[1], None);
            mono(ui, &grids[2], None);
        });
    }
}
