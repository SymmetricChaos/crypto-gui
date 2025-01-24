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

        ui.subheading("Signed");
        ui.label("The Golomb codes can be extended to all integers by assigning negative integer to odd values and all others to even values.");
        ui.checkbox(&mut self.code.signed, "Use Signed");
        ui.add_space(8.0);

        ui.subheading("Separated");
        ui.label("A prefix code can be read without inserting spaces or commas. With this set the output will be comma separated.");
        ui.checkbox(&mut self.code.spaced, "Use Separator");
        ui.add_space(8.0);

        ui.label("A sample list of encodings:");
        ui.add_space(4.0);
        if self.code.signed {
            ui.two_column_table(
                "Integer",
                "Code",
                Box::new(
                    (-5..=5)
                        .into_iter()
                        .map(|n| (n, self.code.i32_to_golomb(n))),
                ),
            );
        } else {
            ui.two_column_table(
                "Integer",
                "Code",
                Box::new((0..=9).into_iter().map(|n| (n, self.code.u32_to_golomb(n)))),
            );
        }

        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
