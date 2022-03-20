use eframe::egui::{RichText, Slider, ComboBox, Label};
use super::View;
use crate::ciphers::{Sigaba, sigaba::{CIPHER_ROTOR_VEC, CONTROL_ROTOR_VEC, Rotor}};

fn rotor_display(ui: &mut eframe::egui::Ui, rotors: &mut [Rotor]) {
    for (n, rotor) in &mut rotors.iter_mut().enumerate() {
        ui.horizontal(|ui| {
            let name = RichText::new(format!("Rotor{}",n+1)).monospace();
            ui.add_sized([20.0,20.0],Label::new(name));
            let characters = RichText::new(&rotor.to_string()).monospace();
            ui.label(characters);
            let range = 0..=rotor.size();
            ui.add(Slider::new(&mut rotor.position, range)
                .clamp_to_range(true)
                .show_value(false)
            );
        });
    }
}

impl View for Sigaba {
    fn ui(&mut self, ui: &mut eframe::egui::Ui) {
 

        ///////////////////////
        //// CIPHER ROTORS ////
        ///////////////////////
        ui.add_space(10.0);
        ui.label("Select Cipher Rotors");
        for i in 0..5 {
            ComboBox::from_id_source(format!("Cipher Rotor {}",i+1))
                .selected_text(self.cipher_rotors()[i].name)
                .show_ui(ui, |ui| {
                for rtr in CIPHER_ROTOR_VEC.iter() {
                    ui.selectable_value(&mut self.cipher_rotors()[i], rtr.clone(), rtr.name.to_string());
                }
            });
        }

        ui.add_space(10.0);
        ui.label("Cipher Rotors");
        rotor_display(ui, self.cipher_rotors());

        ////////////////////////
        //// CONTROL ROTORS ////
        ////////////////////////
        ui.add_space(10.0);
        ui.label("Select Control Rotors");
        for i in 0..5 {
            ComboBox::from_id_source(format!("Control Rotor {}",i+1))
                .selected_text(self.control_rotors()[i].name)
                .show_ui(ui, |ui| {
                for rtr in CONTROL_ROTOR_VEC.iter() {
                    ui.selectable_value(&mut self.control_rotors()[i], rtr.clone(), rtr.name.to_string());
                }
            });
        }

        ui.add_space(10.0);
        ui.label("Control Rotors");
        rotor_display(ui, self.control_rotors());


        //////////////////////
        //// INDEX ROTORS ////
        //////////////////////
        ui.add_space(10.0);
        ui.label("Index Rotors");
        rotor_display(ui, self.index_rotors());

    }
}