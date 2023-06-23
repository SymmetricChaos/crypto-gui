use super::CipherFrame;
use crate::ui_elements::control_string;
use ciphers::{hebern::Hebern, Cipher};
use egui::{Slider, Ui};
use utils::{functions::keyed_alphabet, preset_alphabet::Alphabet};

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
            self.cipher.set_alphabet(&self.alphabet_string);
            for rotor in self.cipher.rotors.rotors.iter_mut() {
                rotor.wiring_str = keyed_alphabet(&mut rotor.wiring_str, &mut self.alphabet_string);
                _ = rotor.set(&self.cipher.alphabet);
            }
        }

        ui.add_space(10.0);
        ui.label("Rotor Wiring");
        for rotor in self.cipher.rotors.rotors.iter_mut() {
            ui.horizontal(|ui| {
                if control_string(ui, &mut rotor.wiring_str).lost_focus() {
                    rotor.wiring_str =
                        keyed_alphabet(&mut rotor.wiring_str, &mut self.alphabet_string);
                    _ = rotor.set(&self.cipher.alphabet);
                }
            });
        }

        if ui.small_button("+").clicked() {
            if self.cipher.rotors.rotors.len() <= 8 {
                self.cipher.rotors.add_rotor(&self.cipher.alphabet);
            }
        }
        if ui.small_button("-").clicked() {
            if self.cipher.rotors.rotors.len() >= 2 {
                self.cipher.rotors.del_rotor();
            }
        }

        ui.add_space(10.0);
        ui.label("Rotor Positions\nTo Be Changed Every Message");
        for rotor in &mut self.cipher.rotors.rotors {
            ui.add(
                Slider::new(
                    &mut rotor.position,
                    0..=self.alphabet_string.chars().count(),
                )
                .clamp_to_range(true),
            );
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
