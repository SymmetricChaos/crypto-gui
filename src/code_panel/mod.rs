use crate::{codes::*, ids::CodeID};
use eframe::egui;
use egui::Ui;
pub mod _generic_components;
pub mod ascii_controls;
pub mod bacon_contols;
pub mod base64_controls;
pub mod baudot_controls;
pub mod fibonacci_controls;
pub mod generic_components;
pub mod godel_controls;
pub mod morse_american_controls;
pub mod morse_controls;
pub mod spelling_alphabet_controls;
pub mod unary_controls;

pub trait ViewableCode: View + Code {}

pub trait View {
    fn ui(&mut self, ui: &mut Ui, errors: &mut String);
}

// Quick simple combo box builder
fn combox_box(code: &[CodeID], identifier: &'static str, active_code: &mut CodeID, ui: &mut Ui) {
    egui::ComboBox::from_id_source(identifier)
        .selected_text(identifier)
        .show_ui(ui, |ui| {
            for id in code {
                ui.selectable_value(active_code, *id, id.to_string());
            }
        });
    ui.add_space(10.0);
}

#[derive(Default)]
pub struct CodeInterface {
    ascii: Ascii,
    morse_itu: MorseITU,
    morse_american: MorseAmerican,
    unary: UnaryCode,
    phonetic: SpellingAlphabet,
    godel: Godel,
}

// Morse,    // MorseAmerican, Morse ITU
// Binary,   // Baudot, Ascii, Bacon, Fibonacci, Base64
// Unary,    // Unary
// Spelling, // Pgp, Phonetic
// Godel,

impl CodeInterface {
    pub fn combo_boxes(&mut self, ui: &mut Ui, active_code: &mut CodeID) {
        combox_box(&[CodeID::Ascii], "Binary", active_code, ui);

        combox_box(
            &[CodeID::MorseITU, CodeID::MorseAmerican],
            "Morse",
            active_code,
            ui,
        );

        combox_box(&[CodeID::Unary], "Unary", active_code, ui);

        combox_box(&[CodeID::SpellingAlphabet], "Spelling", active_code, ui);

        combox_box(&[CodeID::Godel], "Godel", active_code, ui);
    }

    pub fn get_active_code(&mut self, active_code: &CodeID) -> &mut dyn ViewableCode {
        match active_code {
            CodeID::Ascii => &mut self.ascii,
            CodeID::MorseAmerican => &mut self.morse_american,
            CodeID::MorseITU => &mut self.morse_itu,
            CodeID::Godel => &mut self.godel,
            CodeID::Fibonacci => todo!(),
            CodeID::Baudot => todo!(),
            CodeID::Base64 => todo!(),
            CodeID::Pgp => todo!(),
            CodeID::Unary => &mut self.unary,
            CodeID::SpellingAlphabet => &mut self.phonetic,
            CodeID::Bacon => todo!(),
            _ => todo!("unable to get active code"),
        }
    }
}

// pub trait View {
//     fn ui(
//         &mut self,
//         ui: &mut egui::Ui,
//         input: &mut String,
//         _output: &mut String,
//         _errors: &mut String,
//     );
// }

// fn combox_box(
//     code: &[CodeID],
//     identifier: &'static str,
//     active_code: &mut CodeID,
//     ui: &mut egui::Ui,
// ) {
//     egui::ComboBox::from_id_source(identifier)
//         .selected_text(identifier)
//         .show_ui(ui, |ui| {
//             for id in code {
//                 ui.selectable_value(active_code, *id, format!("{}", id));
//             }
//         });
//     ui.add_space(10.0);
// }

// #[derive(Default)]
// pub struct CodeControlPanel {
//     ascii: Ascii,
//     base64: Base64,
//     morse_itu: MorseITU,
//     morse_american: MorseAmerican,
//     godel: Godel,
//     fibonacci: FibonacciCode,
//     unary: UnaryCode,
//     spelling_alphabet: SpellingAlphabet,
//     bacon: Bacon,
// }

// impl CodeControlPanel {
//     pub fn ui(
//         &mut self,
//         ui: &mut egui::Ui,
//         active_code: &mut CodeID,
//         input: &mut String,
//         output: &mut String,
//         errors: &mut String,
//     ) {
//         egui::Grid::new("comboboxes").show(ui, |ui| {
//             combox_box(
//                 &[
//                     CodeID::Ascii,
//                     CodeID::Fibonacci,
//                     CodeID::Unary,
//                     CodeID::Base64,
//                     CodeID::Bacon,
//                 ],
//                 "Binary Codes",
//                 active_code,
//                 ui,
//             );

//             combox_box(
//                 &[CodeID::MorseITU, CodeID::MorseAmerican],
//                 "Morse Code",
//                 active_code,
//                 ui,
//             );

//             combox_box(
//                 &[CodeID::Godel, CodeID::SpellingAlphabet],
//                 "Other Codes",
//                 active_code,
//                 ui,
//             );
//         });

//         ui.add_space(16.0);
//         ui.separator();
//         ui.add_space(16.0);

//         let name = RichText::new(String::from(*active_code)).strong().heading();
//         ui.add(egui::Label::new(name));
//         ui.label(active_code.description());

//         ui.add_space(16.0);
//         ui.separator();
//         ui.add_space(16.0);

//         match active_code {
//             CodeID::Ascii => self.ascii.ui(ui, input, output, errors),
//             CodeID::Base64 => self.base64.ui(ui, input, output, errors),
//             CodeID::MorseITU => self.morse_itu.ui(ui, input, output, errors),
//             CodeID::Godel => self.godel.ui(ui, input, output, errors),
//             CodeID::Fibonacci => self.fibonacci.ui(ui, input, output, errors),
//             CodeID::MorseAmerican => self.morse_american.ui(ui, input, output, errors),
//             CodeID::Unary => self.unary.ui(ui, input, output, errors),
//             CodeID::SpellingAlphabet => self.spelling_alphabet.ui(ui, input, output, errors),
//             CodeID::Bacon => self.bacon.ui(ui, input, output, errors),
//             _ => {
//                 ui.label("IN PROGRESS");
//             }
//         }
//     }
// }

// pub struct CodeDisplayPanel {}

// impl Default for CodeDisplayPanel {
//     fn default() -> Self {
//         Self {}
//     }
// }

// impl CodeDisplayPanel {
//     pub fn ui(
//         &mut self,
//         ui: &mut egui::Ui,
//         input: &mut String,
//         output: &mut String,
//         errors: &mut String,
//         active_code: &mut CodeID,
//         control_panel: &CodeControlPanel,
//     ) {
//         ui.add_space(32.0);
//         ui.label("INPUT TEXT");
//         ui.add(TextEdit::multiline(input).font(TextStyle::Monospace));
//         ui.add_space(16.0);
//         ui.label("OUTPUT TEXT");
//         ui.add(TextEdit::multiline(output).font(TextStyle::Monospace));

//         match active_code {
//             CodeID::Ascii => encode_decode(ui, &control_panel.ascii, input, output, errors),
//             CodeID::Base64 => encode_decode(ui, &control_panel.base64, input, output, errors),
//             CodeID::MorseITU => encode_decode(ui, &control_panel.morse_itu, input, output, errors),
//             CodeID::MorseAmerican => {
//                 encode_decode(ui, &control_panel.morse_american, input, output, errors)
//             }
//             CodeID::Godel => encode_decode(ui, &control_panel.godel, input, output, errors),
//             CodeID::Fibonacci => encode_decode(ui, &control_panel.fibonacci, input, output, errors),
//             CodeID::Unary => encode_decode(ui, &control_panel.unary, input, output, errors),
//             CodeID::SpellingAlphabet => {
//                 encode_decode(ui, &control_panel.spelling_alphabet, input, output, errors)
//             }
//             CodeID::Bacon => encode_decode(ui, &control_panel.bacon, input, output, errors),
//             _ => *errors = String::from("button must be added to DisplayPanel struct"),
//         }

//         ui.add_space(10.0);
//         if ui.button("clear").clicked() {
//             input.clear();
//             output.clear();
//             errors.clear();
//         }

//         ui.add_space(10.0);
//         if ui.button("swap input/output").clicked() {
//             std::mem::swap(input, output)
//         }

//         if !errors.is_empty() {
//             ui.add_space(24.0);
//             ui.label(
//                 RichText::new(errors.clone())
//                     .color(Color32::RED)
//                     .background_color(Color32::BLACK)
//                     .monospace(),
//             );
//         }
//     }
// }
