use super::{generic_components::fill_code_columns, View, ViewableCode};
use crate::{
    codes::{spelling_alphabet::SpellingAlphabetMode, SpellingAlphabet},
    egui_aux::subheading,
};

impl ViewableCode for SpellingAlphabet {}

impl View for SpellingAlphabet {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _errors: &mut String) {
        ui.group(|ui| {
            ui.label(subheading("Alphabet"));
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.mode, SpellingAlphabetMode::Nato, "NATO (ICAO)");
                ui.selectable_value(&mut self.mode, SpellingAlphabetMode::Ccb, "CCB");
                ui.selectable_value(
                    &mut self.mode,
                    SpellingAlphabetMode::Usn1908,
                    "US Navy (1908) (long)",
                );
                ui.selectable_value(
                    &mut self.mode,
                    SpellingAlphabetMode::Usn1908Alt,
                    "US Navy (1908) (short)",
                );
                ui.selectable_value(
                    &mut self.mode,
                    SpellingAlphabetMode::Wu1912,
                    "Western Union (1912)",
                );
                ui.selectable_value(
                    &mut self.mode,
                    SpellingAlphabetMode::Wu1942,
                    "Western Union (1942)",
                );

                ui.selectable_value(
                    &mut self.mode,
                    SpellingAlphabetMode::Us1941,
                    "US Joint Army/Navy (1941)",
                );
            });
        });

        fill_code_columns(9, 4, ui, Box::new(self.chars_codes()));
    }
}
