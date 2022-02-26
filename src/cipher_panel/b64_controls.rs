use eframe::egui::{RichText, TextEdit};

use super::View;
use super::generic_components::*;
use crate::ciphers::B64;


impl View for B64 {
    fn ui(&mut self, ui: &mut eframe::egui::Ui) {

        randomize_button(ui, self);
        ui.add_space(16.0);

        ui.label("Polybius Key Word");
        ui.add(TextEdit::singleline(self.polybius.control_key()));

        ui.label(RichText::new(format!("Grid\n{}",self.polybius)).monospace());
        ui.add_space(16.0);

        ui.label("First Columnar Key Word");
        ui.add(TextEdit::singleline(self.columnar2.control_key()));

        ui.label("Second Columnar Key Word");
        ui.add(TextEdit::singleline(self.columnar1.control_key()));
    }
}
