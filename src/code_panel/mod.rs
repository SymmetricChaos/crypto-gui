use codes::{
    errors::CodeError,
    ids::{CodeCategory, CodeId},
    traits::Code,
};

use eframe::egui;
use egui::Ui;

use self::{
    ascii85_controls::Ascii85Frame, ascii_controls::AsciiFrame, bacon_contols::BaconFrame,
    balanced_ternary_controls::BalancedTernaryFrame, base16_controls::Base16Frame,
    base32_controls::Base32Frame, base64_controls::Base64Frame, base_n_controls::BaseNFrame,
    basex_controls::BaseXFrame, baudot_controls::BaudotFrame,
    biquinary_controls::BiquinaryDecimalFrame, block_controls::BlockCodeFrame,
    braille_ascii_controls::BrailleAsciiFrame, braille_controls::BrailleFrame,
    crc_controls::CyclicRedundancyCheckFrame, damm_controls::DammFrame,
    elias_controls::EliasCodeFrame, factoradic_controls::FactoradicFrame,
    fibonacci_controls::FibonacciCodeFrame, godel_controls::GodelFrame,
    gray_controls::GrayCodeFrame, hamming_controls::HammingFrame, isbn_contols::IsbnFrame,
    itf_controls::ItfFrame, levenshtein_controls::LevenshteinCodeFrame,
    linotype_controls::LinotypeFrame, luhn_controls::LuhnAlgorithmFrame,
    m_of_n_controls::MofNCodeFrame, morse_controls::MorseFrame, needle_controls::NeedleFrame,
    numeric_controls::BytesAsNumbersFrame, parity_check_controls::ParityBitFrame,
    pgp_controls::PgpWordsFrame, punycode_controls::PunycodeFrame,
    repetition_controls::RepetitionFrame, romaji_controls::RomajiFrame,
    roman_numeral_controls::RomanNumeralFrame, skey_controls::SKeyWordsFrame,
    spelling_alphabet_controls::SpellingAlphabetFrame, symmetric_unary_controls::SymUnaryCodeFrame,
    tap_code_controls::TapCodeFrame, twos_complement_controls::TwosComplementFrame,
    unary_controls::UnaryCodeFrame, unicode_controls::UnicodeFrame, upc_controls::UpcFrame,
    verhoeff_controls::VerhoeffFrame,
};

mod ascii85_controls;
mod ascii_controls;
mod bacon_contols;
mod balanced_ternary_controls;
mod base16_controls;
mod base32_controls;
mod base64_controls;
mod base_n_controls;
mod basex_controls;
mod baudot_controls;
mod biquinary_controls;
mod block_controls;
mod braille_ascii_controls;
mod braille_controls;
mod crc_controls;
mod damm_controls;
mod elias_controls;
mod factoradic_controls;
mod fibonacci_controls;
mod godel_controls;
mod gray_controls;
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
mod roman_numeral_controls;
mod skey_controls;
mod spelling_alphabet_controls;
mod symmetric_unary_controls;
mod tap_code_controls;
mod twos_complement_controls;
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
    active_code: &mut Option<CodeId>,
    code_category: CodeCategory,
    ui: &mut Ui,
) {
    ui.horizontal(|ui| {
        egui::ComboBox::from_id_source(code_category.to_string())
            .selected_text(code_category.to_string())
            .show_ui(ui, |ui| {
                for id in code {
                    ui.selectable_value(active_code, Some(*id), id.to_string());
                }
            });
        ui.label("+").on_hover_text(code_category.description());
    });

    ui.add_space(10.0);
}

#[derive(Default)]
pub struct CodeInterface {
    // Text Standards
    ascii: AsciiFrame,
    baudot: BaudotFrame,
    braille: BrailleFrame,
    braille_ascii: BrailleAsciiFrame,
    linotype: LinotypeFrame,
    morse: MorseFrame,
    needle: NeedleFrame,
    punycode: PunycodeFrame,
    romaji: RomajiFrame,
    spelling: SpellingAlphabetFrame,
    unicode: UnicodeFrame,

    // Binary to Text
    ascii85: Ascii85Frame,
    base16: Base16Frame,
    base32: Base32Frame,
    base64: Base64Frame,
    basex: BaseXFrame,
    numeric: BytesAsNumbersFrame,
    pgp: PgpWordsFrame,
    skey: SKeyWordsFrame,

    // Error Correcting and Detecting
    crc: CyclicRedundancyCheckFrame,
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

    // Integer
    basen: BaseNFrame,
    balanced_ternary: BalancedTernaryFrame,
    godel: GodelFrame,
    gray: GrayCodeFrame,
    roman: RomanNumeralFrame,
    twos_complement: TwosComplementFrame,
    factoradic: FactoradicFrame,
    biquinary: BiquinaryDecimalFrame,

    // Prefix
    fixed_width: BlockCodeFrame,
    elias: EliasCodeFrame,
    fibonacci: FibonacciCodeFrame,
    levenshtein: LevenshteinCodeFrame,
    unary: UnaryCodeFrame,
    unary_symmetric: SymUnaryCodeFrame,

    // Other Codes
    bacon: BaconFrame,
    tap: TapCodeFrame,
}

impl CodeInterface {
    pub fn combo_boxes(&mut self, ui: &mut Ui, active_code: &mut Option<CodeId>) {
        combox_box(
            &[
                CodeId::Ascii,
                CodeId::Baudot,
                CodeId::Braille,
                CodeId::BrailleAscii,
                CodeId::Linotype,
                CodeId::Morse,
                CodeId::Needle,
                CodeId::Punycode,
                CodeId::Romaji,
                CodeId::SpellingAlphabet,
                CodeId::Unicode,
            ],
            active_code,
            CodeCategory::TextStandard,
            ui,
        );
        combox_box(
            &[
                CodeId::Ascii85,
                CodeId::BaseX,
                CodeId::Base32,
                CodeId::Base64,
                CodeId::ByteAsNum,
                CodeId::Pgp,
                CodeId::Skey,
            ],
            active_code,
            CodeCategory::BinaryToText,
            ui,
        );
        combox_box(
            &[
                CodeId::BaseN,
                CodeId::BalancedTernary,
                CodeId::BiquinaryDecimal,
                CodeId::Elias,
                CodeId::Factoradic,
                CodeId::Fibonacci,
                CodeId::Godel,
                CodeId::Gray,
                CodeId::Levenshtein,
                CodeId::RomanNumeral,
                CodeId::TwosComplement,
                CodeId::Unary,
                CodeId::UnarySymmetric,
            ],
            active_code,
            CodeCategory::Integer,
            ui,
        );
        combox_box(
            &[
                CodeId::Ascii,
                CodeId::FixedWidth,
                CodeId::Elias,
                CodeId::Fibonacci,
                CodeId::Levenshtein,
                CodeId::MofN,
                CodeId::Unary,
                CodeId::UnarySymmetric,
                CodeId::Unicode,
            ],
            active_code,
            CodeCategory::Prefix,
            ui,
        );
        combox_box(
            &[
                CodeId::CyclicRedundancyCheck,
                CodeId::Damm,
                CodeId::Hamming,
                CodeId::Luhn,
                CodeId::MofN,
                CodeId::ParityBit,
                CodeId::Repetition,
                CodeId::Verhoeff,
            ],
            active_code,
            CodeCategory::ErrorCorrecting,
            ui,
        );
        combox_box(
            &[CodeId::Isbn, CodeId::Itf, CodeId::Upc],
            active_code,
            CodeCategory::Commercial,
            ui,
        );
        combox_box(
            &[CodeId::Bacon, CodeId::Tap],
            active_code,
            CodeCategory::Other,
            ui,
        );
    }

    pub fn get_active_code(&mut self, active_code: &CodeId) -> &mut dyn CodeFrame {
        match active_code {
            CodeId::Ascii => &mut self.ascii,
            CodeId::Ascii85 => &mut self.ascii85,
            CodeId::Bacon => &mut self.bacon,
            CodeId::BalancedTernary => &mut self.balanced_ternary,
            CodeId::BaseN => &mut self.basen,
            CodeId::BaseX => &mut self.basex,
            CodeId::Base16 => &mut self.base16,
            CodeId::Base32 => &mut self.base32,
            CodeId::Base64 => &mut self.base64,
            CodeId::Baudot => &mut self.baudot,
            CodeId::BiquinaryDecimal => &mut self.biquinary,
            CodeId::Braille => &mut self.braille,
            CodeId::BrailleAscii => &mut self.braille_ascii,
            CodeId::ByteAsNum => &mut self.numeric,
            CodeId::CyclicRedundancyCheck => &mut self.crc,
            CodeId::Damm => &mut self.damm,
            CodeId::Elias => &mut self.elias,
            CodeId::Factoradic => &mut self.factoradic,
            CodeId::Fibonacci => &mut self.fibonacci,
            CodeId::FixedWidth => &mut self.fixed_width,
            CodeId::Godel => &mut self.godel,
            CodeId::Gray => &mut self.gray,
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
            CodeId::RomanNumeral => &mut self.roman,
            CodeId::Skey => &mut self.skey,
            CodeId::SpellingAlphabet => &mut self.spelling,
            CodeId::Tap => &mut self.tap,
            CodeId::TwosComplement => &mut self.twos_complement,
            CodeId::Unary => &mut self.unary,
            CodeId::UnarySymmetric => &mut self.unary_symmetric,
            CodeId::Unicode => &mut self.unicode,
            CodeId::Upc => &mut self.upc,
            CodeId::Verhoeff => &mut self.verhoeff,
            // _ => panic!("unknown code selected"),
        }
    }
}
