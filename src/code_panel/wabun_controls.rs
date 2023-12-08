use codes::text_standards::wabun::{Wabun, WabunRep};

use super::CodeFrame;
use crate::ui_elements::UiElements;

pub struct WabunFrame {
    code: Wabun,
}

impl Default for WabunFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
        }
    }
}

impl CodeFrame for WabunFrame {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.subheading("Representation");
        ui.horizontal(|ui| {
            ui.selectable_value(
                &mut self.code.representation,
                WabunRep::Ascii,
                "ASCII symbols",
            );
            ui.selectable_value(
                &mut self.code.representation,
                WabunRep::HalfBlock,
                "Halfblock (Line Code)",
            );
            ui.selectable_value(
                &mut self.code.representation,
                WabunRep::Word,
                "Dit/Dah (Words)",
            );
        });

        ui.add_space(16.0);
        ui.fill_code_columns(30, 4, Box::new(self.code.chars_codes()));
        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
