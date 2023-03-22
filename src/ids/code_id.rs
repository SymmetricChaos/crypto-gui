use std::fmt::Display;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum CodeID {
    Ascii,
    MorseAmerican,
    MorseITU,
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
            CodeID::Ascii => "The American Standard Code for Information Interchange was created in the 1960s and consists of 128 characters sufficient to write essentially all English text and to control the printing device. Originally a 7-bit code it is now most often seen as an 8-bit code with the leading bit always set to zero. The limitations of ASCII became more apparent as it emerged as a defacto standard for computer text even outside the United States. Today the UTF-8 of Unicode encoding fully subsumes ASCII.",
            CodeID::Godel => "The Gödel encoding was created by Kurt Gödel as part of his proof of first Incompleteness Theorem in order to convert statements of mathematical logic into numbers that are then subject to mathematical logic. The version here can encode whatever kind of text you choose, however, because it produces huge numbers the maximum message is currently limited to 50 characters.\n\nThe encoding works as follows. Each symbol is assigned a positive natural number. Each symbol of the message is then assigned a prime number raised to the power of the number associated with the symbol and those values are all multiplied together. The original message can then be recovered by factoring. For example if A = 1, B = 2, and C = 3 the message BBC would be encoded as 2^2 * 3^2 * 5^3 = 4500.\n\nThe default encoding below puts the Basic Latin alphabet in order of frequncy as used in English in order to produce smaller numbers for most messages.",
            CodeID::MorseITU => "The best known version of Morse Code is the ITU Standard. It uses two kind of signals the 'dit' and 'dah' with the dah defined as three times the length of the dit. Morse code also requires periods with no signal, called spaces, in order to differentiate characters. The subset of ITU Morse below covers all the printing characters. Additional control signals and prosigns are not yet supported. The space between dits and dahs is the same length as a dit, between characters is a space of three dits, between words is a space of seven dits.", 
            CodeID::MorseAmerican => "An early, now obsolete, version of Morse Code",
            CodeID::Fibonacci => "The Fibonacci code is named because it uses the Fibonacci sequence to generate a variable width encoding of some arbitrary alphabet. More common characters are assigned shorter codes. This allows very large alphabets to be encoded efficiently so long as characters vary in frequency following a geometric distribution.",
            CodeID::Unary => "The Unary Encoding is the simplest prefix code and thus the simplest useful variable length code. No code word appears as a prefix of any other code word.",
            CodeID::SpellingAlphabet => "Spelling Alphabets or Phonetic Alphabets.",
            CodeID::Base64 => "Base64 is a binary code that is meant to re-encode arbitrary binary data as ASCII symbols that can then be transmitted safely through text channels and decoded on the other side.",
            CodeID::Unicode => "Unicode is an international standard for encoding of text using in most of the world writing systems with over 100,000 code points defined. There are three major encodings used called UTF-8, UTF-16, and UTF-32.",
            CodeID::Punycode => "Punycode is a method for re-encoding short Unicode strings using only ASCII characters, originally created for use with Internationalized Domain Names. The characters which are not ASCII are stripped out of the string, a delimeter character is placed after the remaining characters, then the non-ASCII characters are encoded onto the end using a method that records their position and Unicode codepoint. For example the sentence \"TạisaohọkhôngthểchỉnóitiếngViệt\" is encoded as \"TisaohkhngthchnitingVit-kjcr8268qyxafd2f1b9g\".",
            _ => "Missing description. Please complain to the author.",
        }
    }
}

impl Display for CodeID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            CodeID::Ascii => "ASCII",
            CodeID::MorseITU => "ITU Morse",
            CodeID::Godel => "Gödel",
            CodeID::Fibonacci => "Fibonacci",
            CodeID::MorseAmerican => "American Morse",
            CodeID::Baudot => "Baudot",
            CodeID::Base64 => "Base64",
            CodeID::Unary => "Unary",
            CodeID::SpellingAlphabet => "Spelling Alphabet",
            CodeID::Pgp => "PGP Word List",
            CodeID::Bacon => "Bacon",
            CodeID::Unicode => "Unicode",
            CodeID::Punycode => "Punycode",
        };
        write!(f, "{}", name)
    }
}

impl From<CodeID> for String {
    fn from(id: CodeID) -> Self {
        id.to_string()
    }
}
