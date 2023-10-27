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
                ui.selectable_value(&mut self.code.language, BrailleLanguage::English, "English");
                ui.selectable_value(&mut self.code.language, BrailleLanguage::French, "French");
            });
        });

        ui.add_space(16.0);
        ui.fill_code_columns(10, 6, Box::new(self.code.language.chars_codes()));
        ui.add_space(32.0)
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
