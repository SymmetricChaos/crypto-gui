use eframe::egui::Ui;
use rand::prelude::StdRng;
use eframe::egui::{RichText, TextEdit, TextStyle};


use super::generic_components::*;
use super::View;
use crate::ciphers::Slidefair;

impl View for Slidefair {
    fn ui(&mut self, ui: &mut Ui, rng: &mut StdRng) {
        randomize_reset(ui, self, rng);
        ui.add_space(16.0);

        ui.label("Select Alphabet");
        ui.add(TextEdit::singleline(self.control_alphabet()).font(TextStyle::Monospace));
        ui.add_space(16.0);

        ui.label("Spacer Character\nInserted at end as padding if needed");
        ui.add(
            TextEdit::singleline(self.control_spacer())
                .font(TextStyle::Monospace)
                .desired_width(15.0),
        );

        ui.label("Grid");
        for row in self.rows() {
            ui.label(RichText::new(row).monospace());
        }

        ui.add_space(16.0);
    }
}
