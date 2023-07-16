use super::CodeFrame;
use crate::ui_elements::UiElements;
use codes::text_standards::linotype::Linotype;

pub struct LinotypeFrame {
    code: Linotype,
}

impl Default for LinotypeFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
        }
    }
}

impl CodeFrame for LinotypeFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.fill_code_columns(32, 4, self.code.chars_codes());
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
