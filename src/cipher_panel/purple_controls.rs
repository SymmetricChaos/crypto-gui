use super::{View, ViewableCipher};
use crate::ciphers::{Purple, purple::switch::SwitchSpeed};
use eframe::egui::{Slider, TextEdit, TextStyle, Ui};

impl ViewableCipher for Purple {}

impl View for Purple {
    fn ui(&mut self, ui: &mut Ui, errors: &mut String) {
        ui.label("Sixes Position\nTo Be Changed Every Message");
        ui.add(Slider::new(&mut self.switches.sixes.position, 0..=24).clamp_to_range(true));


        ui.label("Twenties Positions\nTo Be Changed Every Message");
        for switch in self.switches.twenties.iter_mut() {
            ui.add(Slider::new(&mut switch.position, 0..=24).clamp_to_range(true));
        }
        
        // TODO: Selections must be exclusive
        ui.label("Select Twenties Speeds");
        for switch in self.switches.twenties.iter_mut() {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut switch.speed, SwitchSpeed::Fast, "Fast");
                ui.selectable_value(&mut switch.speed, SwitchSpeed::Middle, "Middle");
                ui.selectable_value(&mut switch.speed, SwitchSpeed::Slow, "Slow");
            });
        }
        
        ui.add_space(10.0);
        ui.label("Plugboard");
        if ui.add(TextEdit::singleline(&mut self.plugboard_string).font(TextStyle::Monospace)).changed() {
            match self.set_plugboard() {
                Ok(_) => (),
                Err(e) => *errors = e.to_string(),
            }
        };
    }
}
