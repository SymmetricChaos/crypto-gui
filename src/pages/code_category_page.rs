use crate::ids::CodeId;

#[derive(Debug, PartialEq, Eq)]
pub enum CodeCategory {
    Morse,    // MorseAmerican, Morse ITU
    Binary,   // Baudot, Ascii, Bacon, Fibonacci, Base64
    Unary,    // Unary
    Spelling, // Pgp, Phonetic
    Godel,
}

impl Default for CodeCategory {
    fn default() -> Self {
        Self::Morse
    }
}

impl CodeCategory {
    pub fn description_of_category(&self) -> &'static str {
        match self {
            CodeCategory::Morse => "<<<DESCRIPTION NEEDED>>",
            CodeCategory::Binary => "<<<DESCRIPTION NEEDED>>",
            CodeCategory::Unary => "<<<DESCRIPTION NEEDED>>",
            CodeCategory::Spelling => "<<<DESCRIPTION NEEDED>>",
            CodeCategory::Godel => "<<<DESCRIPTION NEEDED>>",
        }
    }

    pub fn all_cipher_in_category(&self) -> &[CodeId] {
        match self {
            CodeCategory::Morse => &[CodeId::Morse],
            CodeCategory::Binary => &[
                CodeId::Ascii,
                CodeId::Bacon,
                CodeId::Baudot,
                CodeId::Base64,
                CodeId::Fibonacci,
            ],
            CodeCategory::Unary => &[CodeId::Unary],
            CodeCategory::Spelling => &[CodeId::Pgp, CodeId::SpellingAlphabet],
            CodeCategory::Godel => &[CodeId::Godel],
        }
    }
}
