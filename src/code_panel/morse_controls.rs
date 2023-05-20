use codes::morse::{Morse, MorseRep, MorseStandard};
use egui::SelectableLabel;

use crate::egui_aux::subheading;

use super::{generic_components::fill_code_columns, CodeFrame};

pub struct MorseFrame {
    code: Morse,
}

impl Default for MorseFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
        }
    }
}

impl CodeFrame for MorseFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.group(|ui| {
            ui.label(subheading("Variant"));
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.code.standard, MorseStandard::Itu, "ITU Morse");
                if ui
                    .selectable_value(
                        &mut self.code.standard,
                        MorseStandard::American,
                        "American Morse",
                    )
                    .clicked()
                {
                    if self.code.mode == MorseRep::Ascii || self.code.mode == MorseRep::CdotNDash {
                        self.code.mode = MorseRep::HalfBlock
                    }
                }
            });
        });

        ui.label(subheading("Representation"));
        if self.code.standard != MorseStandard::American {
            ui.horizontal(|ui| {
                ui.selectable_value(
                    &mut self.code.mode,
                    MorseRep::HalfBlock,
                    "Halfblock Line Code",
                );
                ui.selectable_value(&mut self.code.mode, MorseRep::Binary, "Binary Line Code");
                ui.selectable_value(&mut self.code.mode, MorseRep::Ascii, "ASCII symbols");
                ui.selectable_value(&mut self.code.mode, MorseRep::CdotNDash, "Cdot and En-dash");
            });
        } else {
            ui.horizontal(|ui| {
                ui.selectable_value(
                    &mut self.code.mode,
                    MorseRep::HalfBlock,
                    "Halfblock Line Code",
                );
                ui.selectable_value(&mut self.code.mode, MorseRep::Binary, "Binary Line Code");
                ui.add_enabled(false, SelectableLabel::new(false, "ASCII symbols"));
                ui.add_enabled(false, SelectableLabel::new(false, "Cdot and En-dash"));
            });
        }

        ui.add_space(16.0);
        match self.code.standard {
            MorseStandard::Itu => ui.label("The best known variant of Morse code is the standard adopted by the International Telegraphy Union. There are two kinds of marks, dits and dahs. A dit lasts for one unit of time and a dah for three times that. The space between marks is the length of a dit. Between letters a space the length of a dah is used. Sentences words a space the length of seven dits is used."),
            MorseStandard::American => ui.label("The original 1844 created by Morse and Vail was much more complex than the better known international standard. Rather than two kinds of marks there were four. The dor was of one unit, the dash of two units, the long dash of five (used for the letter 'L'), and the longer dash of six (used for the number '0'). Gaps between marks also varied in size. A gap within a character could be either one or two units in length."),
        };

        ui.add_space(16.0);
        fill_code_columns(20, 3, ui, Box::new(self.code.chars_codes()))
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
