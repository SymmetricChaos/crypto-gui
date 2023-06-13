use codes::binary_to_text::pgp_words::PgpWords;

use crate::ui_elements::{binary_to_text_input_mode, fill_code_columns};

use super::CodeFrame;

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
        binary_to_text_input_mode(ui, &mut self.code.mode);
        ui.add_space(16.0);
        fill_code_columns(64, 4, ui, Box::new(self.code.chars_codes()));
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
