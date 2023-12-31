use super::CipherFrame;
use crate::ui_elements::UiElements;
use ciphers::{
    machines::enigma::{rotors::REFLECTOR_VEC, EnigmaM3, REFLECTOR_MAP, ROTOR_VEC},
    Cipher,
};
use egui::{ComboBox, Slider, Ui};
use rand::{thread_rng, Rng};
use utils::{preset_alphabet::Alphabet, text_functions::shuffled_str};

#[derive(Default)]
pub struct EnigmaM3Frame {
    cipher: EnigmaM3,
    plugboard_string: String,
}

impl EnigmaM3Frame {
    fn randomize_plugboard(&mut self) {
        let mut rng = thread_rng();
        let alpha = shuffled_str(Alphabet::BasicLatin.slice(), &mut rng);
        let mut cs = alpha.chars();
        self.plugboard_string.clear();
        for _ in 0..rng.gen_range(6..10) {
            self.plugboard_string.push(cs.next().unwrap());
            self.plugboard_string.push(cs.next().unwrap());
            self.plugboard_string.push(' ');
        }
        self.plugboard_string.pop();

        self.cipher
            .state
            .plugboard
            .set_plugboard(&self.plugboard_string)
            .expect("error randomly generating plugboard")
    }

    fn randomize_rotors(&mut self) {
        for rotor in self.cipher.state.rotors.iter_mut() {
            *rotor = ROTOR_VEC[thread_rng().gen_range(0..ROTOR_VEC.len())];
        }
    }

    fn randomize_reflector(&mut self) {
        self.cipher.state.reflector = REFLECTOR_VEC[thread_rng().gen_range(0..REFLECTOR_VEC.len())];
    }

    fn randomize_positions(&mut self) {
        for rotor in self.cipher.state.rotors.iter_mut() {
            rotor.position = thread_rng().gen_range(0..26);
        }
    }

    fn randomize_rings(&mut self) {
        for rotor in self.cipher.state.rotors.iter_mut() {
            rotor.ring = thread_rng().gen_range(0..26);
        }
    }
}

impl CipherFrame for EnigmaM3Frame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        ui.randomize_reset(self);

        ui.add_space(16.0);
        ui.horizontal(|ui| {
            ui.subheading("Rotor Positions");
            if ui.button("🎲").clicked() {
                self.randomize_positions();
            }
        });
        ui.label("Changed for every message");
        for rotor in &mut self.cipher.state.rotors {
            ui.add(Slider::new(&mut rotor.position, 0..=26).clamp_to_range(true));
        }

        ui.add_space(16.0);
        ui.horizontal(|ui| {
            ui.subheading("Ring Settings (Ringstellung)");
            if ui.button("🎲").clicked() {
                self.randomize_rings();
            }
        });
        ui.label("Changed daily.");
        for rotor in &mut self.cipher.state.rotors {
            ui.add(Slider::new(&mut rotor.ring, 0..=26).clamp_to_range(true));
        }

        ui.add_space(16.0);
        ui.horizontal(|ui| {
            ui.subheading("Rotors (Walzen)");
            if ui.button("🎲").clicked() {
                self.randomize_rotors();
            }
        });
        ui.label("Changed daily.");
        for i in 0..3 {
            ui.horizontal(|ui| {
                ComboBox::from_id_source(format!("Rotor {}", i + 1))
                    .selected_text(self.cipher.state.rotors[i].name)
                    .show_ui(ui, |ui| {
                        for rtr in ROTOR_VEC.iter() {
                            ui.selectable_value(
                                &mut self.cipher.state.rotors[i],
                                *rtr,
                                rtr.name.to_string(),
                            );
                        }
                    });
                ui.mono(self.cipher.state.rotors[i]);
            });
        }

        ui.add_space(16.0);
        ui.horizontal(|ui| {
            ui.subheading("Reflector (Umkehrwalze)");
            if ui.button("🎲").clicked() {
                self.randomize_reflector();
            }
        });
        ui.horizontal(|ui| {
            ComboBox::from_id_source("Reflector")
                .selected_text(self.cipher.state.reflector.name)
                .show_ui(ui, |ui| {
                    for rfl in REFLECTOR_MAP.values() {
                        ui.selectable_value(
                            &mut self.cipher.state.reflector,
                            *rfl,
                            format!("{}", rfl.name),
                        );
                    }
                });
            ui.mono(self.cipher.state.reflector);
        });

        ui.add_space(16.0);
        ui.horizontal(|ui| {
            ui.subheading("Plugboard (Steckerbrett)");
            if ui.button("🎲").clicked() {
                self.randomize_plugboard();
            }
        });
        ui.label("Changed daily.");
        if ui.control_string(&mut self.plugboard_string).changed() {
            match self.cipher.state.set_plugboard(&self.plugboard_string) {
                Ok(_) => (),
                Err(e) => {
                    ui.error_text(&e.inner());
                }
            }
        };
        ui.add_space(16.0);
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        self.randomize_plugboard();
        self.randomize_rotors();
        self.randomize_positions();
        self.randomize_rings();
        self.randomize_reflector();
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
