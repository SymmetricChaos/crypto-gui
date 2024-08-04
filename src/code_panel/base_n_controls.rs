use super::CodeFrame;
use crate::ui_elements::UiElements;
use codes::{letter_word_code::IOMode, mathematical::base_n::BaseN};
use egui::{Slider, TextEdit};
use strum::IntoEnumIterator;
use utils::text_functions::unique_string;

pub struct BaseNFrame {
    code: BaseN,
    words_string: String,
}

impl Default for BaseNFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
            words_string: String::new(),
        }
    }
}

impl CodeFrame for BaseNFrame {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/codes/src/mathematical/base_n.rs",
        );
        ui.add_space(8.0);

        ui.group(|ui| {
            ui.subheading("Mode");
            for variant in IOMode::iter() {
                ui.selectable_value(&mut self.code.mode, variant, variant.to_string());
            }
        });
        ui.add_space(16.0);

        // ui.subheading("Bijective Base-N");
        // ui.label("Bijective representation covers all positive integers without using zero. This allows a bijection between numbers and their representations as no leading zeros can be added.");
        // ui.checkbox(&mut self.code.bijective, "");

        // if self.code.bijective {
        //     ui.subheading("Base");
        //     ui.label(
        //         "Widely accepted representations for bijective numbers in base-N exist only for values 1 to 35.",
        //     );
        //     ui.add(Slider::new(&mut self.code.radix, 1..=35));
        //     ui.add_space(16.0);
        // } else {
        ui.subheading("Base");
        ui.label(
            "Widely accepted representations for numbers in base-N exist only for values 2 to 36.",
        );
        ui.add(Slider::new(&mut self.code.radix, 2..=36));
        ui.add_space(16.0);
        // }

        match self.code.mode {
            IOMode::Letter => {
                ui.label("Provide an alphabet. Numerical codes, starting with zero, will be created in ascending order will be assigned to each character.");
                if ui.control_string(&mut self.code.maps.alphabet).changed() {
                    unique_string(&mut self.code.maps.alphabet);
                    self.code.maps.alphabet.retain(|x| x != '�');
                };
                // ui.fill_code_columns(16, 5, Box::new(self.code.maps.chars_codes()));
            }
            IOMode::Word => {
                ui.label("Provide any number of words or phrases separated by commas. Numerical codes, starting with zero, will be created in ascending order will be assigned to each character.");
                if ui
                    .add(TextEdit::multiline(&mut self.words_string))
                    .changed()
                {
                    self.code.maps.set_words(&self.words_string)
                };
                // ui.fill_code_columns(16, 5, Box::new(self.code.maps.words_codes()));
            }
            IOMode::Integer => {
                // if self.code.bijective {
                //     ui.label("Convert between \"standard\" base-10 numbers and their representation in another a bijective base. The first 32 encodings appear below.");
                //     let pairs = (1..33).map(|n| (n.to_string(), self.code.encode_u32(n).unwrap()));
                //     ui.fill_code_columns(16, 5, Box::new(pairs));
                // } else {
                ui.label("Convert between \"standard\" base-10 numbers and their representation in another base. The first 32 encodings appear below.");
                let pairs = (0..32).map(|n| (n.to_string(), self.code.encode_u32(n).unwrap()));
                ui.fill_code_columns(16, 5, Box::new(pairs));
                // }
            }
        }
        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
