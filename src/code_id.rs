use std::fmt::Display;


#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum CodeID {
    Ascii,
    Morse,
    Godel,
    Fibonacci,
}

impl Default for CodeID {
    fn default() -> Self {
        Self::Ascii
    }
}

impl CodeID {
    // Describe the history of the code
    pub fn description(&self) -> &'static str {
        match self {
            CodeID::Godel => "The Gödel encoding was created by Kurt Gödel as part of his proof of first Incompleteness Theorem in order to convert statements of mathematical logic into numbers that are then subject to mathematical logic. The version here can encode whatever kind of text you choose, however, because it produces huge numbers the maximum message is currently limited to 50 characters.\n\nThe encoding works as follows. Each symbol is assigned a positive natural number. Each symbol of the message is then assigned a prime number raised to the power of the number associated with the symbol and those values are all multiplied together. The original message can then be recovered by factoring. For example if A = 1, B = 2, and C = 3 the message BBC would be encoded as 2^2 * 3^2 * 5^3 = 4500.\n\nThe default encoding below puts the Basic Latin alphabet in order of frequncy as used in English in order to produce smaller numbers.",
            _ => "Missing description. Please complain to the author.",
        }
    }
}



impl Display for  CodeID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            CodeID::Ascii => "ASCII",
            CodeID::Morse => "Morse",
            CodeID::Godel => "Gödel",
            CodeID::Fibonacci => "Fibonacci",
        };
        write!(f,"{}",name)
    }
}

impl From<CodeID> for String {
    fn from(id: CodeID) -> Self {
        id.to_string()
    }
}