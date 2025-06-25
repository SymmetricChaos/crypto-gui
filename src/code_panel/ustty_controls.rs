use super::CodeFrame;
use crate::ui_elements::UiElements;
use codes::text_standards::ustty::{BitOrder, UsTty};

pub struct UsTtyFrame {
    code: UsTty,
}

impl Default for UsTtyFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
        }
    }
}

impl CodeFrame for UsTtyFrame {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/codes/src/text_standards/ustty.rs",
        );
        ui.add_space(8.0);

        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(
                    &mut self.code.bit_order,
                    BitOrder::LsbL,
                    "Least Significant Bit Left",
                );
                ui.selectable_value(
                    &mut self.code.bit_order,
                    BitOrder::LsbR,
                    "Least Significant Bit Right",
                );
            });
        });
        ui.add_space(8.0);

        ui.checkbox(&mut self.code.spaced, "Print Bits as Groups of Five");
        ui.add_space(8.0);

        ui.fill_code_columns(16, 4, self.code.codes_chars());
        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
