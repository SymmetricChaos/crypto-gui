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

impl CyclicRedundancyCheckFrame {
    fn set_generator(&mut self, s: &str) {
        self.generator_string = String::from(s);
        self.generator_string_err.clear();
        self.code.generator = BitPolynomial::from_str(&self.generator_string).unwrap();
    }
}

impl CodeFrame for CyclicRedundancyCheckFrame {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.subheading("CRC Standards");
            ui.label("While many generator polyomials can be used not all are equally effective and various standard choices exist.");
            ui.horizontal(|ui| {
                if ui.button("CRC-3-GSM").clicked() {
                    self.set_generator("1101");
                }
                if ui.button("CRC-4-ITU").clicked() {
                    self.set_generator("11001");
                }
                if ui.button("CRC-5-EPC").clicked() {
                    self.set_generator("100101");
                }
                if ui.button("CRC-5-ITU").clicked() {
                    self.set_generator("101011");
                }
                if ui.button("CRC-5-USB").clicked() {
                    self.set_generator("101001");
                }
                if ui.button("CRC-6-GSM").clicked() {
                    self.set_generator("1111011");
                }
                if ui.button("CRC-6-ITU").clicked() {
                    self.set_generator("1100001");
                }
                if ui.button("CRC-32").clicked() {
                    self.set_generator("111011011011100010000011001000001");
                }
            });
        });

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
