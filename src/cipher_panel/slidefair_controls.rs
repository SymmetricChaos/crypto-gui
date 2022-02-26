use eframe::egui::{RichText, TextEdit, TextStyle};

use super::View;
use super::generic_components::*;
use crate::ciphers::Slidefair;

impl View for Slidefair {
    fn ui(&mut self, ui: &mut eframe::egui::Ui) {
        
        ui.label("Select Alphabet");
        ui.add(TextEdit::singleline(self.control_alphabet()).font(TextStyle::Monospace));
        ui.add_space(16.0);

        ui.label("Spacer Character\nInserted at end as padding if needed");
        ui.add(TextEdit::singleline(self.control_spacer()).font(TextStyle::Monospace).desired_width(15.0));

        ui.label(RichText::new(format!("Grid\n{}",self)).monospace());
        ui.add_space(16.0);

        randomize_button(ui, self);
    }
}
