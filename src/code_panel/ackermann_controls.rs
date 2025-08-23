use super::CodeFrame;
use crate::ui_elements::UiElements;
use codes::mathematical::ackermann::{number_to_set, Ackermann};

pub struct AckermannFrame {
    code: Ackermann,
}

impl Default for AckermannFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
        }
    }
}

impl CodeFrame for AckermannFrame {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/codes/src/mathematical/ackermann.rs",
        );
        ui.add_space(8.0);

        ui.label("The first few sets and their associated integers.");
        ui.add_space(4.0);
        let pairs = (0..16).map(|n| (n.to_string(), number_to_set(n)));
        ui.fill_code_columns(16, 2, Box::new(pairs));

        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
