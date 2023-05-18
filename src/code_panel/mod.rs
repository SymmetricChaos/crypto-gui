use crate::{
    codes::{
        binary_to_text::{ascii85::Ascii85, numeric::BytesAsNumbers},
        morse::Morse,
        romaji::romaji::Romaji,
        *,
    },
    ids::CodeId,
};
use eframe::egui;
use egui::Ui;
pub mod ascii85_controls;
pub mod ascii_controls;
pub mod bacon_contols;
pub mod base32_controls;
pub mod base64_controls;
pub mod baudot_controls;
pub mod block_controls;
pub mod fibonacci_controls;
pub mod generic_components;
pub mod godel_controls;
pub mod isbn_contols;
pub mod levenshtein_controls;
pub mod linotype_controls;
pub mod luhn_controls;
pub mod m_of_n_controls;
pub mod morse_controls;
pub mod needle_controls;
pub mod numeric_controls;
pub mod parity_check_controls;
pub mod pgp_controls;
pub mod punycode_controls;
pub mod repetition_controls;
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
    baudot: Baudot,
    linotype: Linotype,
    morse: Morse,
    needle: Needle,
    punycode: Punycode,
    romaji: Romaji,
    spelling: SpellingAlphabet,
    unicode: Unicode,

    // Binary to Text
    ascii85: Ascii85,
    base32: Base32,
    base64: Base64,
    numeric: BytesAsNumbers,
    pgp: PgpWords,
    skey: SKeyWords,

    // Error Correcting and Detecting
    isbn: Isbn,
    luhn: LuhnAlgorithm,
    m_of_n: MofNCode,
    parity_bit: ParityBit,
    repetition: Repetition,

    // Mathematical
    fibonacci: FibonacciCode,
    godel: Godel,
    levenshtein: LevenshteinCode,
    unary: UnaryCode,

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
                CodeId::Linotype,
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
            &[
                CodeId::Ascii85,
                CodeId::Base32,
                CodeId::Base64,
                CodeId::ByteAsNum,
                CodeId::Pgp,
                CodeId::Skey,
            ],
            "Binary-to-Text",
            active_code,
            ui,
        );
        combox_box(
            &[
                CodeId::Fibonacci,
                CodeId::Godel,
                CodeId::Levenshtein,
                CodeId::Unary,
            ],
            "Mathematical",
            active_code,
            ui,
        );
        combox_box(
            &[
                CodeId::Isbn,
                CodeId::Luhn,
                CodeId::MofN,
                CodeId::ParityBit,
                CodeId::Repetition,
            ],
            "Error Correcting Codes",
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
            CodeId::Ascii85 => &mut self.ascii85,
            CodeId::Bacon => &mut self.bacon,
            CodeId::Base32 => &mut self.base32,
            CodeId::Base64 => &mut self.base64,
            CodeId::Baudot => &mut self.baudot,
            CodeId::Block => &mut self.block,
            CodeId::ByteAsNum => &mut self.numeric,
            CodeId::Fibonacci => &mut self.fibonacci,
            CodeId::Godel => &mut self.godel,
            CodeId::Hamming => todo!("ADD HAMMING"),
            CodeId::Isbn => &mut self.isbn,
            CodeId::Levenshtein => &mut self.levenshtein,
            CodeId::Linotype => &mut self.linotype,
            CodeId::Luhn => &mut self.luhn,
            CodeId::MofN => &mut self.m_of_n,
            CodeId::Morse => &mut self.morse,
            CodeId::Needle => &mut self.needle,
            CodeId::ParityBit => &mut self.parity_bit,
            CodeId::Pgp => &mut self.pgp,
            CodeId::Punycode => &mut self.punycode,
            CodeId::Repetition => &mut self.repetition,
            CodeId::Romaji => &mut self.romaji,
            CodeId::Skey => &mut self.skey,
            CodeId::SpellingAlphabet => &mut self.spelling,
            CodeId::Tap => &mut self.tap,
            CodeId::Unary => &mut self.unary,
            CodeId::Unicode => &mut self.unicode,
        }
    }
}
