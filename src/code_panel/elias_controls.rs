use super::CodeFrame;
use crate::ui_elements::{invert_bits, prefix_code_sep, UiElements};
use codes::{
    mathematical::elias::{elias::EliasCode, EliasVariant},
    traits::Code,
};

pub struct EliasCodeFrame {
    code: EliasCode,
}

impl Default for EliasCodeFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
        }
    }
}

impl CodeFrame for EliasCodeFrame {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/codes/src/mathematical/elias",
        );
        ui.add_space(8.0);

        ui.group(|ui| {
            ui.subheading("Variant");
            ui.selectable_value(&mut self.code.variant, EliasVariant::Delta, "Delta δ");
            ui.selectable_value(&mut self.code.variant, EliasVariant::Gamma, "Gamma γ");
            ui.selectable_value(&mut self.code.variant, EliasVariant::Omega, "Omega ω");
        });
        ui.add_space(8.0);

        prefix_code_sep(ui, &mut self.code.spaced);

        invert_bits(ui, &mut self.code.invert);

        ui.label("A sample list of encodings:");
        ui.two_column_table(
            "Integer",
            "Code",
            Box::new(self.code.n_pairs(32).into_iter()),
        );

        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn Code {
        &self.code
    }
}
