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
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/codes/src/ecc/m_of_n.rs",
        );
        ui.add_space(8.0);

        ui.subheading("Length");
        ui.label("Total number of bits in each code.");
        if ui.add(Slider::new(&mut self.code.length, 2..=10)).changed() {
            self.code.weight = self.code.weight.clamp(1, self.code.length - 1);
        }
        ui.add_space(8.0);

        ui.subheading("Weight");
        ui.label("Number of 1s in each code.");
        if ui.add(Slider::new(&mut self.code.weight, 1..=9)).changed() {
            self.code.weight = self.code.weight.clamp(1, self.code.length - 1);
        }
        ui.add_space(8.0);

        ui.label(format!("Total Codes: {}", self.code.total_codes()));
        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
