use super::{View, ViewableCipher};
use crate::ciphers::Purple;
use eframe::egui::{Slider, TextEdit, TextStyle, Ui};

impl ViewableCipher for Purple {}

impl View for Purple {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        ui.label("Sixes Position\nTo Be Changed Every Message");
        ui.add(Slider::new(&mut self.switches.sixes.position, 0..=24).clamp_to_range(true));


        ui.label("Twenties Positions\nTo Be Changed Every Message");
        for switch in self.switches.twenties.iter_mut() {
            ui.add(Slider::new(&mut switch.position, 0..=24).clamp_to_range(true));
        }
        
        ui.label("Select Twenties Speeds");
        // exclusive selections for speeds


        ui.add_space(10.0);
        ui.label("Plugboard");
        if ui.add(TextEdit::singleline(&mut self.plugboard_string).font(TextStyle::Monospace)).changed() {
            // convert plugs description to hashmaps
        };
    }
}
