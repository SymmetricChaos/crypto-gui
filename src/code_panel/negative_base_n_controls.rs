use codes::mathematical::negative_base_n::NegativeBaseN;
use egui::Slider;

use crate::ui_elements::UiElements;

use super::CodeFrame;

pub struct NegativeBaseNFrame {
    code: NegativeBaseN,
}

impl CodeFrame for NegativeBaseNFrame {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/codes/src/mathematical/negative_base_n.rs",
        );
        ui.add_space(8.0);

        ui.subheading("Base");
        ui.label(
            "Widely accepted representations for numbers in base-N exist only for values 2 to 36.",
        );
        ui.add(Slider::new(&mut self.code.radix, -2..=-36));
        ui.add_space(16.0);

        ui.label("Convert between \"standard\" base-10 numbers and their representation in a negative base. The first 32 encodings appear below.");
        let pairs = (0..32).map(|n| (n.to_string(), self.code.encode_i32(n).unwrap()));
        ui.fill_code_columns(16, 5, Box::new(pairs));

        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
