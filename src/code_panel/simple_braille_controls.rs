use codes::braille::simple_braille::{BrailleLanguage, SimpleBraille};

use super::CodeFrame;
use crate::ui_elements::UiElements;

pub struct BrailleFrame {
    code: SimpleBraille,
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
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/tree/master/codes/src/braille",
        );
        ui.add_space(8.0);

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
            BrailleLanguage::English => ui.label("A partial implementation of the modern Unified English Braille. A more complete implementation of Grade 1 UEB is availble."),
            BrailleLanguage::French => ui.label("Louis Braille's orginal encoding."),
            BrailleLanguage::American => ui.label("American Braille is a now obsolete Braille encoding that rearranged some characters so that frequently used ones required fewer dots to ease writing by hand."),
        };

        if let Some(sign) = self.code.language.capital_sign() {
            ui.label(format!("Capital Prefix  {}", sign));
        }

        if let Some(sign) = self.code.language.number_sign() {
            ui.label(format!("Numeric Prefix  {}", sign));
        }

        ui.add_space(16.0);
        ui.fill_code_columns(8, 8, Box::new(self.code.language.chars_codes()));
        ui.add_space(32.0)
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
