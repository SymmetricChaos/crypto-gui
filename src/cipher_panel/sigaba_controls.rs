use super::{View, ViewableCipher};
use crate::ciphers::{
    sigaba::{Rotor, BIG_ROTOR_VEC},
    Sigaba,
};
use eframe::egui::{ComboBox, RichText, Slider, Ui};

impl ViewableCipher for Sigaba {}

fn rotor_display(ui: &mut eframe::egui::Ui, rotors: &mut [Rotor]) {
    for (_, rotor) in &mut rotors.iter_mut().enumerate() {
        ui.horizontal(|ui| {
            // let name = RichText::new(format!("Rotor{}",n+1)).monospace();
            // ui.add_sized([20.0,20.0],Label::new(name));
            let characters = RichText::new(&rotor.to_string()).monospace();
            ui.label(characters);
            let range = 0..=rotor.size();
            ui.add(
                Slider::new(&mut rotor.position, range)
                    .clamp_to_range(true)
                    .show_value(false),
            );
        });
    }
}

impl View for Sigaba {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        // if ui.button("Restore State").clicked() {
        //     self.previous_state()
        // }

        ///////////////////////
        //// CIPHER ROTORS ////
        ///////////////////////
        ui.add_space(10.0);
        ui.label(
            RichText::new("Cipher Rotors").heading()
        ).on_hover_text("Message passes through these rotors during operation. Their pseudorandom movement is decided by the Control Rotors and Index Rotors.");
        for i in 0..5 {
            ui.horizontal(|ui| {
                ComboBox::from_id_source(format!("Cipher Rotor {}", i + 1))
                    .selected_text(self.cipher_rotors()[i].name)
                    .show_ui(ui, |ui| {
                        for rtr in BIG_ROTOR_VEC.iter() {
                            ui.selectable_value(
                                &mut self.cipher_rotors()[i],
                                rtr.clone(),
                                rtr.name.to_string(),
                            );
                        }
                    });
                ui.checkbox(&mut self.cipher_rotors()[i].reversed, "reversed");
            });
        }
        ui.add_space(10.0);
        rotor_display(ui, self.cipher_rotors());

        ////////////////////////
        //// CONTROL ROTORS ////
        ////////////////////////
        ui.add_space(20.0);
        ui.label(
            RichText::new("Control Rotors").heading()
        ).on_hover_text("These rotors move in a simple pattern during operation to produce control signals and send that to the Index Rotors.");
        for i in 0..5 {
            ui.horizontal(|ui| {
                ComboBox::from_id_source(format!("Control Rotor {}", i + 1))
                    .selected_text(self.control_rotors()[i].name)
                    .show_ui(ui, |ui| {
                        for rtr in BIG_ROTOR_VEC.iter() {
                            ui.selectable_value(
                                &mut self.control_rotors()[i],
                                rtr.clone(),
                                rtr.name.to_string(),
                            );
                        }
                    });
                ui.checkbox(&mut self.control_rotors()[i].reversed, "reversed");
            });
        }
        ui.add_space(10.0);
        rotor_display(ui, self.control_rotors());

        //////////////////////
        //// INDEX ROTORS ////
        //////////////////////
        ui.add_space(20.0);
        ui.label(
            RichText::new("Index Rotors").heading()
        ).on_hover_text("These rotors stay in position during encryption. The signal from the Control Rotors is sent through them to the Cipher Rotors to decide which move.");
        ui.horizontal(|ui| {
            for (n, rotor) in &mut self.index_rotors().iter_mut().enumerate() {
                //let characters = RichText::new(format!("{}{}",n+1,&rotor.to_string())).monospace();
                //ui.label(characters);
                let val = format!("{}{}", n + 1, &rotor.to_string());
                let range = 0..=9;
                ui.add(
                    Slider::new(&mut rotor.position, range)
                        .clamp_to_range(true)
                        .show_value(false)
                        .vertical()
                        .text(val),
                );
            }
        });
    }
}
