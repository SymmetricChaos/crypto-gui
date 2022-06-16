use crate::{ciphers::substitution::Plugboard, egui_aux::mono_strong};

use super::{View, generic_components::control_string};
use eframe::egui::Ui;
use rand::prelude::StdRng;

impl View for Plugboard {
    fn ui(&mut self, ui: &mut Ui, _rng: &mut StdRng, _errors: &mut String) {
  
        ui.add_space(16.0);
        ui.label("Plugboard Pairs");
        ui.horizontal(|ui| {
            if control_string(ui, &mut self.pairs).changed() {
                self.set_plugboard_silent();
            }
        });
 
        let nrows = 8;
        let ncols = 8;
        ui.columns(ncols, |columns| {
            let mut ctr = 0;
            let mut col = 0;
            for pair in self.show_settings() {
                mono_strong(&mut columns[col], &pair, None);
                ctr += 1;
                if ctr % nrows == 0 {
                    col += 1
                }
            }
        });
    }
}
 