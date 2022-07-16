use eframe::egui::{self, Slider, Ui};

use super::{View, ViewableCipher, _generic_components::*};
use crate::ciphers::M209;

impl ViewableCipher for M209 {}

fn lug_pair(ui: &mut egui::Ui, pair: &mut (usize, usize)) {
    ui.add(
        egui::DragValue::new(&mut pair.0)
            .clamp_range(0usize..=6)
            .speed(0.1),
    );
    ui.add(
        egui::DragValue::new(&mut pair.1)
            .clamp_range(0usize..=6)
            .speed(0.1),
    );
}

impl View for M209 {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        randomize_reset(ui, self);
        ui.add_space(16.0);

        ui.label("Alphabet");
        ui.label("ABDCEFGHIJKLMNOPQRSTUVWXYZ");
        ui.add_space(16.0);

        ui.label("Rotor Settings");
        for rotor in self.get_wheels() {
            let len = rotor.rotor_length() - 1;
            ui.add(Slider::new(&mut rotor.active, 0..=len).show_value(false));
            ui.label(format!("{}", rotor));
        }

        ui.add_space(16.0);

        let lugs = &mut self.lugs;
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
    }
}
