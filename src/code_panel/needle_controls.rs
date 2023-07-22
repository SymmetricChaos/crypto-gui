use super::CodeFrame;
use crate::ui_elements::UiElements;
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
        ui.subheading("Alphabet");
        if ui.control_string(&mut self.code.alphabet).changed() {
            self.code.set_map()
        }
        ui.add_space(16.0);
        ui.fill_code_columns(5, 4, self.code.chars_codes());
        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
