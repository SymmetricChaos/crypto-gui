
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PresetAlphabet {
    BasicLatin,
    BasicLatinNoJ,
    BasicLatinNoQ,
    BasicLatinWithDigits,
    Digits0,
    Digits1,
    Ascii94, // The printing ASCII symbols without the space
    Ascii95, // The printing ASCII symbols with the space
    Greek,
    Latin, //Classical Latin
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
            PresetAlphabet::Greek => "ΑΒΓΔΕΖΗΘΙΚΛΜΝΞΟΠΡΣΤΥΦΧΨΩ", //All of these are Unicode Greek even the ones draw identically to ASCII
            PresetAlphabet::Latin => "ABCDEFGHIKLMNOPQRSTVXY",
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
    pub fn chars(&self) -> std::str::Chars {
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



#[derive(Clone, Debug)]
pub struct Alphabet {
    alpha: String,
}

impl Alphabet {

    pub fn len(&self) -> usize {
        self.alpha.chars().count()
    }

    pub fn nth(&self, n: usize, offset: usize) -> Option<char> {
        let n = (n + offset) % self.len();
        self.alpha.chars().nth(n)
    }

    pub fn pos(&self, c: char, offset: usize) -> Option<usize> {
        Some((self.alpha.chars().position(|x| x == c)? + self.len() - offset)  % self.len())
    }

    pub fn show(&self, offset: usize) -> String {
        let mut out = String::with_capacity(self.alpha.len());
        out.push_str(&self.alpha[offset..]);
        out.push_str(&self.alpha[0..offset]);
        out
    }

}

impl From<PresetAlphabet> for Alphabet {
    fn from(alphabet: PresetAlphabet) -> Self {
        Self{ alpha: String::from(alphabet) }
    }
}

impl From<String> for Alphabet {
    fn from(string: String) -> Self {
        Self{ alpha: string }
    }
}

impl From<&str> for Alphabet {
    fn from(string: &str) -> Self {
        Self{ alpha: string.to_string() }
    }
}



#[cfg(test)]
mod alphabet_tests {
    use super::*;

    #[test]
    fn show_offset() {
        let alphabet = Alphabet::from("ABCD");
        assert_eq!(alphabet.show(1),"BCDA");
    }

    #[test]
    fn nth_offset()  {
        let alphabet = Alphabet::from("ABCD");
        assert_eq!(alphabet.nth(1,1).unwrap(),'C');
    }

    #[test]
    fn pos_offset() {
        let alphabet = Alphabet::from("ABCD");
        assert_eq!(alphabet.pos('C',1).unwrap(),1);
    }

}
