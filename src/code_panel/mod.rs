use crate::ids::CodeId;
use codes::{errors::CodeError, traits::Code};
use eframe::egui;
use egui::Ui;

use self::{ascii85_controls::Ascii85Frame, ascii_controls::AsciiFrame, isbn_contols::IsbnFrame};
pub mod generic_components;

mod ascii85_controls;
mod ascii_controls;
mod isbn_contols;

pub trait CodeFrame {
    fn ui(&mut self, ui: &mut Ui, errors: &mut String);
    fn code(&self) -> &dyn Code;
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        self.code().encode(text)
    }
    fn decode(&self, text: &str) -> Result<String, CodeError> {
        self.code().decode(text)
    }
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
    ascii: AsciiFrame,
    baudot: Baudot,
    linotype: Linotype,
    morse: Morse,
    needle: Needle,
    punycode: Punycode,
    romaji: Romaji,
    spelling: SpellingAlphabet,
    unicode: Unicode,

    // Binary to Text
    ascii85: Ascii85Frame,
    base32: Base32,
    base64: Base64,
    numeric: BytesAsNumbers,
    pgp: PgpWords,
    skey: SKeyWords,

    // Error Correcting and Detecting
    isbn: IsbnFrame,
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

    pub fn get_active_code(&mut self, active_code: &CodeId) -> &mut dyn CodeFrame {
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
