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
            });
        });

        match self.code.language {
            BrailleLanguage::English => ui.label("A partial implementation of Unified English Braille."),
            BrailleLanguage::French => ui.label("Louis Braille's orginal encoding."),
            BrailleLanguage::American => ui.label("American Braille is a now obsolete Braille encoding that rearranged some characters so that frequently used ones required fewer dots to ease writing by hand."),
        };

        ui.add_space(16.0);
        ui.fill_code_columns(8, 8, Box::new(self.code.language.chars_codes()));
        ui.add_space(32.0)
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
