use super::CipherFrame;
use crate::ui_elements::{mono, UiElements};
use ciphers::{
    machines::enigma::{rotors::REFLECTOR_VEC, EnigmaM3, REFLECTOR_MAP, ROTOR_VEC},
    Cipher,
};
use egui::{ComboBox, Label, Slider, Ui};
use rand::{thread_rng, Rng};
use utils::{preset_alphabet::Alphabet, text_functions::random_sample};

#[derive(Default)]
pub struct EnigmaM3Frame {
    cipher: EnigmaM3,
    plugboard_string: String,
}

impl EnigmaM3Frame {
    fn randomize_plugboard(&mut self) {
        let alpha = random_sample(Alphabet::BasicLatin.slice(), 14, &mut thread_rng());
        let mut cs = alpha.chars();
        self.plugboard_string.clear();
        for _ in 0..7 {
            self.plugboard_string.push(cs.next().unwrap());
            self.plugboard_string.push(cs.next().unwrap());
            self.plugboard_string.push(' ');
        }

        self.cipher
            .state
            .plugboard
            .set_plugboard(&self.plugboard_string)
            .expect("error randomly generating plugboard")
    }

    fn randomize_rotors(&mut self) {
        self.cipher.state.reflector = REFLECTOR_VEC[thread_rng().gen_range(0..REFLECTOR_VEC.len())];
        for rotor in self.cipher.state.rotors.iter_mut() {
            *rotor = ROTOR_VEC[thread_rng().gen_range(0..ROTOR_VEC.len())];
            rotor.position = thread_rng().gen_range(0..26);
            rotor.ring = thread_rng().gen_range(0..26);
        }
    }
}

impl CipherFrame for EnigmaM3Frame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        ui.randomize_reset(self);

        ui.label("Rotor Positions\nTo Be Changed Every Message");
        for rotor in &mut self.cipher.state.rotors {
            ui.add(Slider::new(&mut rotor.position, 0..=26).clamp_to_range(true));
        }

        ui.add_space(10.0);
        ui.label("Ring Settings").on_hover_text("Ringstellung");
        for rotor in &mut self.cipher.state.rotors {
            ui.add(Slider::new(&mut rotor.ring, 0..=26).clamp_to_range(true));
        }

        ui.add_space(10.0);
        ui.label("Select Rotors");

        for i in 0..3 {
            ComboBox::from_id_source(format!("Rotor {}", i + 1))
                .selected_text(format!("Rotor {}", i + 1))
                .show_ui(ui, |ui| {
                    for rtr in ROTOR_VEC.iter() {
                        ui.selectable_value(
                            &mut self.cipher.state.rotors[i],
                            *rtr,
                            rtr.name.to_string(),
                        );
                    }
                });
        }

        ui.add_space(10.0);
        ui.label("Rotors").on_hover_text("Walzen");
        for rotor in &mut self.cipher.state.rotors {
            ui.horizontal(|ui| {
                ui.add_sized([20.0, 20.0], Label::new(mono(rotor.name)));
                ui.mono(rotor);
            });
        }

        ui.add_space(10.0);
        ComboBox::from_id_source("Reflector")
            .selected_text("Select Reflector")
            .show_ui(ui, |ui| {
                for rfl in REFLECTOR_MAP.values() {
                    ui.selectable_value(
                        &mut self.cipher.state.reflector,
                        *rfl,
                        format!("{}", rfl.name),
                    );
                }
            });

        ui.add_space(10.0);
        ui.label("Reflector").on_hover_text("Umkehrwalze");
        ui.horizontal(|ui| {
            ui.add_sized(
                [20.0, 20.0],
                Label::new(mono(self.cipher.state.reflector.name)),
            );
            ui.mono(self.cipher.state.reflector);
        });

        ui.add_space(10.0);
        ui.label("Plugboard").on_hover_text("Steckerbrett");
        if ui.control_string(&mut self.plugboard_string).changed() {
            match self.cipher.state.set_plugboard(&self.plugboard_string) {
                Ok(_) => (),
                Err(e) => {
                    ui.error_text(&e.inner());
                }
            }
        };
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        self.randomize_plugboard();
        self.randomize_rotors();
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
