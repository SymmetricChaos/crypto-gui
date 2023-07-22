use codes::ecc::repetition::Repetition;
use egui::Slider;

use crate::ui_elements::UiElements;

use super::CodeFrame;

pub struct RepetitionFrame {
    code: Repetition,
}

impl Default for RepetitionFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
        }
    }
}

impl CodeFrame for RepetitionFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.subheading("Block Size");
        ui.add(Slider::new(&mut self.code.block_size, 3..=9));

        ui.add_space(16.0);

        if self.code.block_size % 2 == 0 {
            ui.label(format!(
                "Correct {}-bit errors\nDetect {}-bit errors",
                self.code.block_size / 2 - 1,
                self.code.block_size / 2
            ));
        } else {
            ui.label(format!("Correct {}-bit errors", self.code.block_size / 2));
        }
        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
