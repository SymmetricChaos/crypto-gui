use egui::SelectableLabel;

use super::{generic_components::fill_code_columns, View, ViewableCode};
use crate::codes::morse::{Morse, MorseRep, MorseStandard};

impl ViewableCode for Morse {}

impl View for Morse {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _errors: &mut String) {
        ui.label("Representation");
        if self.standard != MorseStandard::American {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.mode, MorseRep::HalfBlock, "Halfblock Line Code");
                ui.selectable_value(&mut self.mode, MorseRep::Binary, "Binary Line Code");
                ui.selectable_value(&mut self.mode, MorseRep::Ascii, "ASCII symbols");
                ui.selectable_value(&mut self.mode, MorseRep::CdotNDash, "Cdot and En-dash");
            });
        } else {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.mode, MorseRep::HalfBlock, "Halfblock Line Code");
                ui.selectable_value(&mut self.mode, MorseRep::Binary, "Binary Line Code");
                ui.add_enabled(false, SelectableLabel::new(false, "ASCII symbols"));
                ui.add_enabled(false, SelectableLabel::new(false, "Cdot and En-dash"));
            });
        }
        ui.add_space(10.0);
        ui.label("Standard");
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.standard, MorseStandard::Itu, "ITU Morse");
            if ui
                .selectable_value(
                    &mut self.standard,
                    MorseStandard::American,
                    "American Morse",
                )
                .clicked()
            {
                if self.mode == MorseRep::Ascii || self.mode == MorseRep::CdotNDash {
                    self.mode = MorseRep::HalfBlock
                }
            }
        });
        ui.add_space(10.0);
        fill_code_columns(20, 3, ui, Box::new(self.chars_codes()))
    }
}
