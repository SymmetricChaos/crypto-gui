use super::CodeFrame;
use crate::ui_elements::UiElements;
use codes::text_standards::braille::{Braille, BrailleLanguage};

pub struct BrailleFrame {
    code: Braille,
}

impl Default for BrailleFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
        }
    }
}

impl CodeFrame for BrailleFrame {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.subheading("Language");
            ui.horizontal(|ui| {
                ui.selectable_value(
                    &mut self.code.language,
                    BrailleLanguage::English,
                    "English (Unified)",
                );
                ui.selectable_value(&mut self.code.language, BrailleLanguage::French, "French");
                ui.selectable_value(
                    &mut self.code.language,
                    BrailleLanguage::American,
                    "American",
                );
                ui.selectable_value(&mut self.code.language, BrailleLanguage::Ascii, "ASCII");
            });
        });

        match self.code.language {
            BrailleLanguage::English => ui.label("A partial implementation of Unified English Braille."),
            BrailleLanguage::French => ui.label("Louis Braille's orginal encoding."),
            BrailleLanguage::American => ui.label("American Braille is a now obsolete Braille encoding that rearranged some characters so that frequently used ones required fewer dots to ease writing by hand."),
            BrailleLanguage::Ascii => ui.label("Braille ASCII is a strict one-to-one encoding from a range of ASCII characters to the 64 possible braille characters. This encoding it not meant to be used directly as there is no particular meaningful correspondence between the ASCII symbols and the Braille symbols. Instead it is a standard for alternative fonts."),
        };

        ui.add_space(16.0);
        ui.fill_code_columns(10, 6, Box::new(self.code.language.chars_codes()));
        ui.add_space(32.0)
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
