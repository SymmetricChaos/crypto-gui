use super::CodeFrame;
use crate::ui_elements::UiElements;
use codes::mathematical::balanced_ternary::{encode_i32, BalancedTernary};

pub struct BalancedTernaryFrame {
    code: BalancedTernary,
}

impl Default for BalancedTernaryFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
        }
    }
}

impl CodeFrame for BalancedTernaryFrame {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/codes/src/mathematical/balanced_ternary.rs",
        );
        ui.add_space(8.0);

        ui.label("Convert between \"standard\" base-10 numbers and their representation in balanced ternary. Example encodings from -15 to 16 appear below.");
        let pairs = (-15..=16).map(|n| (n.to_string(), encode_i32(n).unwrap()));
        ui.fill_code_columns(16, 5, Box::new(pairs));

        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
