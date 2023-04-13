use egui::TextEdit;

use super::{generic_components::fill_code_columns, View, ViewableCode};
use crate::codes::BlockCode;

impl ViewableCode for BlockCode {}

impl View for BlockCode {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, errors: &mut String) {
        ui.label("Alphabet");
        if ui
            .add(TextEdit::singleline(&mut self.alphabet_string))
            .changed()
        {
            self.set_alphabet();
        };
        ui.add_space(16.0);

        ui.label("Symbols");
        if ui
            .add(TextEdit::singleline(&mut self.symbol_string))
            .changed()
        {
            self.set_symbols();
        };
        ui.add_space(16.0);

        ui.horizontal(|ui| {
            if ui.small_button("-").clicked() {
                if let Err(e) = self.decrease_width() {
                    *errors = e.to_string();
                } else {
                    errors.clear()
                }
            }
            ui.label(self.width.to_string());
            if ui.small_button("+").clicked() {
                if let Err(e) = self.increase_width() {
                    *errors = e.to_string();
                } else {
                    errors.clear()
                }
            }
        });
        fill_code_columns(24, 6, ui, self.chars_codes());
    }
}
