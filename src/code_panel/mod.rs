use crate::{
    codes::{morse::Morse, romaji::romaji::Romaji, *},
    ids::CodeID,
};
use eframe::egui;
use egui::Ui;
pub mod ascii_controls;
pub mod bacon_contols;
pub mod base32_controls;
pub mod base64_controls;
pub mod baudot_controls;
pub mod block_controls;
pub mod fibonacci_controls;
pub mod generic_components;
pub mod godel_controls;
pub mod morse_controls;
pub mod needle_controls;
pub mod pgp_controls;
pub mod punycode_controls;
pub mod romaji_controls;
pub mod spelling_alphabet_controls;
pub mod tap_code_controls;
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
    unicode: Unicode,
    punycode: Punycode,
    spelling: SpellingAlphabet,
    morse: Morse,
    romaji: Romaji,
    baudot: Baudot,
    needle: Needle,

    // Mathematical
    godel: Godel,
    unary: UnaryCode,
    fibonacci: FibonacciCode,

    // Other Codes
    base32: Base32,
    base64: Base64,
    pgp: PgpWords,
    bacon: Bacon,
    tap: TapCode,
    block: BlockCode,
}

impl CodeInterface {
    pub fn combo_boxes(&mut self, ui: &mut Ui, active_code: &mut CodeID) {
        combox_box(
            &[
                CodeID::Ascii,
                CodeID::Baudot,
                CodeID::Morse,
                CodeID::Needle,
                CodeID::Punycode,
                CodeID::SpellingAlphabet,
                CodeID::Unicode,
            ],
            "Text Standards",
            active_code,
            ui,
        );
        combox_box(
            &[CodeID::Godel, CodeID::Fibonacci, CodeID::Unary],
            "Mathematical",
            active_code,
            ui,
        );
        combox_box(
            &[
                CodeID::Bacon,
                CodeID::Base32,
                CodeID::Base64,
                CodeID::Block,
                CodeID::Pgp,
                CodeID::Tap,
            ],
            "Other Codes",
            active_code,
            ui,
        );
    }

    pub fn get_active_code(&mut self, active_code: &CodeID) -> &mut dyn ViewableCode {
        match active_code {
            CodeID::Ascii => &mut self.ascii,
            CodeID::Morse => &mut self.morse,
            CodeID::Godel => &mut self.godel,
            CodeID::Fibonacci => &mut self.fibonacci,
            CodeID::Baudot => &mut self.baudot,
            CodeID::Base32 => &mut self.base32,
            CodeID::Base64 => &mut self.base64,
            CodeID::Pgp => &mut self.pgp,
            CodeID::Unary => &mut self.unary,
            CodeID::SpellingAlphabet => &mut self.spelling,
            CodeID::Bacon => &mut self.bacon,
            CodeID::Unicode => &mut self.unicode,
            CodeID::Punycode => &mut self.punycode,
            CodeID::Block => &mut self.block,
            CodeID::Tap => &mut self.tap,
            CodeID::Needle => &mut self.needle,
            CodeID::Romaji => &mut self.romaji,
            //_ => todo!("unable to get active code"),
        }
    }
}
