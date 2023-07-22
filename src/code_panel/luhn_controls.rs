use super::CodeFrame;
use crate::ui_elements::UiElements;
use codes::ecc::luhn::LuhnAlgorithm;
use egui::Slider;

pub struct LuhnAlgorithmFrame {
    code: LuhnAlgorithm,
}

impl Default for LuhnAlgorithmFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
        }
    }
}

impl CodeFrame for LuhnAlgorithmFrame {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.subheading("Modulus");
        ui.add(Slider::new(&mut self.code.modulus, 2..=36).step_by(2.0));
        if self.code.modulus % 2 != 0 {
            ui.error_text("modulus must be even");
        }
        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
