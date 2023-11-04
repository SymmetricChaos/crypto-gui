use super::CodeFrame;
use crate::ui_elements::UiElements;
use codes::binary_to_text::base16::Base16;

pub struct Base16Frame {
    code: Base16,
}

impl Default for Base16Frame {
    fn default() -> Self {
        Self {
            code: Default::default(),
        }
    }
}

impl CodeFrame for Base16Frame {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.checkbox(&mut self.code.upper, "Uppercase");

        if self.code.upper {
            ui.fill_code_columns(8, 4, Box::new((0..16).zip("0123456789ABCDEF".chars())));
        } else {
            ui.fill_code_columns(8, 4, Box::new((0..16).zip("0123456789abcdef".chars())));
        }

        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
