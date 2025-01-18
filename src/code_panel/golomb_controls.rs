use super::CodeFrame;
use crate::ui_elements::UiElements;
use codes::mathematical::golomb::Golomb;
use egui::DragValue;

pub struct GolombFrame {
    code: Golomb,
    modulus: u32,
}

impl Default for GolombFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
            modulus: 3,
        }
    }
}

impl GolombFrame {}

impl CodeFrame for GolombFrame {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/codes/src/mathematical/golomb.rs",
        );
        ui.add_space(8.0);

        ui.subheading("Modulus");
        if ui
            .add(DragValue::new(&mut self.modulus).range(3..=64))
            .changed()
        {
            self.code.set_modulus(self.modulus);
        }
        ui.add_space(8.0);

        ui.checkbox(&mut self.code.spaced, "Use Spaces");
        ui.add_space(8.0);

        ui.label("A sample list of encodings:");
        ui.two_column_table(
            "Code",
            "Integer",
            Box::new(
                (0..self.modulus)
                    .into_iter()
                    .map(|n| (n, self.code.u32_to_bits(n))),
            ),
        );

        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
