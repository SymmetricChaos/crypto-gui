use crate::ids::CodeID;

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

    pub fn all_cipher_in_category(&self) -> &[CodeID] {
        match self {
            CodeCategory::Morse => &[CodeID::MorseAmerican, CodeID::MorseITU],
            CodeCategory::Binary => &[
                CodeID::Ascii,
                CodeID::Bacon,
                CodeID::Baudot,
                CodeID::Base64,
                CodeID::Fibonacci,
            ],
            CodeCategory::Unary => &[CodeID::Unary],
            CodeCategory::Spelling => &[CodeID::Pgp, CodeID::SpellingAlphabet],
            CodeCategory::Godel => &[CodeID::Godel],
        }
    }
}
