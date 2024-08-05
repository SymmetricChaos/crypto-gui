use super::CodeFrame;
use crate::ui_elements::{integer_letter_code_controls, integer_word_code_controls, UiElements};
use codes::{letter_word_code::IOMode, mathematical::base_n_bijective::BaseNBijective};
use egui::Slider;
use strum::IntoEnumIterator;

pub struct BaseNBijectiveFrame {
    code: BaseNBijective,
    words_string: String,
}

impl Default for BaseNBijectiveFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
            words_string: String::new(),
        }
    }
}

impl CodeFrame for BaseNBijectiveFrame {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/codes/src/mathematical/base_n_bijective.rs",
        );
        ui.add_space(8.0);

        ui.group(|ui| {
            ui.subheading("Mode");
            for variant in IOMode::iter() {
                ui.selectable_value(&mut self.code.mode, variant, variant.to_string());
            }
        });
        ui.add_space(16.0);

        ui.subheading("Base");
        ui.label(
                "Widely accepted representations for bijective numbers in base-N exist only for values 1 to 35.",
            );
        ui.add(Slider::new(&mut self.code.radix, 1..=35));
        ui.add_space(16.0);

        match self.code.mode {
            IOMode::Letter => {
                integer_letter_code_controls(ui, &mut self.code.maps.alphabet);
                // ui.fill_code_columns(16, 5, Box::new(self.code.maps.chars_codes()));
            }
            IOMode::Word => {
                integer_word_code_controls(ui, &mut self.words_string, &mut self.code.maps);
                // ui.fill_code_columns(16, 5, Box::new(self.code.maps.words_codes()));
            }
            IOMode::Integer => {
                ui.label("Convert between \"standard\" base-10 numbers and their representation in a bijective base. The first 32 encodings appear below.");
                let pairs = (1..33).map(|n| (n.to_string(), self.code.encode_u32(n).unwrap()));
                ui.fill_code_columns(16, 5, Box::new(pairs));
            }
        }
        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
