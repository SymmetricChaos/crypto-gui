use super::CodeFrame;
use codes::mathematical::godel::Godel;
use egui::TextEdit;
use itertools::Itertools;

pub struct GodelFrame {
    code: Godel,
    words_string: String,
}

impl Default for GodelFrame {
    fn default() -> Self {
        let code = Godel::default();
        let words_string = code.words.join(", ");
        Self { code, words_string }
    }
}

impl CodeFrame for GodelFrame {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/codes/src/mathematical/godel.rs",
        );
        ui.add_space(8.0);

        ui.label("Provide any number of symbols or words separated by commas. Each is assigned a sequential numeric value starting from 1. When decoding the '�' symbol is used for codes with an assigned meaning.");
        if ui
            .add(TextEdit::singleline(&mut self.words_string))
            .lost_focus()
        {
            self.code.words = self
                .words_string
                .split(',')
                .map(|s| s.trim().to_string())
                .collect_vec();
        };
        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
