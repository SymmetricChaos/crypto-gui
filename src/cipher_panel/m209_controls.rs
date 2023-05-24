use ciphers::{Cipher, M209};
use egui::{Slider, Ui};

use super::CipherFrame;

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

#[derive(Default)]
pub struct M209Frame {
    cipher: M209,
}

impl CipherFrame for M209Frame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        // randomize_reset(ui, self);
        ui.add_space(16.0);

        ui.label("Alphabet");
        ui.label("ABDCEFGHIJKLMNOPQRSTUVWXYZ");
        ui.add_space(16.0);

        ui.label("Rotor Settings");
        for rotor in self.cipher.get_wheels() {
            let len = rotor.rotor_length() - 1;
            ui.add(Slider::new(&mut rotor.active, 0..=len).show_value(false));
            ui.label(format!("{}", rotor));
        }

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
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {}

    fn reset(&mut self) {
        *self = Self::default()
    }
}
