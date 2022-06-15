use crate::ciphers::substitution::Plugboard;

use super::View;
use eframe::egui::{TextEdit, TextStyle, Ui};
use rand::prelude::StdRng;

impl View for Plugboard {
    fn ui(&mut self, ui: &mut Ui, _rng: &mut StdRng, _errors: &mut String) {
        
        ui.add_space(10.0);
        ui.label("Plugboard Pairs");
        ui.add(TextEdit::singleline(&mut self.pairs).font(TextStyle::Monospace));
    }

}
