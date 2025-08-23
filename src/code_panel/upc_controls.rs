use codes::commercial::upc::{is_valid_upc_a, upc_a_check_digit, Upc};
use itertools::Itertools;
use utils::errors::GeneralError;

use crate::ui_elements::UiElements;

use super::CodeFrame;

pub struct UpcFrame {
    pub code: Upc,
    pub example: String,
}

impl Default for UpcFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
            example: String::from("036000291452"),
        }
    }
}

fn upca_example(text: &str) -> Result<String, GeneralError> {
    if !text.is_ascii() {
        return Err(GeneralError::input(format!(
            "{} is not a valid UPC-A code",
            text
        )));
    }

    let mut out = String::with_capacity(12);
    out.push_str(text);
    if text.len() == 11 {
        out.push(upc_a_check_digit(text)?);
    } else if text.len() == 12 {
        if !is_valid_upc_a(text) {
            return Err(GeneralError::input(format!(
                "{} is not a valid UPC-A code",
                text
            )));
        }
    } else {
        return Err(GeneralError::input(format!(
            "{} is not a valid UPC-A code",
            text
        )));
    }

    Ok(out)
}

impl CodeFrame for UpcFrame {
    fn ui(&mut self, ui: &mut eframe::egui::Ui) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/codes/src/commercial/upc.rs",
        );
        ui.add_space(8.0);

        // ui.group(|ui| {
        //     ui.subheading("Variant");
        //     ui.horizontal(|ui| {
        //         ui.selectable_value(&mut self.code.variant, , );

        //     });
        // });

        // match self.code.variant {

        // };

        ui.subheading("Example UPC-A");
        if ui.text_edit_singleline(&mut self.example).changed() {
            while self.example.chars().count() > 12 {
                self.example.pop();
            }
        }
        match upca_example(&self.example) {
            Ok(digits) => {
                ui.horizontal(|ui| {
                    ui.mono(digits.chars().join(" "));

                    ui.mono(" (digits)");
                });
                ui.mono("1 3 1 3 1 3 1 3 1 3 1 3 (weights)");
                ui.horizontal(|ui| {
                    ui.mono(
                        digits
                            .chars()
                            .map(|c| c.to_digit(10).unwrap())
                            .zip([1, 3].into_iter().cycle())
                            .map(|(a, b)| (a * b) % 10)
                            .join(" "),
                    );
                    ui.mono("(weighted values)");
                });
                ui.label("The weighted sum is always a multiple of ten due to the check digit.");
            }
            Err(e) => {
                ui.error_text(e);
            }
        }
        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
