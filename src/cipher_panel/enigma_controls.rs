use super::{View, ViewableCipher};
use crate::ciphers::{
    enigma::{REFLECTORS, ROTOR_VEC},
    EnigmaM3,
};
use eframe::egui::{ComboBox, Label, RichText, Slider, TextEdit, TextStyle, Ui};

impl ViewableCipher for EnigmaM3 {}

impl View for EnigmaM3 {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        ui.label("Rotor Positions\nTo Be Changed Every Message");
        for rotor in &mut self.state.rotors {
            ui.add(Slider::new(&mut rotor.position, 0..=26).clamp_to_range(true));
        }

        ui.add_space(10.0);
        ui.label("Ring Settings").on_hover_text("Ringstellung");
        for rotor in &mut self.state.rotors {
            ui.add(Slider::new(&mut rotor.ring, 0..=26).clamp_to_range(true));
        }

        ui.add_space(10.0);
        ui.label("Select Rotors");

        for i in 0..3 {
            ComboBox::from_id_source(format!("Rotor {}", i + 1))
                .selected_text(format!("Rotor {}", i + 1))
                .show_ui(ui, |ui| {
                    for rtr in ROTOR_VEC.iter() {
                        ui.selectable_value(&mut self.state.rotors[i], *rtr, rtr.name.to_string());
                    }
                });
        }

        ui.add_space(10.0);
        ui.label("Rotors").on_hover_text("Walzen");
        for rotor in &mut self.state.rotors {
            ui.horizontal(|ui| {
                let name = RichText::new(rotor.name).monospace();
                ui.add_sized([20.0, 20.0], Label::new(name));
                let characters = RichText::new(&rotor.to_string()).monospace();
                ui.label(characters);
            });
        }

        ui.add_space(10.0);
        ComboBox::from_id_source("Reflector")
            .selected_text("Select Reflector")
            .show_ui(ui, |ui| {
                for rfl in REFLECTORS.values() {
                    ui.selectable_value(&mut self.state.reflector, *rfl, format!("{}", rfl.name));
                }
            });

        ui.add_space(10.0);
        ui.label("Reflector").on_hover_text("Umkehrwalze");
        ui.horizontal(|ui| {
            let name = RichText::new(self.state.reflector.name).monospace();
            ui.add_sized([20.0, 20.0], Label::new(name));
            let text = RichText::new(&self.state.reflector.to_string()).monospace();
            ui.label(text);
        });

        ui.add_space(10.0);
        ui.label("Plugboard").on_hover_text("Steckerbrett");
        ui.add(TextEdit::singleline(&mut self.state.plugboard_pairs).font(TextStyle::Monospace));
    }
}
