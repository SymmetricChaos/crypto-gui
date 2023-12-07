use std::fmt::Display;

use json::JsonValue;
use lazy_static::lazy_static;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum CodeId {
    Ascii,
    Ascii85,
    Bacon,
    BalancedTernary,
    Barbier,
    BaseN,
    BaseX,
    Base16,
    Base32,
    Base64,
    Baudot,
    BiquinaryDecimal,
    Braille,
    BrailleEncoding,
    ByteAsNum,
    Ccsid,
    CcsidBinary,
    CyclicRedundancyCheck,
    Damm,
    Elias,
    Factoradic,
    Fibonacci,
    FixedWidth,
    Godel,
    Gray,
    Hamming,
    Isbn,
    Itf,
    Levenshtein,
    Linotype,
    Luhn,
    MofN,
    Morse,
    Needle,
    ParityBit,
    Pgp,
    Punycode,
    Repetition,
    Romaji,
    RomanNumeral,
    Skey,
    SpellingAlphabet,
    Tap,
    TwosComplement,
    Ueb,
    Unary,
    UnarySymmetric,
    Unicode,
    Upc,
    Verhoeff,
    Wabun,
}

impl Default for CodeId {
    fn default() -> Self {
        Self::Ascii
    }
}

impl CodeId {
    pub fn description(&self) -> &'static str {
        match CODE_INFORMATION[self.to_string()].as_str() {
            Some(s) => s,
            None => "<<<MISSING DESCRIPTION>>>",
        }
    }
}

impl Display for CodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            CodeId::Ascii => "ASCII",
            CodeId::Ascii85 => "Ascii85",
            CodeId::Bacon => "Bacon",
            CodeId::BalancedTernary => "Balanced Ternary",
            CodeId::Barbier => "Barbier",
            CodeId::BaseN => "Base-N",
            CodeId::BaseX => "BaseX",
            CodeId::Base16 => "Base16/Hexcode",
            CodeId::Base32 => "Base32",
            CodeId::Base64 => "Base64",
            CodeId::Baudot => "Baudot",
            CodeId::BiquinaryDecimal => "Biquinary Coded Decimal",
            CodeId::Braille => "Simplified Braille",
            CodeId::BrailleEncoding => "Braille Encodings",
            CodeId::ByteAsNum => "Bytes as Numbers",
            CodeId::Ccsid => "CCSID",
            CodeId::CcsidBinary => "CCSID",
            CodeId::CyclicRedundancyCheck => "Cyclic Redundancy Check",
            CodeId::Damm => "Damm",
            CodeId::Elias => "Elias",
            CodeId::Factoradic => "Factoradic",
            CodeId::Fibonacci => "Fibonacci",
            CodeId::FixedWidth => "Fixed-Width",
            CodeId::Godel => "Gödel",
            CodeId::Gray => "Gray Code",
            CodeId::Hamming => "Hamming Code",
            CodeId::Isbn => "ISBN",
            CodeId::Itf => "ITF",
            CodeId::Levenshtein => "Levenshtein",
            CodeId::Linotype => "Linotype",
            CodeId::Luhn => "Luhn's Algorithm",
            CodeId::MofN => "M-of-N",
            CodeId::Morse => "Morse",
            CodeId::Needle => "Needle",
            CodeId::ParityBit => "Parity Bit",
            CodeId::Pgp => "PGP Words",
            CodeId::Punycode => "Punycode",
            CodeId::Repetition => "Repetition",
            CodeId::Romaji => "Romaji",
            CodeId::RomanNumeral => "Roman Numeral",
            CodeId::Skey => "S/KEY",
            CodeId::SpellingAlphabet => "Spelling Alphabet",
            CodeId::Tap => "Tap",
            CodeId::TwosComplement => "Two's Complement",
            CodeId::Ueb => "Unified English Braille",
            CodeId::Unary => "Unary",
            CodeId::UnarySymmetric => "Symmetric Unary",
            CodeId::Unicode => "Unicode",
            CodeId::Upc => "UPC",
            CodeId::Verhoeff => "Verhoeff",
            CodeId::Wabun => "Wabun",
        };
        write!(f, "{}", name)
    }
}

impl From<CodeId> for String {
    fn from(id: CodeId) -> Self {
        id.to_string()
    }
}

const JSON_CODE_INFORMATION: &'static str = include_str!("code_descriptions.json");

lazy_static! {
    pub static ref CODE_INFORMATION: JsonValue = {
        json::parse(&JSON_CODE_INFORMATION.replace('\u{feff}', ""))
            .expect("unable to parse code_descriptions")
    };
}
