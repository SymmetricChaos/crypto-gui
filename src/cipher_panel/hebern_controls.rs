use super::View;
use eframe::egui::{ComboBox, Label, RichText, Slider, TextEdit, TextStyle, Ui};
use rand::prelude::StdRng;

impl View for Heberm {
    fn ui(&mut self, ui: &mut Ui, _rng: &mut StdRng, _errors: &mut String) {
        ui.label("Rotor Positions\nTo Be Changed Every Message");
        for rotor in &mut self.state.rotors {
            ui.add(Slider::new(&mut rotor.position, 0..=26).clamp_to_range(true));
        }

        ui.add_space(10.0);
        ui.label("Rotors");
        for rotor in &mut self.state.rotors {
            ui.horizontal(|ui| {
                let name = RichText::new(rotor.name).monospace();
                ui.add_sized([20.0, 20.0], Label::new(name));
                let characters = RichText::new(&rotor.to_string()).monospace();
                ui.label(characters);
            });
        }

    }
}
