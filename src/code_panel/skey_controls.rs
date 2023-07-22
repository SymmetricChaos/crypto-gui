use super::CodeFrame;
use crate::ui_elements::UiElements;
use codes::binary_to_text::skey::SKeyWords;

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
        ui.binary_to_text_input_mode(&mut self.code.mode);
        ui.add_space(16.0);
        ui.fill_code_columns(256, 8, Box::new(self.code.chars_codes()));
        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
