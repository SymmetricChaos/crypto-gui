use eframe::egui::{RichText, TextEdit, Slider, ComboBox, Ui, Label};
use super::View;
use crate::ciphers::{EnigmaM3, enigma::Rotor};

// fn rotor_selector(ciphers: &[Rotor], identifier: &'static str, ui: &mut Ui) {
//     ComboBox::from_id_source(identifier)
//         .selected_text(identifier)
//         .show_ui(ui, |ui| {
//             for id in ciphers {
//                 ui.selectable_value(active_cipher, *id, format!("{}",id));
//             }
//         });
//     ui.add_space(10.0);
// }

impl View for EnigmaM3 {
    fn ui(&mut self, ui: &mut eframe::egui::Ui) {
 
        ui.label("Rotor Positions\nTo Be Changed Every Message");
        for rotor in &mut self.state.rotors {
            ui.add(Slider::new(&mut rotor.position, 0..=26)
                .clamp_to_range(true)
            );
        };
 
        ui.label("Ring Settings").on_hover_text("Ringstellung");
        for rotor in &mut self.state.rotors {
            ui.add(Slider::new(&mut rotor.ring, 0..=26)
                .clamp_to_range(true)
            );
        };
 
        // ui.label("Set Reflector Position")
        // ui.add(Slider::new(&mut self.state.reflector.position, 0..=26)
        //     .show_value(false)
        //     .clamp_to_range(true)
        // );
 
        ui.label("Select Rotors");
        ui.label("THREE COMBO BOXES HERE");
 
        ui.label("Rotors").on_hover_text("Walzen");;
        for rotor in &mut self.state.rotors {
            ui.horizontal(|ui| {
                let name = RichText::new(rotor.name).monospace();
                ui.add_sized([20.0,20.0],Label::new(name));
                let characters = RichText::new(&rotor.to_string()).monospace();
                ui.label(characters);
 
            });
        }
 
        ui.label("Select Reflector");
        ui.label("COMBO BOX HERE");
 
        ui.add_space(10.0);
        ui.label("Reflector").on_hover_text("Umkehrwalze");
        ui.horizontal(|ui| {
            let name = RichText::new(self.state.reflector.name).monospace();
            ui.add_sized([20.0,20.0],Label::new(name));
            let text = RichText::new(&self.state.reflector.to_string()).monospace();
            ui.label(text);
        });
 
        ui.add_space(10.0);
        ui.label("Plugboard").on_hover_text("Steckerbrett");
        ui.label("SOME INTERFACE HERE");
 
    }
}