use ciphers::{
    enigma::{REFLECTORS, ROTOR_VEC},
    Cipher, EnigmaM3,
};
use egui::{ComboBox, Label, Slider, Ui};

use crate::egui_aux::{error_text, mono};

use super::{CipherFrame, _generic_components::control_string};

#[derive(Default)]
pub struct EnigmaM3Frame {
    cipher: EnigmaM3,
    plugboard_string: String,
}

impl CipherFrame for EnigmaM3Frame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
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
                ui.label(mono(rotor));
            });
        }

        ui.add_space(10.0);
        ComboBox::from_id_source("Reflector")
            .selected_text("Select Reflector")
            .show_ui(ui, |ui| {
                for rfl in REFLECTORS.values() {
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
            ui.label(mono(self.cipher.state.reflector));
        });

        ui.add_space(10.0);
        ui.label("Plugboard").on_hover_text("Steckerbrett");
        if control_string(ui, &mut self.plugboard_string).changed() {
            match self.cipher.state.set_plugboard(&self.plugboard_string) {
                Ok(_) => (),
                Err(e) => {
                    ui.label(error_text(&e.inner()));
                }
            }
        };
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {}

    fn reset(&mut self) {
        *self = Self::default()
    }
}
