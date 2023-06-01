use std::fmt::Display;

use json::JsonValue;
use lazy_static::lazy_static;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum CodeId {
    Ascii,
    Ascii85,
    Bacon,
    Base32,
    Base64,
    Baudot,
    Block,
    ByteAsNum,
    Fibonacci,
    Godel,
    Hamming,
    Isbn,
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
    Skey,
    SpellingAlphabet,
    Tap,
    Unary,
    Unicode,
    Verhoeff,
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
            CodeId::Base32 => "Base32",
            CodeId::Base64 => "Base64",
            CodeId::Baudot => "Baudot",
            CodeId::Block => "Block",
            CodeId::ByteAsNum => "Bytes as Numbers",
            CodeId::Fibonacci => "Fibonacci",
            CodeId::Godel => "Gödel",
            CodeId::Hamming => "Hamming Code",
            CodeId::Isbn => "ISBN",
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
            CodeId::Skey => "S/KEY",
            CodeId::SpellingAlphabet => "Spelling Alphabet",
            CodeId::Tap => "Tap",
            CodeId::Unary => "Unary",
            CodeId::Unicode => "Unicode",
            CodeId::Verhoeff => "Verhoeff",
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
