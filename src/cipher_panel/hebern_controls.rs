
use crate::ciphers::hebern::Hebern;

use super::{View, ViewableCipher, generic_components::control_string};
use eframe::egui::{Slider, TextEdit, Ui};

impl ViewableCipher for Hebern {}

impl View for Hebern {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {

        ui.add_space(10.0);
        ui.label("Alphabet");
        if control_string(ui, &mut self.rotors.alphabet_string).changed() {
            self.rotors.set_alphabet()
        }

        ui.add_space(10.0);
        ui.label("Rotor Wiring");
        for rotor in self.rotors.rotors.iter_mut() {
            ui.horizontal(|ui| {
                ui.add_enabled(rotor.editable, TextEdit::singleline(&mut rotor.wiring_str));
                if rotor.editable {
                    if ui.button("save").clicked() {
                        match rotor.set(&self.rotors.alphabet) {
                            Ok(_) => { rotor.editable = false; rotor.error.clear(); },
                            Err(e) => rotor.error = e.inner(),
                        }
                    }
                } else {
                    if ui.button("edit").clicked() {
                        rotor.editable = true;
                    }
                }
                if ui.button("random").clicked() {
                    rotor.randomize(&self.rotors.alphabet)
                }
                ui.label(&rotor.error);
            });
        };

        ui.add_space(10.0);
        ui.label("Rotor Positions\nTo Be Changed Every Message");
        for rotor in &mut self.rotors.rotors {
            ui.add(Slider::new(&mut rotor.position, 0..=26).clamp_to_range(true));
        }


    }
}
