use std::str::Chars;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alphabet {
    BasicLatin,
    BasicLatinNoC,
    BasicLatinNoJ,
    BasicLatinNoQ,
    BasicLatinWithDigits,
    Digits0,        // Digits starting at 0 and ending at 9
    Digits1,        // Digits start at 1 and ending with 0
    Ascii94,        // The printing ASCII symbols without the space
    Ascii95,        // The printing ASCII symbols with the space
    Ascii128, // The ASCII symbols with control pictures for non-printing characters except space
    AsciiLdh, // The LDH (letter, digit, hyphen) subset of ASCII used by IDNA, in ascending order per ASCII code
    ClassicalLatin, // Classical Latin lacks J, U, and W
    Base64, // 64 safe to use ASCII symbols, low chance of being interpreted if the string is parsed
}

impl Alphabet {
    // Pointer to a static string slice
    pub fn slice(&self) -> &'static str {
        match self {
            Alphabet::BasicLatin =>    "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
            Alphabet::BasicLatinNoC => "ABDEFGHIJKLMNOPQRSTUVWXYZ",
            Alphabet::BasicLatinNoJ => "ABCDEFGHIKLMNOPQRSTUVWXYZ",
            Alphabet::BasicLatinNoQ => "ABCDEFGHIJKLMNOPRSTUVWXYZ",
            Alphabet::BasicLatinWithDigits => "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789",
            Alphabet::Digits0 => "0123456789",
            Alphabet::Digits1 => "1234567890",
            Alphabet::Ascii94 => "!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~",
            Alphabet::Ascii95 => " !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~",
            Alphabet::Ascii128 => "␀␁␂␃␄␅␆␇␈␉␊␋␌␍␎␏␐␑␒␓␔␕␖␗␘␙␚␛␜␝␞␟ !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~␡",
            Alphabet::AsciiLdh => "-0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz",
            Alphabet::ClassicalLatin => "ABCDEFGHIKLMNOPQRSTVXYZ",
            Alphabet::Base64 => "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/",
        }
    }

    // Owned string
    pub fn string(&self) -> String {
        self.slice().to_string()
    }

    // Length in Unicode code points
    pub fn len(&self) -> usize {
        // This could be a match statement but this is easier
        self.slice().chars().count()
    }

    // Iterate over characters
    pub fn chars(&self) -> Chars<'_> {
        self.slice().chars()
    }
}

impl From<Alphabet> for String {
    fn from(alphabet: Alphabet) -> Self {
        alphabet.string()
    }
}

impl From<Alphabet> for &'static str {
    fn from(alphabet: Alphabet) -> Self {
        alphabet.slice()
    }
}
