use codes::ecc::m_of_n::MofNCode;
use egui::Slider;

use super::CodeFrame;

pub struct MofNCodeFrame {
    code: MofNCode,
}

impl Default for MofNCodeFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
        }
    }
}

impl CodeFrame for MofNCodeFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.group(|ui| {
            ui.label("Weight");
            ui.add(Slider::new(&mut self.code.weight, 1..=self.code.length));
            ui.label("Length");
            ui.add(Slider::new(&mut self.code.length, 0..=10));
        });
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
