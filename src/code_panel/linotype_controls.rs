use super::{generic_components::fill_code_columns, View, ViewableCode};
use crate::{
    codes::{linotype::MagazineVariant, Linotype},
    egui_aux::subheading,
};

impl ViewableCode for Linotype {}

impl View for Linotype {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _errors: &mut String) {
        ui.group(|ui| {
            ui.label(subheading("Magazine"));
            ui.horizontal(|ui| {
                ui.selectable_value(
                    &mut self.variant,
                    MagazineVariant::NinetyChannel,
                    "90 Channel",
                );
            });
        });

        fill_code_columns(32, 4, ui, self.chars_codes());
    }
}
