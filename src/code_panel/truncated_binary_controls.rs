use super::CodeFrame;
use crate::ui_elements::{prefix_code_sep, UiElements};
use codes::mathematical::truncated_binary::TruncatedBinary;
use egui::DragValue;

pub struct TruncatedBinaryFrame {
    code: TruncatedBinary,
    alphabet_size: u32,
}

impl Default for TruncatedBinaryFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
            alphabet_size: 10,
        }
    }
}

impl TruncatedBinaryFrame {}

impl CodeFrame for TruncatedBinaryFrame {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/codes/src/mathematical/truncated_binary.rs",
        );
        ui.add_space(8.0);

        ui.subheading("Alphabet Size");
        if ui
            .add(DragValue::new(&mut self.alphabet_size).range(2..=64))
            .changed()
        {
            self.code.set_consts(self.alphabet_size);
        }
        ui.label(&format!(
            "maximum length (k) = {}",
            self.alphabet_size.ilog2() + 1
        ));
        ui.label(&format!(
            "cutoff (u): = {}",
            (1 << (self.alphabet_size.ilog2() + 1)) - self.alphabet_size
        ));
        ui.add_space(8.0);

        prefix_code_sep(ui, &mut self.code.spaced);

        ui.label("A sample list of encodings:");
        ui.two_column_table(
            "Code",
            "Integer",
            Box::new(
                (0..self.alphabet_size)
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
