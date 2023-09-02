use super::CodeFrame;
use crate::ui_elements::UiElements;
use codes::ecc::cyclic_redundancy_check::CyclicRedundancyCheck;
use utils::bit_polynomial::BitPolynomial;

pub struct CyclicRedundancyCheckFrame {
    code: CyclicRedundancyCheck,
    generator_string: String,
    generator_string_err: String,
}

impl Default for CyclicRedundancyCheckFrame {
    fn default() -> Self {
        let code = CyclicRedundancyCheck::default();
        let generator_string: String = code.generator.to_string();
        Self {
            code,
            generator_string,
            generator_string_err: String::new(),
        }
    }
}

impl CodeFrame for CyclicRedundancyCheckFrame {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.subheading("Generator Polynomial");
        if ui.control_string(&mut self.generator_string).changed() {
            match BitPolynomial::from_str(&self.generator_string) {
                Ok(g) => {
                    self.generator_string_err.clear();
                    self.code.generator = g;
                }
                Err(e) => self.generator_string_err = e.to_string(),
            }
        }
        ui.label(self.code.generator.polynomial_string());
        if !self.generator_string_err.is_empty() {
            ui.label(&self.generator_string_err);
        }

        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
