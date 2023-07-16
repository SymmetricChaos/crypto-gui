use codes::text_standards::baudot::{Baudot, BaudotVersion};

use crate::ui_elements::UiElements;

use super::CodeFrame;

pub struct BaudotFrame {
    code: Baudot,
}

impl Default for BaudotFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
        }
    }
}

impl CodeFrame for BaudotFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.group(|ui| {
            ui.subheading("Variant");
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.code.version, BaudotVersion::Ita1, "ITA1 (Baudot)");
                ui.selectable_value(
                    &mut self.code.version,
                    BaudotVersion::Ita2,
                    "ITA2 (Baudot-Murray)",
                );
                ui.selectable_value(&mut self.code.version, BaudotVersion::UsTty, "US TTY");
            });
        });
        ui.add_space(16.0);
        match self.code.version {
            BaudotVersion::Ita1 => ui.label("The first International Telegraphy Alphabet was a version of Baudot's original encoding."),
            BaudotVersion::Ita2 => ui.label("The second International Telegraphy Alphabet based on work by Baudot and Murray became a widely used standard that made ITA1 obsolete. This variant is still in minor usage."),
            BaudotVersion::UsTty => ui.label("The US Teleteypewriter code is a less common variant used in the United States."),
        };

        ui.add_space(16.0);
        ui.fill_code_columns(16, 4, self.code.codes_chars());
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
