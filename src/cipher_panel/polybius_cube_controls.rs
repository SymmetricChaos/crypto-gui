use eframe::egui::{Color32, RichText, Ui};
use rand::prelude::StdRng;

use super::{generic_components::*, View};
use crate::{ciphers::polybius::PolybiusCube, egui_aux::mono};

impl View for PolybiusCube {
    fn ui(&mut self, ui: &mut Ui, rng: &mut StdRng, _errors: &mut String) {
        randomize_reset(ui, self, rng);
        ui.add_space(16.0);

        ui.add_space(10.0);
        ui.label(
            RichText::new(self.alphabet())
                .monospace()
                .background_color(Color32::BLACK),
        );
        ui.add_space(16.0);

        ui.label("Key Word");
        if control_string(ui, &mut self.key_word).changed() {
            self.set_key()
        }
        ui.add_space(16.0);

        ui.label("Labels");
        if control_string(ui, &mut self.labels_string).changed()  {
            self.set_labels();
        }

        ui.label("Grid");
        let grids = self.show_grids();
        ui.horizontal( |ui| {
            mono(ui, &grids[0], None);
            mono(ui, &grids[1], None);
            mono(ui, &grids[2], None);
        });
        
        
        ui.add_space(16.0);
    }
}
