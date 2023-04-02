use std::fmt::Display;

use json::JsonValue;
use lazy_static::lazy_static;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum CodeID {
    Ascii,
    Morse,
    Godel,
    Fibonacci,
    Baudot,
    Base64,
    Pgp,
    Unary,
    SpellingAlphabet,
    Bacon,
    Unicode,
    Punycode,
    Block,
    Tap,
}

impl Default for CodeID {
    fn default() -> Self {
        Self::Ascii
    }
}

impl CodeID {
    // Describe the history of the code
    pub fn description(&self) -> &'static str {
        match CODE_INFORMATION[self.to_string()].as_str() {
            Some(s) => s,
            None => "<<<MISSING DESCRIPTION>>>",
        }
    }
}

impl Display for CodeID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            CodeID::Ascii => "ASCII",
            CodeID::Morse => "Morse",
            CodeID::Godel => "Gödel",
            CodeID::Fibonacci => "Fibonacci",
            CodeID::Baudot => "Baudot",
            CodeID::Base64 => "Base64",
            CodeID::Unary => "Unary",
            CodeID::SpellingAlphabet => "Spelling Alphabet",
            CodeID::Pgp => "PGP Words",
            CodeID::Bacon => "Bacon",
            CodeID::Unicode => "Unicode",
            CodeID::Punycode => "Punycode",
            CodeID::Block => "Block",
            CodeID::Tap => "Tap",
        };
        write!(f, "{}", name)
    }
}

impl From<CodeID> for String {
    fn from(id: CodeID) -> Self {
        id.to_string()
    }
}

const JSON_CODE_INFORMATION: &'static str = include_str!("code_descriptions.json");

lazy_static! {
    pub static ref CODE_INFORMATION: JsonValue = {
        json::parse(&JSON_CODE_INFORMATION.replace('\u{feff}', ""))
            .expect("unable to parse code_descriptions.json")
    };
}
