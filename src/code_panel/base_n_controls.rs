use super::CodeFrame;
use crate::ui_elements::UiElements;
use codes::{mathematical::base_n::BaseN, traits::IOMode};
use egui::{Slider, TextEdit};
use utils::text_functions::unique_string;

pub struct BaseNFrame {
    code: BaseN,
}

impl Default for BaseNFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
        }
    }
}

impl CodeFrame for BaseNFrame {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.add_space(16.0);

        ui.group(|ui| {
            ui.subheading("Mode");
            ui.selectable_value(&mut self.code.mode, IOMode::Integer, "Integer");
            ui.selectable_value(&mut self.code.mode, IOMode::Letter, "Letter");
            ui.selectable_value(&mut self.code.mode, IOMode::Word, "Word");
        });
        ui.add_space(16.0);

        ui.subheading("Bijective Base-N");
        ui.label("Bijective representation covers all positive integers without using zero. This allows a bijection between numbers and their representations as no leading zeros can be added.");
        ui.checkbox(&mut self.code.bijective, "");

        if self.code.bijective {
            ui.subheading("Base");
            ui.label(
                "Widely accepted representations for bijective numbers in base-N exist only for values 1 to 35.",
            );
            ui.add(Slider::new(&mut self.code.radix, 1..=35));
            ui.add_space(16.0);
        } else {
            ui.subheading("Base");
            ui.label(
                "Widely accepted representations for numbers in base-N exist only for values 2 to 36.",
            );
            ui.add(Slider::new(&mut self.code.radix, 2..=36));
            ui.add_space(16.0);
        }

        match self.code.mode {
            IOMode::Letter => {
                ui.label("Provide an alphabet. Numerical codes, starting with zero, will be created in ascending order will be assigned to each character.");
                if ui.control_string(&mut self.code.maps.alphabet).changed() {
                    unique_string(&mut self.code.maps.alphabet);
                    self.code.maps.alphabet.retain(|x| x != '�');
                    self.code.set_letter_map();
                };
                ui.fill_code_columns(16, 5, Box::new(self.code.maps.chars_codes()));
            }
            IOMode::Word => {
                ui.label("Provide any number of words or phrases separated by commas. Numerical codes, starting with zero, will be created in ascending order will be assigned to each character.");
                if ui
                    .add(TextEdit::multiline(&mut self.code.maps.words_string))
                    .changed()
                {
                    self.code.set_word_map();
                };
                ui.fill_code_columns(16, 5, Box::new(self.code.maps.words_codes()));
            }
            IOMode::Integer => {
                if self.code.bijective {
                    ui.label("Convert between \"standard\" base-10 numbers and their representation in another a bijective base. The first 32 encodings appear below.");
                    let pairs = (1..33).map(|n| (n.to_string(), self.code.encode_u32(n).unwrap()));
                    ui.fill_code_columns(16, 5, Box::new(pairs));
                } else {
                    ui.label("Convert between \"standard\" base-10 numbers and their representation in another base. The first 32 encodings appear below.");
                    let pairs = (0..32).map(|n| (n.to_string(), self.code.encode_u32(n).unwrap()));
                    ui.fill_code_columns(16, 5, Box::new(pairs));
                }
            }
        }
        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
