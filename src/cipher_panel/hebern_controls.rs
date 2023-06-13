use super::CipherFrame;
use crate::ui_elements::control_string;
use ciphers::{hebern::Hebern, Cipher};
use egui::{Slider, TextEdit, Ui};
use utils::preset_alphabet::Alphabet;

pub struct HebernFrame {
    cipher: Hebern,
    alphabet_string: String,
}

impl Default for HebernFrame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
            alphabet_string: Alphabet::BasicLatin.into(),
        }
    }
}

impl CipherFrame for HebernFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        ui.add_space(10.0);
        ui.label("Alphabet");
        if control_string(ui, &mut self.alphabet_string).changed() {
            self.cipher.set_alphabet(&self.alphabet_string)
        }

        ui.add_space(10.0);
        ui.label("Rotor Wiring");
        for rotor in self.cipher.rotors.rotors.iter_mut() {
            ui.horizontal(|ui| {
                ui.add_enabled(rotor.editable, TextEdit::singleline(&mut rotor.wiring_str));
                if rotor.editable {
                    if ui.small_button("save").clicked() {
                        match rotor.set(&self.cipher.alphabet) {
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
                // if ui.small_button("random").clicked() {
                //     match rotor.randomize(&self.cipher.alphabet) {
                //         Ok(_) => rotor.error.clear(),
                //         Err(e) => rotor.error = e.inner(),
                //     }
                // }
                if ui.small_button("fill").clicked() {
                    match rotor.fill(&self.cipher.alphabet) {
                        Ok(_) => rotor.error.clear(),
                        Err(e) => rotor.error = e.inner(),
                    }
                }
                ui.label(&rotor.error);
            });
        }

        ui.add_space(10.0);
        ui.label("Rotor Positions\nTo Be Changed Every Message");
        for rotor in &mut self.cipher.rotors.rotors {
            ui.add(Slider::new(&mut rotor.position, 0..=26).clamp_to_range(true));
        }
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {}

    fn reset(&mut self) {
        *self = Self::default()
    }
}
