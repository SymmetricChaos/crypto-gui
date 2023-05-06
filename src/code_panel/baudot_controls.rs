use super::{generic_components::fill_code_columns, View, ViewableCode};
use crate::{
    codes::{baudot::BaudotVersion, Baudot},
    egui_aux::subheading,
};

impl ViewableCode for Baudot {}

impl View for Baudot {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _errors: &mut String) {
        ui.group(|ui| {
            ui.label(subheading("Variant"));
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.version, BaudotVersion::Ita1, "ITA1 (Baudot)");
                ui.selectable_value(
                    &mut self.version,
                    BaudotVersion::Ita2,
                    "ITA2 (Baudot-Murray)",
                );
                ui.selectable_value(&mut self.version, BaudotVersion::UsTty, "US TTY");
            });
        });
        ui.add_space(16.0);
        match self.version {
            BaudotVersion::Ita1 => ui.label("The first International Telegraphy Alphabet was a widely used version of Baudot's original encoding."),
            BaudotVersion::Ita2 => ui.label("The second International Telegraphy Alphabet based on work by Baudot and Murray became a widely used standard that made ITA1 obsolete. This variant is still in minor usage."),
            BaudotVersion::UsTty => ui.label("The US Teleteypewriter code is a less common variant used in the United States."),
        };

        ui.add_space(16.0);
        fill_code_columns(16, 4, ui, self.codes_chars());
    }
}
