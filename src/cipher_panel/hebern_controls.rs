use super::CipherFrame;
use crate::ui_elements::{control_string, randomize_reset};
use ciphers::{machines::hebern::Hebern, Cipher};
use egui::{Slider, Ui};
use rand::{thread_rng, Rng};
use utils::{
    preset_alphabet::Alphabet,
    text_functions::{keyed_alphabet, shuffled_str},
};

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
        randomize_reset(ui, self);
        ui.add_space(8.0);
        ui.label("Alphabet");
        if control_string(ui, &mut self.alphabet_string).changed() {
            self.cipher.set_alphabet(&self.alphabet_string);
            for rotor in self.cipher.rotors.rotors.iter_mut() {
                rotor.wiring_str = keyed_alphabet(&mut rotor.wiring_str, &mut self.alphabet_string);
                _ = rotor.set(&self.cipher.alphabet);
            }
        }

        ui.add_space(8.0);
        ui.horizontal(|ui| {
            ui.label("Rotors");
            ui.separator();
            if ui.button("randomize wiring").clicked() {
                let mut rng = thread_rng();
                for rotor in self.cipher.rotors.rotors.iter_mut() {
                    rotor.wiring_str = shuffled_str(&self.alphabet_string, &mut rng)
                }
            }
            ui.separator();
            if ui.button("randomize positions").clicked() {
                let mut rng = thread_rng();
                let max = self.alphabet_string.chars().count() - 1;
                for rotor in self.cipher.rotors.rotors.iter_mut() {
                    rotor.position = rng.gen_range(0..max);
                }
            }
        });
        ui.add_space(4.0);

        ui.horizontal(|ui| {
            if ui.small_button("+").clicked() {
                if self.cipher.rotors.rotors.len() <= 8 {
                    self.cipher.rotors.add_rotor(&self.cipher.alphabet);
                }
            }
            if ui.small_button("–").clicked() {
                if self.cipher.rotors.rotors.len() >= 2 {
                    self.cipher.rotors.del_rotor();
                }
            }
        });
        ui.add_space(4.0);

        for rotor in self.cipher.rotors.rotors.iter_mut() {
            ui.horizontal(|ui| {
                if control_string(ui, &mut rotor.wiring_str).lost_focus() {
                    rotor.wiring_str =
                        keyed_alphabet(&mut rotor.wiring_str, &mut self.alphabet_string);
                    _ = rotor.set(&self.cipher.alphabet);
                }
                ui.add(Slider::new(
                    &mut rotor.position,
                    0..=self.alphabet_string.chars().count() - 1,
                ));
            });
        }
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        let max = self.alphabet_string.chars().count() - 1;
        for rotor in self.cipher.rotors.rotors.iter_mut() {
            rotor.wiring_str = shuffled_str(&self.alphabet_string, &mut rng);
            rotor.position = rng.gen_range(0..max);
        }
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
