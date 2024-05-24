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
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/codes/src/mathematical/gray.rs",
        );
        ui.add_space(8.0);

        ui.subheading("Width");
        ui.add(Slider::new(&mut self.code.width, 4..=8));
        ui.add_space(8.0);
        ui.checkbox(&mut self.code.spaced, "Use Spaces");
        ui.add_space(16.0);
        let chars_codes =
            (0..2_u32.pow(self.code.width as u32)).map(|n| (n, self.code.encode_u32(n)));
        ui.fill_code_columns(32, 8, Box::new(chars_codes));
    }

    fn code(&self) -> &dyn Code {
        &self.code
    }
}
