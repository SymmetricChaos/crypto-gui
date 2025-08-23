use std::{fmt::Display, str::Chars};

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// One of several common alphabets
pub enum Alphabet {
    /// ABCDEFGHIJKLMNOPQRSTUVWXYZ
    BasicLatin,
    /// ABDEFGHIJKLMNOPQRSTUVWXYZ
    BasicLatinNoC,
    /// ABCDEFGHIKLMNOPQRSTUVWXYZ
    BasicLatinNoJ,
    /// ABCDEFGHIJKLMNOPRSTUVWXYZ
    BasicLatinNoQ,
    /// ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789
    Alphanumeric,
    /// The 94 printing ASCII symbols (not including space)
    Ascii94,
    /// The 95 printing ASCII symbols (including space)
    Ascii95,
    ///  The 128 ASCII symbols with control pictures for non-printing characters and the ASCII space character
    Ascii128,
    /// ABCDEFGHIKLMNOPQRSTVXYZ (lacks J, U, and W)
    ClassicalLatin,
    /// ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/
    Base64,
}

impl Alphabet {
    /// Pointer to a static string slice
    pub const fn slice(&self) -> &'static str {
        match self {
            Alphabet::BasicLatin =>    "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
            Alphabet::BasicLatinNoC => "ABDEFGHIJKLMNOPQRSTUVWXYZ",
            Alphabet::BasicLatinNoJ => "ABCDEFGHIKLMNOPQRSTUVWXYZ",
            Alphabet::BasicLatinNoQ => "ABCDEFGHIJKLMNOPRSTUVWXYZ",
            Alphabet::Alphanumeric => "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789",
            Alphabet::Ascii94 => "!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~",
            Alphabet::Ascii95 => " !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~",
            Alphabet::Ascii128 => "␀␁␂␃␄␅␆␇␈␉␊␋␌␍␎␏␐␑␒␓␔␕␖␗␘␙␚␛␜␝␞␟ !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~␡",
            Alphabet::ClassicalLatin => "ABCDEFGHIKLMNOPQRSTVXYZ",
            Alphabet::Base64 => "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/",
        }
    }

    /// Owned string
    pub fn string(&self) -> String {
        self.slice().to_string()
    }

    /// Length in Unicode code points
    pub fn len(&self) -> usize {
        // This could be a match statement but this is easier
        self.slice().chars().count()
    }

    /// Iterate over characters
    pub fn chars(&self) -> Chars<'_> {
        self.slice().chars()
    }

    /// Does the character exist in the alphabet?
    pub fn contains(&self, c: &char) -> bool {
        self.chars().contains(c)
    }

    /// Position of a character in the alphabet
    pub fn position(&self, c: char) -> Option<usize> {
        self.chars().position(|x| c == x)
    }

    /// Pretty name
    pub fn name(&self) -> &'static str {
        match self {
            Alphabet::BasicLatin => "Basic Latin",
            Alphabet::BasicLatinNoC => "Basic Latin, No C",
            Alphabet::BasicLatinNoJ => "Basic Latin, No J",
            Alphabet::BasicLatinNoQ => "Basic Latin, No Q",
            Alphabet::Alphanumeric => "Alphanumeric",
            Alphabet::Ascii94 => "ASCII",
            Alphabet::Ascii95 => "ASCII (with space)",
            Alphabet::Ascii128 => "Full ASCII",
            Alphabet::ClassicalLatin => "Classical Latin",
            Alphabet::Base64 => "Base64",
        }
    }
}

impl Display for Alphabet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.slice())
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

impl AsRef<str> for Alphabet {
    fn as_ref(&self) -> &str {
        self.slice()
    }
}
