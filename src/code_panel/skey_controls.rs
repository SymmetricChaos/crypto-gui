use codes::binary_to_text::skey::SKeyWords;

use crate::ui_elements::{binary_to_text_input_mode, fill_code_columns};

use super::CodeFrame;

pub struct SKeyWordsFrame {
    code: SKeyWords,
}

impl Default for SKeyWordsFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
        }
    }
}

impl CodeFrame for SKeyWordsFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.add_space(16.0);
        binary_to_text_input_mode(ui, &mut self.code.mode);
        ui.add_space(16.0);
        fill_code_columns(256, 8, ui, Box::new(self.code.chars_codes()));
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
