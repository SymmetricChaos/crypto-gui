use crate::{codes::*, ids::CodeID};
use eframe::egui;
use egui::Ui;
pub mod ascii_controls;
pub mod bacon_contols;
pub mod base64_controls;
pub mod baudot_controls;
pub mod fibonacci_controls;
pub mod generic_components;
pub mod godel_controls;
pub mod morse_american_controls;
pub mod morse_controls;
pub mod pgp_controls;
pub mod punycode_controls;
pub mod spelling_alphabet_controls;
pub mod unary_controls;
pub mod unicode_controls;

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
    // Text Standards
    ascii: Ascii,
    bacon: Bacon,
    unicode: Unicode,
    punycode: Punycode,
    spelling: SpellingAlphabet,
    morse_itu: MorseITU,
    morse_american: MorseAmerican,
    baudot: Baudot,

    // Data Encodings
    base64: Base64,
    pgp: PgpWords,
    fibonacci: FibonacciCode,
    unary: UnaryCode,
    godel: Godel,
}

impl CodeInterface {
    pub fn combo_boxes(&mut self, ui: &mut Ui, active_code: &mut CodeID) {
        combox_box(
            &[
                CodeID::Ascii,
                CodeID::Bacon,
                CodeID::Unicode,
                CodeID::MorseITU,
                CodeID::MorseAmerican,
                CodeID::SpellingAlphabet,
                CodeID::Baudot,
            ],
            "Text Standards",
            active_code,
            ui,
        );

        combox_box(
            &[CodeID::Godel, CodeID::Unary, CodeID::Base64],
            "Other Codes",
            active_code,
            ui,
        );
    }

    pub fn get_active_code(&mut self, active_code: &CodeID) -> &mut dyn ViewableCode {
        match active_code {
            CodeID::Ascii => &mut self.ascii,
            CodeID::MorseAmerican => &mut self.morse_american,
            CodeID::MorseITU => &mut self.morse_itu,
            CodeID::Godel => &mut self.godel,
            CodeID::Fibonacci => &mut self.fibonacci,
            CodeID::Baudot => &mut self.baudot,
            CodeID::Base64 => &mut self.base64,
            CodeID::Pgp => &mut self.pgp,
            CodeID::Unary => &mut self.unary,
            CodeID::SpellingAlphabet => &mut self.spelling,
            CodeID::Bacon => &mut self.bacon,
            CodeID::Unicode => &mut self.unicode,
            CodeID::Punycode => &mut self.punycode,
            //_ => todo!("unable to get active code"),
        }
    }
}
