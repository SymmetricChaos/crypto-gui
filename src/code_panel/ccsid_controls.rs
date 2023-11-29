use super::CodeFrame;
use crate::ui_elements::UiElements;
use codes::text_standards::code_pages::{Ccsid, DisplayMode};

pub struct CcsidFrame {
    code: Ccsid,
}

impl Default for CcsidFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
        }
    }
}

impl CodeFrame for CcsidFrame {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.subheading("Representation");
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.code.mode, DisplayMode::Binary, "Binary");
                ui.selectable_value(&mut self.code.mode, DisplayMode::Octal, "Octal");
                ui.selectable_value(&mut self.code.mode, DisplayMode::Decimal, "Decimal");
                ui.selectable_value(&mut self.code.mode, DisplayMode::Hex, "Hexadecimal");
            });
        });
        ui.add_space(8.0);
        ui.checkbox(&mut self.code.spaced, "Use Spaces");
        ui.add_space(16.0);
        ui.two_column_table("Character", "Code", self.code.chars_codes());
        ui.add_space(32.0)
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
