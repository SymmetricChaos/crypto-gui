use crate::ciphers::hebern::Hebern;

use super::{View, ViewableCipher};
use eframe::egui::{Slider, TextEdit, Ui};

impl ViewableCipher for Hebern {}

impl View for Hebern {
    fn ui(&mut self, ui: &mut Ui, errors: &mut String) {
        ui.label("Rotor Positions\nTo Be Changed Every Message");
        for rotor in &mut self.rotors.rotors {
            ui.add(Slider::new(&mut rotor.position, 0..=26).clamp_to_range(true));
        }

        for rotor in self.rotors.rotors.iter_mut() {
            ui.horizontal(|ui| {
                ui.add_enabled(rotor.editable, TextEdit::singleline(&mut rotor.wiring_str));
                if rotor.editable {
                    if ui.button("save").clicked() {
                        match rotor.set(&self.rotors.alphabet) {
                            Ok(_) => { rotor.editable = false },
                            Err(_) => *errors = "unable to build rotor".to_string(),
                        }
                    }
                } else {
                    if ui.button("edit").clicked() {
                        rotor.editable = true;
                    }
                }
            });
        };


    }
}
