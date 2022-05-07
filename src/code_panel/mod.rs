use crate::code_id::CodeID;
use crate::codes::*;
use eframe::egui::{self, Color32, RichText, TextEdit, TextStyle};
pub mod generic_components;
use self::generic_components::encode_decode;

pub mod ascii_controls;
pub mod base64_controls;
pub mod fibonacci_controls;
pub mod godel_controls;
pub mod morse_american_controls;
pub mod morse_controls;
pub mod spelling_alphabet_controls;
pub mod unary_controls;

pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui);
}

fn combox_box(
    code: &[CodeID],
    identifier: &'static str,
    active_code: &mut CodeID,
    ui: &mut egui::Ui,
) {
    egui::ComboBox::from_id_source(identifier)
        .selected_text(identifier)
        .show_ui(ui, |ui| {
            for id in code {
                ui.selectable_value(active_code, *id, format!("{}", id));
            }
        });
    ui.add_space(10.0);
}

#[derive(Default)]
pub struct CodeControlPanel {
    ascii: Ascii,
    base64: Base64,
    morse_itu: MorseITU,
    morse_american: MorseAmerican,
    godel: Godel,
    fibonacci: FibonacciCode,
    unary: UnaryCode,
    spelling_alphabet: SpellingAlphabet,
}

impl CodeControlPanel {
    pub fn ui(&mut self, ui: &mut egui::Ui, active_code: &mut CodeID) {
        egui::Grid::new("comboboxes").show(ui, |ui| {
            combox_box(
                &[
                    CodeID::Ascii,
                    CodeID::Fibonacci,
                    CodeID::Unary,
                    CodeID::Base64,
                ],
                "Binary Codes",
                active_code,
                ui,
            );

            combox_box(
                &[CodeID::MorseITU, CodeID::MorseAmerican],
                "Morse Code",
                active_code,
                ui,
            );

            combox_box(
                &[CodeID::Godel, CodeID::SpellingAlphabet],
                "Other Codes",
                active_code,
                ui,
            );
        });

        ui.add_space(16.0);
        ui.separator();
        ui.add_space(16.0);

        let name = RichText::new(String::from(*active_code)).strong().heading();
        ui.add(egui::Label::new(name));
        ui.label(active_code.description());

        ui.add_space(16.0);
        ui.separator();
        ui.add_space(16.0);

        match active_code {
            CodeID::Ascii => self.ascii.ui(ui),
            CodeID::Base64 => self.base64.ui(ui),
            CodeID::MorseITU => self.morse_itu.ui(ui),
            CodeID::Godel => self.godel.ui(ui),
            CodeID::Fibonacci => self.fibonacci.ui(ui),
            CodeID::MorseAmerican => self.morse_american.ui(ui),
            CodeID::Unary => self.unary.ui(ui),
            CodeID::SpellingAlphabet => self.spelling_alphabet.ui(ui),
            _ => {
                ui.label("IN PROGRESS");
            }
        }
    }
}

pub struct CodeDisplayPanel {}

impl Default for CodeDisplayPanel {
    fn default() -> Self {
        Self {}
    }
}

impl CodeDisplayPanel {
    pub fn ui(
        &mut self,
        ui: &mut egui::Ui,
        input: &mut String,
        output: &mut String,
        errors: &mut String,
        active_code: &mut CodeID,
        control_panel: &CodeControlPanel,
    ) {
        ui.add_space(32.0);
        ui.label("INPUT TEXT");
        ui.add(TextEdit::multiline(input).font(TextStyle::Monospace));
        ui.add_space(16.0);
        ui.label("OUTPUT TEXT");
        ui.add(TextEdit::multiline(output).font(TextStyle::Monospace));

        match active_code {
            CodeID::Ascii => encode_decode(ui, &control_panel.ascii, input, output, errors),
            CodeID::Base64 => encode_decode(ui, &control_panel.base64, input, output, errors),
            CodeID::MorseITU => encode_decode(ui, &control_panel.morse_itu, input, output, errors),
            CodeID::MorseAmerican => {
                encode_decode(ui, &control_panel.morse_american, input, output, errors)
            }
            CodeID::Godel => encode_decode(ui, &control_panel.godel, input, output, errors),
            CodeID::Fibonacci => encode_decode(ui, &control_panel.fibonacci, input, output, errors),
            CodeID::Unary => encode_decode(ui, &control_panel.unary, input, output, errors),
            CodeID::SpellingAlphabet => {
                encode_decode(ui, &control_panel.spelling_alphabet, input, output, errors)
            }
            _ => *errors = String::from("button must be added to DisplayPanel struct"),
        }

        ui.add_space(10.0);
        if ui.button("clear").clicked() {
            input.clear();
            output.clear();
            errors.clear();
        }

        ui.add_space(10.0);
        if ui.button("swap input/output").clicked() {
            std::mem::swap(input, output)
        }

        if !errors.is_empty() {
            ui.add_space(24.0);
            ui.label(
                RichText::new(errors.clone())
                    .color(Color32::RED)
                    .background_color(Color32::BLACK)
                    .monospace(),
            );
        }
    }
}
