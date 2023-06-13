use codes::binary_to_text::numeric::BytesAsNumbers;
use egui::Slider;

use crate::ui_elements::{binary_to_text_input_mode, fill_code_columns, subheading};

use super::CodeFrame;

pub struct BytesAsNumbersFrame {
    code: BytesAsNumbers,
}

impl Default for BytesAsNumbersFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
        }
    }
}

impl CodeFrame for BytesAsNumbersFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.add_space(16.0);
        ui.label(subheading("Radix"));
        ui.label("The radix (or base) of the representation can vary between 2 and 36 as there are standard ways of printing these numbers.");
        let radix_range = 2..=36;
        if ui
            .add(Slider::new(&mut self.code.radix, radix_range))
            .changed()
        {
            self.code.set_width()
        }
        ui.add_space(16.0);
        ui.label("Fixed width numbers insert zeroes to the left so that every number takes up the same amount of space. This may be easier to read.");
        ui.checkbox(&mut self.code.fixed_width, "Fixed Width");
        ui.add_space(16.0);
        binary_to_text_input_mode(ui, &mut self.code.mode);
        ui.add_space(16.0);

        fill_code_columns(32, 8, ui, Box::new(self.code.chars_codes()));
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
