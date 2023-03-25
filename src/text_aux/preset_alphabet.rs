use std::str::Chars;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PresetAlphabet {
    BasicLatin,
    BasicLatinNoJ,
    BasicLatinNoQ,
    BasicLatinWithDigits,
    Digits0,
    Digits1,
    Ascii94,  // The printing ASCII symbols without the space
    Ascii95,  // The printing ASCII symbols with the space
    Ascii128, // The ASCII symbols with control pictures for non-printing characters including space
    AsciiLdh, // The LDH (letter, digit, hyphen) subset of ASCII used by IDNA, in ascending order per ASCII code
    Greek,
    ClassicalLatin, //Classical Latin
    Base64, // 64 safe to use ASCII symbols, low chance of being interpreted if the string is parsed
    Spanish,
    German,
}

impl PresetAlphabet {
    // Pointer to a static string slice
    pub fn slice(&self) -> &'static str {
        match self {
            PresetAlphabet::BasicLatin => "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
            PresetAlphabet::BasicLatinNoJ => "ABCDEFGHIKLMNOPQRSTUVWXYZ",
            PresetAlphabet::BasicLatinNoQ => "ABCDEFGHIJKLMNOPRSTUVWXYZ",
            PresetAlphabet::BasicLatinWithDigits => "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789",
            PresetAlphabet::Digits0 => "0123456789",
            PresetAlphabet::Digits1 => "1234567890",
            PresetAlphabet::Ascii94 => "!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~",
            PresetAlphabet::Ascii95 => " !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~",
            PresetAlphabet::Ascii128 => "␀␁␂␃␄␅␆␇␈␉␊␋␌␍␎␏␐␑␒␓␔␕␖␗␘␙␚␛␜␝␞␟␠!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~␡",
            PresetAlphabet::AsciiLdh => "-0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz",
            PresetAlphabet::Greek => "ΑΒΓΔΕΖΗΘΙΚΛΜΝΞΟΠΡΣΤΥΦΧΨΩ", //All of these are Unicode Greek even the ones draw identically to ASCII
            PresetAlphabet::ClassicalLatin => "ABCDEFGHIKLMNOPQRSTVXYZ",
            PresetAlphabet::Base64 => "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/",
            PresetAlphabet::Spanish => "ABCDEFGHIJKLMNÑOPQRSTUVWXYZ",
            PresetAlphabet::German => "ABCDEFGHIJKLMNOPQRSTUVWXYZÄÖÜß",
        }
    }

    // Owned string
    pub fn string(&self) -> String {
        self.slice().to_string()
    }

    // Length in Unicode characters
    pub fn len(&self) -> usize {
        // This could be a match statement but this is easier
        self.slice().chars().count()
    }

    // Iterate over characters
    pub fn chars(&self) -> Chars<'_> {
        self.slice().chars()
    }
}

impl From<PresetAlphabet> for String {
    fn from(alphabet: PresetAlphabet) -> Self {
        alphabet.string()
    }
}

impl From<PresetAlphabet> for &'static str {
    fn from(alphabet: PresetAlphabet) -> Self {
        alphabet.slice()
    }
}
