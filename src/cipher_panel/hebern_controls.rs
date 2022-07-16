use crate::ciphers::hebern::Hebern;

use super::{View, ViewableCipher, _generic_components::control_string};
use eframe::egui::{Slider, TextEdit, Ui};

impl ViewableCipher for Hebern {}

impl View for Hebern {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        ui.add_space(10.0);
        ui.label("Alphabet");
        if control_string(ui, &mut self.alphabet_string).changed() {
            self.set_alphabet()
        }

        ui.add_space(10.0);
        ui.label("Rotor Wiring");
        for rotor in self.rotors.rotors.iter_mut() {
            ui.horizontal(|ui| {
                ui.add_enabled(rotor.editable, TextEdit::singleline(&mut rotor.wiring_str));
                if rotor.editable {
                    if ui.small_button("save").clicked() {
                        match rotor.set(&self.alphabet) {
                            Ok(_) => {
                                rotor.editable = false;
                                rotor.error.clear();
                            }
                            Err(e) => rotor.error = e.inner(),
                        }
                    }
                } else {
                    if ui.small_button("edit").clicked() {
                        rotor.editable = true;
                    }
                }
                if ui.small_button("random").clicked() {
                    match rotor.randomize(&self.alphabet) {
                        Ok(_) => rotor.error.clear(),
                        Err(e) => rotor.error = e.inner(),
                    }
                }
                if ui.small_button("fill").clicked() {
                    match rotor.fill(&self.alphabet) {
                        Ok(_) => rotor.error.clear(),
                        Err(e) => rotor.error = e.inner(),
                    }
                }
                ui.label(&rotor.error);
            });
        }

        ui.add_space(10.0);
        ui.label("Rotor Positions\nTo Be Changed Every Message");
        for rotor in &mut self.rotors.rotors {
            ui.add(Slider::new(&mut rotor.position, 0..=26).clamp_to_range(true));
        }
    }
}
