use super::CodeFrame;
use crate::ui_elements::UiElements;
use codes::binary_to_text::pgp_words::PgpWords;

pub struct PgpWordsFrame {
    code: PgpWords,
}

impl Default for PgpWordsFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
        }
    }
}

impl CodeFrame for PgpWordsFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.add_space(16.0);
        ui.binary_to_text_input_mode(&mut self.code.mode);
        ui.add_space(16.0);
        ui.fill_code_columns(64, 4, Box::new(self.code.chars_codes()));
        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
