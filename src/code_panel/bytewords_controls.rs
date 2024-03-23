use codes::binary_to_text::bytewords::{Bytewords, Separator};

use super::CodeFrame;
use crate::ui_elements::UiElements;

pub struct BytewordsFrame {
    code: Bytewords,
}

impl Default for BytewordsFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
        }
    }
}

impl CodeFrame for BytewordsFrame {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.add_space(16.0);
        ui.binary_to_text_input_mode(&mut self.code.mode);
        ui.add_space(16.0);
        ui.subheading("Separator");
        ui.selectable_value(&mut self.code.sep, Separator::Space, "Space");
        ui.selectable_value(&mut self.code.sep, Separator::Dash, "Dash");
        ui.add_space(16.0);
        ui.fill_code_columns(256, 8, Box::new(self.code.chars_codes()));
        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
