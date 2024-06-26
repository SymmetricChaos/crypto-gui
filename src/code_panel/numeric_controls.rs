use super::CodeFrame;
use crate::ui_elements::UiElements;
use codes::binary_to_text::numeric::BytesAsNumbers;
use egui::Slider;

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
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/codes/src/binary_to_text/numeric.rs",
        );
        ui.add_space(8.0);

        ui.subheading("Radix");
        ui.label("The radix (or base) of the representation can vary between 2 and 36 as there are standard ways of printing these.");
        let radix_range = 2..=36;
        if ui
            .add(Slider::new(&mut self.code.radix, radix_range))
            .changed()
        {
            self.code.set_width()
        }
        ui.add_space(16.0);
        ui.subheading("Fixed Width");
        ui.label("Fixed width numbers insert zeroes to the left so that every number takes up the same amount of space. This may be easier to read.");
        ui.checkbox(&mut self.code.fixed_width, "Fixed Width");

        ui.add_space(16.0);
        ui.subheading("Digit Order");
        ui.label("Digits in little-endian order are written in the \"usual\" way starting with the most significant digits with place value decreasing from left-to-right. Numbers may also be written with digits in reverse order.");
        ui.selectable_value(&mut self.code.little_endian, true, "little-endian");
        ui.selectable_value(&mut self.code.little_endian, false, "big-endian");
        ui.add_space(16.0);

        ui.add_space(16.0);
        ui.binary_to_text_input_mode(&mut self.code.mode);
        ui.add_space(16.0);

        ui.fill_code_columns(32, 8, Box::new(self.code.chars_codes()));
        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
