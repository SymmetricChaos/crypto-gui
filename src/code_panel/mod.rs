use codes::{
    errors::CodeError,
    ids::{CodeCategory, CodeId},
    traits::Code,
};

use eframe::egui;
use egui::Ui;

mod ascii85_controls;
mod ascii_controls;
mod bacon_contols;
mod balanced_ternary_controls;
mod barbier_controls;
mod base16_controls;
mod base32_controls;
mod base64_controls;
mod base_n_bijective_controls;
mod base_n_controls;
mod basex_controls;
mod baudot_controls;
mod biquinary_controls;
mod block_controls;
mod braille_encoding_controls;
mod bytewords_controls;
mod ccsid_controls;
mod crc_controls;
mod damm_controls;
mod elias_controls;
mod factoradic_controls;
mod fibonacci_controls;
mod godel_controls;
mod gray_controls;
mod hamming_controls;
mod ics_flags_controls;
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
mod semaphore_controls;
mod simple_braille_controls;
mod skey_controls;
mod spelling_alphabet_controls;
mod symmetric_unary_controls;
mod tap_code_controls;
mod twos_complement_controls;
mod ueb_controls;
mod unary_controls;
mod unicode_controls;
mod upc_controls;
mod verhoeff_controls;
mod wabun_controls;

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
        egui::ComboBox::from_id_salt(code_category.to_string())
            .selected_text(code_category.to_string())
            .show_ui(ui, |ui| {
                for id in code {
                    ui.selectable_value(active_code, Some(*id), id.to_string());
                }
            });
        ui.menu_button("+", |ui| {
            ui.label(code_category.description());
        })
    });

    ui.add_space(10.0);
}

#[derive(Default)]
pub struct CodeInterface {
    // Text Standards
    ascii: ascii_controls::AsciiFrame,
    baudot: baudot_controls::BaudotFrame,
    braille: simple_braille_controls::BrailleFrame,
    braille_encoding: braille_encoding_controls::BrailleEncodingFrame,
    ccsid: ccsid_controls::CcsidFrame,
    flag_semaphore: semaphore_controls::SemaphoreFrame,
    ics_flags: ics_flags_controls::IcsFlagsFrame,
    linotype: linotype_controls::LinotypeFrame,
    morse: morse_controls::MorseFrame,
    needle: needle_controls::NeedleFrame,
    punycode: punycode_controls::PunycodeFrame,
    romaji: romaji_controls::RomajiFrame,
    spelling: spelling_alphabet_controls::SpellingAlphabetFrame,
    ueb: ueb_controls::UebFrame,
    unicode: unicode_controls::UnicodeFrame,
    wabun: wabun_controls::WabunFrame,

    // Binary to Text
    ascii85: ascii85_controls::Ascii85Frame,
    base16: base16_controls::Base16Frame,
    base32: base32_controls::Base32Frame,
    base64: base64_controls::Base64Frame,
    basex: basex_controls::BaseXFrame,
    bytewords: bytewords_controls::BytewordsFrame,
    ccsid_binary: ccsid_controls::CcsidBinaryFrame,
    numeric: numeric_controls::BytesAsNumbersFrame,
    pgp: pgp_controls::PgpWordsFrame,
    skey: skey_controls::SKeyWordsFrame,

    // Error Correcting and Detecting
    crc: crc_controls::CyclicRedundancyCheckFrame,
    damm: damm_controls::DammFrame,
    hamming: hamming_controls::HammingFrame,
    luhn: luhn_controls::LuhnAlgorithmFrame,
    m_of_n: m_of_n_controls::MofNCodeFrame,
    parity_bit: parity_check_controls::ParityBitFrame,
    repetition: repetition_controls::RepetitionFrame,
    verhoeff: verhoeff_controls::VerhoeffFrame,

    // Commercial
    isbn: isbn_contols::IsbnFrame,
    itf: itf_controls::ItfFrame,
    upc: upc_controls::UpcFrame,

    // Integer
    base_n: base_n_controls::BaseNFrame,
    base_n_bij: base_n_bijective_controls::BaseNBijectiveFrame,
    balanced_ternary: balanced_ternary_controls::BalancedTernaryFrame,
    godel: godel_controls::GodelFrame,
    gray: gray_controls::GrayCodeFrame,
    roman: roman_numeral_controls::RomanNumeralFrame,
    twos_complement: twos_complement_controls::TwosComplementFrame,
    factoradic: factoradic_controls::FactoradicFrame,
    biquinary: biquinary_controls::BiquinaryDecimalFrame,

    // Prefix
    fixed_width: block_controls::BlockCodeFrame,
    elias: elias_controls::EliasCodeFrame,
    fibonacci: fibonacci_controls::FibonacciCodeFrame,
    levenshtein: levenshtein_controls::LevenshteinCodeFrame,
    unary: unary_controls::UnaryCodeFrame,
    unary_symmetric: symmetric_unary_controls::SymUnaryCodeFrame,

    // Other Codes
    bacon: bacon_contols::BaconFrame,
    barbier: barbier_controls::BarbierFrame,
    tap: tap_code_controls::TapCodeFrame,
}

impl CodeInterface {
    pub fn combo_boxes(&mut self, ui: &mut Ui, active_code: &mut Option<CodeId>) {
        combox_box(
            &[
                CodeId::Ascii,
                CodeId::Baudot,
                CodeId::Braille,
                CodeId::BrailleEncoding,
                CodeId::Ccsid,
                CodeId::FlagSemaphore,
                CodeId::IcsFlags,
                CodeId::Linotype,
                CodeId::Morse,
                CodeId::Needle,
                CodeId::Punycode,
                CodeId::Romaji,
                CodeId::SpellingAlphabet,
                CodeId::Ueb,
                CodeId::Unicode,
                CodeId::Wabun,
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
                CodeId::Bytewords,
                CodeId::CcsidBinary,
                CodeId::Pgp,
                CodeId::Skey,
            ],
            active_code,
            CodeCategory::BinaryToText,
            ui,
        );
        combox_box(
            &[
                CodeId::BalancedTernary,
                CodeId::BaseN,
                CodeId::BaseNBijective,
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
            &[CodeId::Bacon, CodeId::Barbier, CodeId::Tap],
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
            CodeId::Barbier => &mut self.barbier,
            CodeId::BalancedTernary => &mut self.balanced_ternary,
            CodeId::BaseN => &mut self.base_n,
            CodeId::BaseNBijective => &mut self.base_n_bij,
            CodeId::BaseX => &mut self.basex,
            CodeId::Base16 => &mut self.base16,
            CodeId::Base32 => &mut self.base32,
            CodeId::Base64 => &mut self.base64,
            CodeId::Baudot => &mut self.baudot,
            CodeId::BiquinaryDecimal => &mut self.biquinary,
            CodeId::Braille => &mut self.braille,
            CodeId::BrailleEncoding => &mut self.braille_encoding,
            CodeId::ByteAsNum => &mut self.numeric,
            CodeId::Bytewords => &mut self.bytewords,
            CodeId::Ccsid => &mut self.ccsid,
            CodeId::CcsidBinary => &mut self.ccsid_binary,
            CodeId::CyclicRedundancyCheck => &mut self.crc,
            CodeId::Damm => &mut self.damm,
            CodeId::Elias => &mut self.elias,
            CodeId::Factoradic => &mut self.factoradic,
            CodeId::Fibonacci => &mut self.fibonacci,
            CodeId::FixedWidth => &mut self.fixed_width,
            CodeId::FlagSemaphore => &mut self.flag_semaphore,
            CodeId::Godel => &mut self.godel,
            CodeId::Gray => &mut self.gray,
            CodeId::Hamming => &mut self.hamming,
            CodeId::IcsFlags => &mut self.ics_flags,
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
            CodeId::Ueb => &mut self.ueb,
            CodeId::Unary => &mut self.unary,
            CodeId::UnarySymmetric => &mut self.unary_symmetric,
            CodeId::Unicode => &mut self.unicode,
            CodeId::Upc => &mut self.upc,
            CodeId::Verhoeff => &mut self.verhoeff,
            CodeId::Wabun => &mut self.wabun,
            // _ => panic!("unknown code selected"),
        }
    }
}
