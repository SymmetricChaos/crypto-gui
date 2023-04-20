use std::fmt::Display;

use json::JsonValue;
use lazy_static::lazy_static;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum CodeId {
    Ascii,
    Bacon,
    Base32,
    Base64,
    Baudot,
    Block,
    Fibonacci,
    Godel,
    Morse,
    Needle,
    Pgp,
    Punycode,
    Romaji,
    SpellingAlphabet,
    Tap,
    Unary,
    Unicode,
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
            CodeId::Morse => "Morse",
            CodeId::Godel => "Gödel",
            CodeId::Fibonacci => "Fibonacci",
            CodeId::Baudot => "Baudot",
            CodeId::Base32 => "Base32",
            CodeId::Base64 => "Base64",
            CodeId::Unary => "Unary",
            CodeId::SpellingAlphabet => "Spelling Alphabet",
            CodeId::Pgp => "PGP Words",
            CodeId::Bacon => "Bacon",
            CodeId::Unicode => "Unicode",
            CodeId::Punycode => "Punycode",
            CodeId::Romaji => "Romaji",
            CodeId::Block => "Block",
            CodeId::Tap => "Tap",
            CodeId::Needle => "Needle",
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
