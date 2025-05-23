use super::CodeFrame;
use crate::ui_elements::UiElements;
use codes::mathematical::biquinary_decimal::{BiQuinaryMode, BiquinaryDecimal};

pub struct BiquinaryDecimalFrame {
    code: BiquinaryDecimal,
}

impl Default for BiquinaryDecimalFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
        }
    }
}

impl CodeFrame for BiquinaryDecimalFrame {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/codes/src/mathematical/biquinary_decimal.rs",
        );
        ui.add_space(8.0);

        ui.group(|ui| {
            ui.subheading("Mode");
            ui.horizontal(|ui| {
                ui.selectable_value(
                    &mut self.code.mode,
                    BiQuinaryMode::TwoOfSeven,
                    "Two-of-Seven",
                );
                ui.selectable_value(&mut self.code.mode, BiQuinaryMode::Abacus, "Abacus");
            });
        });

        match self.code.mode {
            BiQuinaryMode::TwoOfSeven => ui.label("The simplest way to represent biquinary decimal digitally is with a two-of-seven code. Every valid number has exactly two bits set. This is equivalent to inverting the lower bits of the abacus representation."),
            BiQuinaryMode::Abacus => ui.label("Typically a physical abacus encoded the ones digit as the number of beads above the gap. Here the number of one bits to the right of the zero bit define the ones place."),
        };

        ui.add_space(16.0);
        ui.two_column_table("Character", "Code", Box::new(self.code.chars_codes()));
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
