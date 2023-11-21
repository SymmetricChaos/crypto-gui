use super::CodeFrame;
use crate::ui_elements::UiElements;
use codes::other::barbier::BarbierCode;

pub struct BarbierFrame {
    code: BarbierCode,
}

impl Default for BarbierFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
        }
    }
}

impl CodeFrame for BarbierFrame {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.add_space(8.0);

        ui.subheading("Grid");
        ui.mono(self.code.show_grid());
        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
