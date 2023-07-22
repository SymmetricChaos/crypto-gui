use codes::{
    commercial::upc::{is_valid_upc_a, upc_a_check_digit, Upc},
    errors::CodeError,
};
use itertools::Itertools;

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

fn handle_example(text: &str) -> Result<String, CodeError> {
    if !text.is_ascii() {
        return Err(CodeError::Input(format!(
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
            return Err(CodeError::Input(format!(
                "{} is not a valid UPC-A code",
                text
            )));
        }
    } else {
        return Err(CodeError::Input(format!(
            "{} is not a valid UPC-A code",
            text
        )));
    }

    Ok(out)
}

impl CodeFrame for UpcFrame {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _errors: &mut String) {
        // ui.group(|ui| {
        //     ui.subheading("Variant");
        //     ui.horizontal(|ui| {
        //         ui.selectable_value(&mut self.code.variant, , );

        //     });
        // });

        // match self.code.variant {

        // };

        ui.text_edit_singleline(&mut self.example);
        // ui.mono("036000291452");
        match handle_example(&self.example) {
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
                ui.error_text(e.inner());
            }
        }
        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
