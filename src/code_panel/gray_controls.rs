use codes::{mathematical::gray::GrayCode, traits::Code};
use egui::Slider;

use crate::ui_elements::UiElements;

use super::CodeFrame;

pub struct GrayCodeFrame {
    code: GrayCode,
}

impl Default for GrayCodeFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
        }
    }
}

impl CodeFrame for GrayCodeFrame {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.subheading("Width");
        ui.add(Slider::new(&mut self.code.width, 4..=8));
        ui.add_space(16.0);
        let chars_codes =
            (0..2_u32.pow(self.code.width as u32)).map(|n| (n, self.code.encode_u32(n)));
        ui.fill_code_columns(32, 8, Box::new(chars_codes));
    }

    fn code(&self) -> &dyn Code {
        &self.code
    }
}
