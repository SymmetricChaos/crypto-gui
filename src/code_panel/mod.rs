use crate::{
    codes::{morse::Morse, romaji::romaji::Romaji, *},
    ids::CodeId,
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
pub mod skey_controls;
pub mod spelling_alphabet_controls;
pub mod tap_code_controls;
pub mod unary_controls;
pub mod unicode_controls;

pub trait ViewableCode: View + Code {}

pub trait View {
    fn ui(&mut self, ui: &mut Ui, errors: &mut String);
}

// Quick simple combo box builder
fn combox_box(code: &[CodeId], identifier: &'static str, active_code: &mut CodeId, ui: &mut Ui) {
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

    // Binary to Text
    base32: Base32,
    base64: Base64,
    pgp: PgpWords,
    skey: SKeyWords,

    // Mathematical
    godel: Godel,
    unary: UnaryCode,
    fibonacci: FibonacciCode,

    // Other Codes
    bacon: Bacon,
    tap: TapCode,
    block: BlockCode,
}

impl CodeInterface {
    pub fn combo_boxes(&mut self, ui: &mut Ui, active_code: &mut CodeId) {
        combox_box(
            &[
                CodeId::Ascii,
                CodeId::Baudot,
                CodeId::Morse,
                CodeId::Needle,
                CodeId::Punycode,
                CodeId::Romaji,
                CodeId::SpellingAlphabet,
                CodeId::Unicode,
            ],
            "Text Standards",
            active_code,
            ui,
        );
        combox_box(
            &[CodeId::Base32, CodeId::Base64, CodeId::Pgp, CodeId::Skey],
            "Binary-to-Text",
            active_code,
            ui,
        );
        combox_box(
            &[CodeId::Godel, CodeId::Fibonacci, CodeId::Unary],
            "Mathematical",
            active_code,
            ui,
        );
        combox_box(
            &[CodeId::Bacon, CodeId::Block, CodeId::Tap],
            "Other Codes",
            active_code,
            ui,
        );
    }

    pub fn get_active_code(&mut self, active_code: &CodeId) -> &mut dyn ViewableCode {
        match active_code {
            CodeId::Ascii => &mut self.ascii,
            CodeId::Morse => &mut self.morse,
            CodeId::Godel => &mut self.godel,
            CodeId::Fibonacci => &mut self.fibonacci,
            CodeId::Baudot => &mut self.baudot,
            CodeId::Base32 => &mut self.base32,
            CodeId::Base64 => &mut self.base64,
            CodeId::Pgp => &mut self.pgp,
            CodeId::Unary => &mut self.unary,
            CodeId::Skey => &mut self.skey,
            CodeId::SpellingAlphabet => &mut self.spelling,
            CodeId::Bacon => &mut self.bacon,
            CodeId::Unicode => &mut self.unicode,
            CodeId::Punycode => &mut self.punycode,
            CodeId::Block => &mut self.block,
            CodeId::Tap => &mut self.tap,
            CodeId::Needle => &mut self.needle,
            CodeId::Romaji => &mut self.romaji,
            //_ => todo!("unable to get active code"),
        }
    }
}
