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
    ascii: Ascii,
    bacon: Bacon,
    unicode: Unicode,
    punycode: Punycode,

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
        combox_box(&[CodeID::Ascii, CodeID::Bacon], "Binary", active_code, ui);

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
            CodeID::Bacon => &mut self.bacon,
            CodeID::Unicode => &mut self.unicode,
            CodeID::Punycode => &mut self.punycode,
            //_ => todo!("unable to get active code"),
        }
    }
}
