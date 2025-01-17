use super::CodeFrame;
use crate::ui_elements::UiElements;
use codes::mathematical::base_n::BaseN;
use egui::Slider;

pub struct BaseNFrame {
    code: BaseN,
}

impl Default for BaseNFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
        }
    }
}

impl CodeFrame for BaseNFrame {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/codes/src/mathematical/base_n.rs",
        );
        ui.add_space(8.0);

        ui.subheading("Base");
        ui.label(
            "Widely accepted representations for numbers in base-N exist only for values 2 to 36.",
        );
        ui.add(Slider::new(&mut self.code.radix, 2..=36));
        ui.add_space(16.0);

        ui.label(format!("The first 32 equivalences between standard base-10 numbers and their representation in base-{}",self.code.radix));
        let pairs = (0..32).map(|n| (n.to_string(), self.code.encode_u32(n).unwrap()));
        ui.fill_code_columns(16, 5, Box::new(pairs));

        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
