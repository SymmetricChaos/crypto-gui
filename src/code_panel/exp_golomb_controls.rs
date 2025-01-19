use super::CodeFrame;
use crate::ui_elements::UiElements;
use codes::mathematical::exp_golomb::{u32_to_exp_golomb, ExpGolomb};

pub struct ExpGolombFrame {
    code: ExpGolomb,
}

impl Default for ExpGolombFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
        }
    }
}

impl ExpGolombFrame {}

impl CodeFrame for ExpGolombFrame {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/codes/src/mathematical/exp_golomb.rs",
        );
        ui.add_space(8.0);

        ui.subheading("Separated");
        ui.label("A prefix code can be read without inserting spaces or commas. With this set the output will be comma separated.");
        ui.checkbox(&mut self.code.spaced, "Use Separator");
        ui.add_space(8.0);

        ui.label("A sample list of encodings:");
        ui.two_column_table(
            "Code",
            "Integer",
            Box::new((0..=16).into_iter().map(|n| (n, u32_to_exp_golomb(n)))),
        );

        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
