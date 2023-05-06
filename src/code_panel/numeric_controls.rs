use egui::Slider;

use crate::{codes::binary_to_text::numeric::BytesAsNumbers, egui_aux::subheading};

use super::{
    generic_components::{binary_to_text_input_mode, fill_code_columns},
    View, ViewableCode,
};

impl ViewableCode for BytesAsNumbers {}

impl View for BytesAsNumbers {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _errors: &mut String) {
        ui.add_space(16.0);
        ui.label(subheading("Radix"));
        ui.label("The radix (or base) of the representation can vary between 2 and 36 as there are standard ways of printing these numbers.");
        let radix_range = 2..=36;
        if ui.add(Slider::new(&mut self.radix, radix_range)).changed() {
            self.set_width()
        }
        ui.add_space(16.0);
        ui.label("Fixed width numbers insert zeroes to the left so that every number takes up the same amount of space. This may be easier to read.");
        ui.checkbox(&mut self.fixed_width, "Fixed Width");
        ui.add_space(16.0);
        binary_to_text_input_mode(ui, &mut self.mode);
        ui.add_space(16.0);

        fill_code_columns(32, 8, ui, Box::new(self.chars_codes()));
    }
}
