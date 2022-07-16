use super::{View, ViewableCipher, _generic_components::*};
use crate::{ciphers::polybius::Trifid, egui_aux::mono};
use eframe::egui::{Slider, Ui};

impl ViewableCipher for Trifid {}

impl View for Trifid {
    fn ui(&mut self, ui: &mut Ui, errors: &mut String) {
        randomize_reset(ui, self);
        ui.add_space(16.0);

        let block_size_range = 3..=30;
        ui.label("Block Size");
        ui.add(Slider::new(&mut self.block_size, block_size_range));

        ui.label("Alphabet");
        if control_string(ui, &mut self.polybius.alphabet_string).changed() {
            match self.polybius.set_alphabet() {
                Ok(_) => (),
                Err(e) => *errors = e.to_string(),
            }
        }

        ui.label("Key Word");
        if control_string(ui, &mut self.polybius.key_word).changed() {
            self.polybius.set_key()
        }
        ui.add_space(16.0);

        ui.label("Grid");
        let grids = self.polybius.show_grids();
        ui.horizontal(|ui| {
            mono(ui, &grids[0], None);
            mono(ui, &grids[1], None);
            mono(ui, &grids[2], None);
        });
    }
}
