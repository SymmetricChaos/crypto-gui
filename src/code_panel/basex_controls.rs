use super::CodeFrame;
use crate::ui_elements::UiElements;
use codes::binary_to_text::basex::BaseX;

pub struct BaseXFrame {
    code: BaseX,
    alphabet: String,
}

impl Default for BaseXFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
            alphabet: String::from("123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz"),
        }
    }
}

impl CodeFrame for BaseXFrame {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.subheading("Alphabet");
        if ui.control_string(&mut self.alphabet).changed() {
            self.code.set_map(&self.alphabet)
        }

        ui.fill_code_columns(16, 4, Box::new(self.code.chars_codes()));
        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
