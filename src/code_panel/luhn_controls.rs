use codes::ecc::luhn::LuhnAlgorithm;
use egui::Slider;

use crate::egui_aux::error_text;

use super::CodeFrame;

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
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.group(|ui| {
            ui.label("Modulus");
            ui.add(Slider::new(&mut self.code.modulus, 2..=36).step_by(2.0));
            if self.code.modulus % 2 != 0 {
                ui.label(error_text("modulus must be even"));
            }
        });
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
