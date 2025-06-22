use super::CodeFrame;
use crate::ui_elements::UiElements;
use codes::mathematical::base_n_bijective::BaseNBijective;
use egui::Slider;

pub struct BaseNBijectiveFrame {
    code: BaseNBijective,
}

impl Default for BaseNBijectiveFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
        }
    }
}

impl CodeFrame for BaseNBijectiveFrame {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/codes/src/mathematical/base_n_bijective.rs",
        );
        ui.add_space(8.0);

        ui.subheading("Base");
        ui.label(
                "Widely accepted representations for bijective numbers in base-N exist only for values 1 to 35.",
            );
        ui.add(Slider::new(&mut self.code.radix, 1..=35));
        ui.add_space(16.0);

        ui.label(format!(
            "The integers from 1 to 32 represented in bijetive base-{}",
            self.code.radix
        ));
        let pairs = (1..33).map(|n| (n.to_string(), self.code.encode_u32(n).unwrap()));
        ui.fill_code_columns(16, 5, Box::new(pairs));

        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
