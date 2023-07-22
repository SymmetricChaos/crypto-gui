use codes::ecc::m_of_n::MofNCode;
use egui::Slider;

use crate::ui_elements::UiElements;

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
            ui.subheading("Length");
            ui.label("Total number of bits.");
            ui.add(Slider::new(&mut self.code.length, 0..=10));
            ui.subheading("Weight");
            ui.label("Number of 1s in each code.");
            ui.add(Slider::new(&mut self.code.weight, 1..=self.code.length));
        });
        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
