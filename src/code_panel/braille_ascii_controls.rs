use codes::braille::braille_ascii::BrailleAscii;

use super::CodeFrame;
use crate::ui_elements::UiElements;

pub struct BrailleAsciiFrame {
    code: BrailleAscii,
}

impl Default for BrailleAsciiFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
        }
    }
}

impl CodeFrame for BrailleAsciiFrame {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.add_space(16.0);
        ui.fill_code_columns(8, 8, Box::new(BrailleAscii::chars_codes()));
        ui.add_space(32.0)
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
