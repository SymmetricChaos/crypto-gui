use eframe::egui::RichText;
use eframe::egui::{TextEdit, Slider};
use super::View;
use super::generic_components::*;
use crate::ciphers::EnigmaM3;

impl View for EnigmaM3 {
    fn ui(&mut self, ui: &mut eframe::egui::Ui) {

        ui.label("Select Rotors");

        ui.add_space(10.0);
        ui.label("Select Reflector");

        ui.add_space(10.0);
        ui.label("Rotors");
        for rotor in &mut self.state.rotors {
            ui.horizontal(|ui| {
                let text = RichText::new(&rotor.to_string()).monospace();
                ui.label(text);
                ui.add(Slider::new(&mut rotor.position, 0..=26)
                    .clamp_to_range(true)
                );
            });
        }

        ui.add_space(10.0);
        ui.label("Reflector");
        ui.horizontal(|ui| {
            // ui.add(Slider::new(&mut self.state.reflector.position, 0..=26)
            //     .show_value(false)
            //     .clamp_to_range(true)
            // );
            let text = RichText::new(&self.state.reflector.to_string()).monospace();
            ui.label(text);
        });
        
        ui.add_space(10.0);
        ui.label("Plugboard");

    }
}
