use super::CipherFrame;
use crate::ui_elements::UiElements;
use ciphers::machines::hebern::Hebern;
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
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/tree/master/ciphers/src/machines/hebern",
        );
        ui.add_space(8.0);

        ui.randomize_reset_cipher(self);
        ui.add_space(8.0);

        ui.subheading("Alphabet");
        if ui.control_string(&mut self.alphabet_string).changed() {
            self.cipher.set_alphabet(&self.alphabet_string);
            for rotor in self.cipher.rotors.rotors.iter_mut() {
                rotor.wiring_str = keyed_alphabet(&mut rotor.wiring_str, &mut self.alphabet_string);
                _ = rotor.set(&self.cipher.alphabet);
            }
        }

        ui.add_space(8.0);
        ui.horizontal(|ui| {
            ui.subheading("Rotors");
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
                if ui.control_string(&mut rotor.wiring_str).lost_focus() {
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

        ui.add_space(16.0);
        if ui.button("Advance Rotors").clicked() {
            self.cipher.rotors.step()
        }
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        let max = self.alphabet_string.chars().count() - 1;
        for rotor in self.cipher.rotors.rotors.iter_mut() {
            rotor.wiring_str = shuffled_str(&self.alphabet_string, &mut rng);
            rotor.position = rng.gen_range(0..max);
        }
    }

    crate::simple_cipher! {}
}
