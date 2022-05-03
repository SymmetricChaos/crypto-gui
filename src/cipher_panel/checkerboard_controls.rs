use eframe::egui::{DragValue, TextEdit, TextStyle, Ui};
use rand::prelude::StdRng;
use super::View;
use super::generic_components::*;
use crate::ciphers::StraddlingCheckerboard;
use crate::egui_aux::mono;


impl View for StraddlingCheckerboard {
    fn ui(&mut self, ui: &mut Ui, rng: &mut StdRng, _errors: &mut String) {

        randomize_reset(ui, self, rng);

        ui.add_space(16.0);
        ui.add(TextEdit::singleline(&mut self.alphabet).font(TextStyle::Monospace));
        if ui.button("set alphabet").clicked() {
            self.set_alphabet()
        };
        
        ui.add_space(16.0);
        let gap0 = 0..=(self.gaps.1-1);
        let gap1 = (self.gaps.0+1)..=9;
        ui.horizontal(|ui| {
            mono(ui, "First Gap ", None);
            ui.add(DragValue::new(&mut self.gaps.0).clamp_range(gap0));
        });
        ui.horizontal(|ui| {
            mono(ui, "Second Gap", None);
            ui.add(DragValue::new(&mut self.gaps.1).clamp_range(gap1));
        });

        ui.add_space(16.0);
        mono(ui, &self.cipher_page(), Some(15.0));
    }
}
