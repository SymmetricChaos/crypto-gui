use eframe::egui::Slider;
use eframe::egui::TextEdit;
use eframe::egui::TextStyle;
use eframe::egui::Ui;
use rand::prelude::StdRng;
use super::View;
use super::generic_components::*;
use crate::ciphers::StraddlingCheckerboard;
use crate::egui_aux::mono;


impl View for StraddlingCheckerboard {
    fn ui(&mut self, ui: &mut Ui, rng: &mut StdRng, _errors: &mut String) {

        randomize_reset(ui, self, rng);

        ui.add(TextEdit::singleline(&mut self.alphabet).font(TextStyle::Monospace));
        
        ui.add_space(16.0);
        let gap0 = 0..=(self.gaps.1-1);
        let gap1 = (self.gaps.0+1)..=9;
        ui.label("First Gap");
        ui.add(Slider::new(&mut self.gaps.0, gap0));
        ui.label("Second Gap");
        ui.add(Slider::new(&mut self.gaps.1, gap1));

        ui.add_space(16.0);
        mono(ui, &self.cipher_page(), Some(15.0));
    }
}
