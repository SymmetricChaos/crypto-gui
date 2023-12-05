use super::CodeFrame;
use crate::ui_elements::UiElements;
use codes::text_standards::morse::{Morse, MorseRep, MorseStandard};

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
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.subheading("Variant");
            ui.horizontal(|ui| {
                ui.selectable_value(
                    &mut self.code.standard,
                    MorseStandard::Itu,
                    "ITU Morse (1865)",
                );
                if ui
                    .selectable_value(
                        &mut self.code.standard,
                        MorseStandard::American,
                        "American Morse (1844)",
                    )
                    .clicked()
                {
                    if self.code.representation == MorseRep::Ascii
                        || self.code.representation == MorseRep::Word
                    {
                        self.code.representation = MorseRep::HalfBlock
                    }
                }
                if ui
                    .selectable_value(&mut self.code.standard, MorseStandard::Gerke, "Gerke Code")
                    .clicked()
                {
                    if self.code.representation == MorseRep::Ascii
                        || self.code.representation == MorseRep::Word
                    {
                        self.code.representation = MorseRep::HalfBlock
                    }
                }
                ui.selectable_value(&mut self.code.standard, MorseStandard::Greek, "Greek Morse");
                ui.selectable_value(
                    &mut self.code.standard,
                    MorseStandard::Wabun,
                    "Wabun Code (Japanese Morse)",
                );
            });
        });

        ui.subheading("Representation");
        ui.horizontal(|ui| {
            // Line code works for everything
            ui.selectable_value(
                &mut self.code.representation,
                MorseRep::HalfBlock,
                "Halfblock (Line Code)",
            );
            // ASCII and words only work for codes with standardized sizes
            ui.add_enabled_ui(
                [
                    MorseStandard::Itu,
                    MorseStandard::Greek,
                    MorseStandard::Wabun,
                ]
                .contains(&self.code.standard),
                |ui| {
                    ui.selectable_value(
                        &mut self.code.representation,
                        MorseRep::Ascii,
                        "ASCII symbols",
                    );
                    ui.selectable_value(
                        &mut self.code.representation,
                        MorseRep::Word,
                        "Dit/Dah (Words)",
                    );
                },
            );
        });

        ui.add_space(16.0);
        match self.code.standard {
            MorseStandard::Itu => ui.label("The best known variant of Morse code is the standard adopted by the International Telegraphy Union. There are two kinds of marks, dits and dahs. A dit lasts for one unit of time and a dah for three times that. The space between marks is the length of a dit. Between letters a space the length of a dah is used. Sentences words a space the length of seven dits is used."),
            MorseStandard::American => ui.label("The original 1844 code created by Morse and Vail was much more complex than the better known international standard. Rather than two kinds of marks there were four. The dit was of one unit, the dash of two units, the long dash of five (used for the letter 'L'), and the longer dash of six (used for the number '0'). Gaps between marks also varied in size. A gap within a character could be either one or two units in length. Only line codes have standard ways of representing American Morse."),
            MorseStandard::Gerke => ui.label("Gerke considerably simplified Morse and Vail's code by standardizing the size of marks and gaps. There are two kinds of marks, dits and dahs. A dit lasts for one unit of time and a dah for three times that. The space between marks is the length of a dit. Between letters a space the length of a dah is used. Sentences words a space the length of seven dits is used. An exception, however, is made for '0' which was encoded as a single mark of length six as in American Morse. Gerke made various other changes: addition of letters commonly needed in German, removal of the letter 'I', and fixed width digits (excepting zero). Due to the long '0' only line codes have standard ways of representing Gerke's code."),
            MorseStandard::Greek => ui.label("text"),
            MorseStandard::Wabun => ui.label("text"),
        };

        ui.add_space(16.0);
        ui.fill_code_columns(20, 3, Box::new(self.code.chars_codes()));
        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
