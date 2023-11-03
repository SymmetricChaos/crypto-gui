use super::CodeFrame;
use crate::ui_elements::UiElements;
use codes::binary_to_text::basex::BaseX;

pub struct BaseXFrame {
    code: BaseX,
    alphabet_string: String,
}

impl Default for BaseXFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
            alphabet_string: String::from(
                "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz",
            ),
        }
    }
}

impl CodeFrame for BaseXFrame {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.subheading("Alphabet");
        ui.label(format!("Base{}", self.code.base()));
        if ui.control_string(&mut self.alphabet_string).changed() {
            self.code.set_map(&self.alphabet_string)
        }

        ui.group(|ui| {
            ui.subheading("Common Alphabets");
            ui.horizontal(|ui| {
                for (name, alphabet) in [
                    ("Base36", "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ"),
                    (
                        "Base56",
                        "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghjkmnpqrstuvwxyz",
                    ),
                    (
                        "Base58",
                        "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz",
                    ),
                    (
                        "Base62",
                        "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz",
                    ),
                ] {
                    if ui.button(name).clicked() {
                        self.alphabet_string = alphabet.into();
                        self.code.set_map(&self.alphabet_string)
                    }
                }
            });
        });

        ui.fill_code_columns(16, 4, Box::new(self.code.chars_codes()));
        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
