use eframe::egui;
use super::View;
use super::generic_components::*;
use crate::ciphers::M209;

fn lug_pair(ui: &mut egui::Ui, pair: &mut (usize,usize)) {
    ui.add(egui::DragValue::new(&mut pair.0).clamp_range(0usize..=6).speed(0.1));
    ui.add(egui::DragValue::new(&mut pair.1).clamp_range(0usize..=6).speed(0.1));
}


pub struct M209Controls {
    cipher: M209,
}

impl Default for M209Controls {
    fn default() -> Self {
        Self { 
            cipher: M209::default(),
        }
    }
}

impl View for M209Controls {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, input: &mut String, output: &mut String) {
        ui.label("Alphabet");
        ui.label("ABDCEFGHIJKLMNOPQRSTUVWXYZ");
        ui.add_space(16.0);

        ui.label("Pins");
        //cipher.set_pins();
        ui.add_space(16.0);

        let lugs = &mut self.cipher.lugs;
        ui.label("Lugs");
        for triple in lugs.chunks_exact_mut(3) {
            ui.horizontal(|ui| {
                lug_pair(ui, &mut triple[0]);
                ui.add_space(4.0);
                lug_pair(ui, &mut triple[1]);
                ui.add_space(4.0);
                lug_pair(ui, &mut triple[2]);
            });
        }

        encrypt_decrypt(ui, &mut self.cipher, input, output);
        ui.add_space(16.0);
        randomize_button(ui, &mut self.cipher);
        ui.add_space(16.0);
        clear_button(ui, input, output);
    }
}
