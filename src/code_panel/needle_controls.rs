use super::CodeFrame;
use crate::ui_elements::{control_string, fill_code_columns, subheading};
use codes::text_standards::needle::Needle;

pub struct NeedleFrame {
    code: Needle,
}

impl Default for NeedleFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
        }
    }
}

impl CodeFrame for NeedleFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.label(subheading("Alphabet"));
        if control_string(ui, &mut self.code.alphabet).changed() {
            self.code.set_map()
        }
        ui.add_space(16.0);
        fill_code_columns(5, 4, ui, self.code.chars_codes());
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
