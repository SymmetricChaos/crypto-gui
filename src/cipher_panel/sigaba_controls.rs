use crate::ui_elements::UiElements;

use super::CipherFrame;
use ciphers::{
    machines::sigaba::{Sigaba, BIG_ROTOR_VEC},
    rotors::Rotor,
    Cipher,
};
use egui::{ComboBox, RichText, Slider, Ui};

#[derive(Default)]
pub struct SigabaFrame {
    cipher: Sigaba,
}

fn rotor_display<const N: usize>(ui: &mut eframe::egui::Ui, rotors: &mut [Rotor<N>]) {
    for (_, rotor) in &mut rotors.iter_mut().enumerate() {
        ui.horizontal(|ui| {
            let characters = RichText::new(&rotor.to_string()).monospace();
            ui.label(characters);
            let range = 0..=N;
            ui.add(Slider::new(&mut rotor.position, range).show_value(false));
        });
    }
}

impl CipherFrame for SigabaFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/tree/master/ciphers/src/machines/sigaba",
        );
        ui.add_space(8.0);

        // if ui.button("Restore State").clicked() {
        //     self.previous_state()
        // }

        ///////////////////////
        //// CIPHER ROTORS ////
        ///////////////////////
        let cipher_rotors = self.cipher.cipher_rotors();
        ui.add_space(10.0);
        ui.subheading("Cipher Rotors"
        ).on_hover_text("Message passes through these rotors during operation. Their pseudorandom movement is decided by the Control Rotors and Index Rotors.");
        for i in 0..5 {
            ui.horizontal(|ui| {
                ComboBox::from_id_salt(format!("Cipher Rotor {}", i + 1))
                    .selected_text(cipher_rotors[i].name)
                    .show_ui(ui, |ui| {
                        for rtr in BIG_ROTOR_VEC.iter() {
                            ui.selectable_value(
                                &mut cipher_rotors[i],
                                rtr.clone(),
                                rtr.name.to_string(),
                            );
                        }
                    });
                ui.checkbox(&mut cipher_rotors[i].reversed, "reversed");
            });
        }
        ui.add_space(10.0);
        rotor_display(ui, cipher_rotors);

        ////////////////////////
        //// CONTROL ROTORS ////
        ////////////////////////
        let control_rotors = self.cipher.control_rotors();
        ui.add_space(20.0);
        ui.subheading(
            "Control Rotors"
        ).on_hover_text("These rotors move in a simple pattern during operation to produce control signals and send that to the Index Rotors.");
        for i in 0..5 {
            ui.horizontal(|ui| {
                ComboBox::from_id_salt(format!("Control Rotor {}", i + 1))
                    .selected_text(control_rotors[i].name)
                    .show_ui(ui, |ui| {
                        for rtr in BIG_ROTOR_VEC.iter() {
                            ui.selectable_value(
                                &mut control_rotors[i],
                                rtr.clone(),
                                rtr.name.to_string(),
                            );
                        }
                    });
                ui.checkbox(&mut control_rotors[i].reversed, "reversed");
            });
        }
        ui.add_space(10.0);
        rotor_display(ui, control_rotors);

        //////////////////////
        //// INDEX ROTORS ////
        //////////////////////
        ui.add_space(20.0);
        ui.subheading(
            "Index Rotors"
        ).on_hover_text("These rotors stay in position during encryption. The signal from the Control Rotors is sent through them to the Cipher Rotors to decide which move.");
        ui.horizontal(|ui| {
            for (n, rotor) in &mut self.cipher.index_rotors().iter_mut().enumerate() {
                //let characters = RichText::new(format!("{}{}",n+1,&rotor.to_string())).monospace();
                //ui.label(characters);
                let val = format!("{}{}", n + 1, &rotor.to_string());
                let range = 0..=9;
                ui.add(
                    Slider::new(&mut rotor.position, range)
                        .show_value(false)
                        .vertical()
                        .text(val),
                );
            }
        });
        ui.add_space(16.0);
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {}

    fn reset(&mut self) {
        *self = Self::default()
    }
}
