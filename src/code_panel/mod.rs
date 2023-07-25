use codes::{errors::CodeError, ids::CodeId, traits::Code};
use eframe::egui;
use egui::Ui;

use self::{
    ascii85_controls::Ascii85Frame, ascii_controls::AsciiFrame, bacon_contols::BaconFrame,
    base32_controls::Base32Frame, base64_controls::Base64Frame, base_n_controls::BaseNFrame,
    baudot_controls::BaudotFrame, block_controls::BlockCodeFrame, damm_controls::DammFrame,
    elias_controls::EliasCodeFrame, fibonacci_controls::FibonacciCodeFrame,
    godel_controls::GodelFrame, hamming_controls::HammingFrame, isbn_contols::IsbnFrame,
    itf_controls::ItfFrame, levenshtein_controls::LevenshteinCodeFrame,
    linotype_controls::LinotypeFrame, luhn_controls::LuhnAlgorithmFrame,
    m_of_n_controls::MofNCodeFrame, morse_controls::MorseFrame, needle_controls::NeedleFrame,
    numeric_controls::BytesAsNumbersFrame, parity_check_controls::ParityBitFrame,
    pgp_controls::PgpWordsFrame, punycode_controls::PunycodeFrame,
    repetition_controls::RepetitionFrame, romaji_controls::RomajiFrame,
    skey_controls::SKeyWordsFrame, spelling_alphabet_controls::SpellingAlphabetFrame,
    tap_code_controls::TapCodeFrame, unary_controls::UnaryCodeFrame,
    unicode_controls::UnicodeFrame, upc_controls::UpcFrame, verhoeff_controls::VerhoeffFrame,
};

mod ascii85_controls;
mod ascii_controls;
mod bacon_contols;
mod base32_controls;
mod base64_controls;
mod base_n_controls;
mod baudot_controls;
mod block_controls;
mod damm_controls;
mod elias_controls;
mod fibonacci_controls;
mod godel_controls;
mod hamming_controls;
mod isbn_contols;
mod itf_controls;
mod levenshtein_controls;
mod linotype_controls;
mod luhn_controls;
mod m_of_n_controls;
mod morse_controls;
mod needle_controls;
mod numeric_controls;
mod parity_check_controls;
mod pgp_controls;
mod punycode_controls;
mod repetition_controls;
mod romaji_controls;
mod skey_controls;
mod spelling_alphabet_controls;
mod tap_code_controls;
mod unary_controls;
mod unicode_controls;
mod upc_controls;
mod verhoeff_controls;

pub trait CodeFrame {
    fn ui(&mut self, ui: &mut Ui);
    fn code(&self) -> &dyn Code;
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        self.code().encode(text)
    }
    fn decode(&self, text: &str) -> Result<String, CodeError> {
        self.code().decode(text)
    }
}

// Quick simple combo box builder
fn combox_box(
    code: &[CodeId],
    identifier: &'static str,
    active_code: &mut Option<CodeId>,
    ui: &mut Ui,
) {
    egui::ComboBox::from_id_source(identifier)
        .selected_text(identifier)
        .show_ui(ui, |ui| {
            for id in code {
                ui.selectable_value(active_code, Some(*id), id.to_string());
            }
        });
    ui.add_space(10.0);
}

#[derive(Default)]
pub struct CodeInterface {
    // Text Standards
    ascii: AsciiFrame,
    baudot: BaudotFrame,
    linotype: LinotypeFrame,
    morse: MorseFrame,
    needle: NeedleFrame,
    punycode: PunycodeFrame,
    romaji: RomajiFrame,
    spelling: SpellingAlphabetFrame,
    unicode: UnicodeFrame,

    // Binary to Text
    ascii85: Ascii85Frame,
    base32: Base32Frame,
    base64: Base64Frame,
    numeric: BytesAsNumbersFrame,
    pgp: PgpWordsFrame,
    skey: SKeyWordsFrame,

    // Error Correcting and Detecting
    damm: DammFrame,
    hamming: HammingFrame,
    luhn: LuhnAlgorithmFrame,
    m_of_n: MofNCodeFrame,
    parity_bit: ParityBitFrame,
    repetition: RepetitionFrame,
    verhoeff: VerhoeffFrame,

    // Commercial
    isbn: IsbnFrame,
    itf: ItfFrame,
    upc: UpcFrame,

    // Mathematical
    basen: BaseNFrame,
    elias: EliasCodeFrame,
    fibonacci: FibonacciCodeFrame,
    godel: GodelFrame,
    levenshtein: LevenshteinCodeFrame,
    unary: UnaryCodeFrame,

    // Other Codes
    bacon: BaconFrame,
    block: BlockCodeFrame,
    tap: TapCodeFrame,
}

impl CodeInterface {
    pub fn combo_boxes(&mut self, ui: &mut Ui, active_code: &mut Option<CodeId>) {
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
                CodeId::BaseN,
                CodeId::Elias,
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
                CodeId::Damm,
                CodeId::Hamming,
                CodeId::Luhn,
                CodeId::MofN,
                CodeId::ParityBit,
                CodeId::Repetition,
                CodeId::Verhoeff,
            ],
            "Error Correcting",
            active_code,
            ui,
        );
        combox_box(
            &[CodeId::Isbn, CodeId::Itf, CodeId::Upc],
            "Commercial",
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
            CodeId::BaseN => &mut self.basen,
            CodeId::Base32 => &mut self.base32,
            CodeId::Base64 => &mut self.base64,
            CodeId::Baudot => &mut self.baudot,
            CodeId::Block => &mut self.block,
            CodeId::ByteAsNum => &mut self.numeric,
            CodeId::Damm => &mut self.damm,
            CodeId::Elias => &mut self.elias,
            CodeId::Fibonacci => &mut self.fibonacci,
            CodeId::Godel => &mut self.godel,
            CodeId::Hamming => &mut self.hamming,
            CodeId::Isbn => &mut self.isbn,
            CodeId::Itf => &mut self.itf,
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
            CodeId::Upc => &mut self.upc,
            CodeId::Verhoeff => &mut self.verhoeff,
            // _ => panic!("unknown code selected"),
        }
    }
}
