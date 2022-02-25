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
    pub inner: String,
}
 
impl Alphabet {
 
    // Length in characters is what we need
    pub fn len(&self) -> usize {
        self.inner.chars().count()
    }
 
    pub fn nth(&self, n: usize, offset: i32) -> Option<char> {
        let len = self.len();
        let idx = ((n + len) as i32 + offset) as usize % len;
        self.inner.chars().nth(idx)
    }
 
    pub fn pos(&self, c: char, offset: i32) -> Option<usize> {
        let shift = (self.len() as i32 - offset) as usize % self.len();
        Some((self.inner.chars().position(|x| x == c)? + shift) % self.len())
    }
 
    pub fn show(&self, offset: i32) -> String {
        let shift = (self.len() as i32 + offset) as usize % self.len();
        let mut out = String::with_capacity(self.inner.len());
        out.push_str(&self.inner[shift..]);
        out.push_str(&self.inner[0..shift]);
        out
    }
 
    pub fn slice(&self) -> &str {
        &self.inner
    }
 
    pub fn contains(&self, c: char) -> bool {
        self.inner.contains(c)
    }

    pub fn offset_char(&self, c: char, offset: i32) -> Option<char> {
        let pos = self.pos(c, 0)?;
        self.nth(pos, offset)
    }
 
}

impl From<PresetAlphabet> for Alphabet {
    fn from(alpha: PresetAlphabet) -> Self {
        Self{ inner: String::from(alpha) }
    }
}

impl From<String> for Alphabet {
    fn from(string: String) -> Self {
        Self{ inner: string }
    }
}
 
impl From<&str> for Alphabet {
    fn from(string: &str) -> Self {
        Self{ inner: string.to_string() }
    }
}
 
impl From<Alphabet> for String {
    fn from(alphabet: Alphabet) -> Self {
        alphabet.inner
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

    #[test]
    fn offset_char() {
        let alphabet = Alphabet::from("ABCD");
        assert_eq!(alphabet.offset_char('C',1).unwrap(),'D');
    }
 
	// Offset should behave as expected even if it is negative
    #[test]
    fn show_offset_neg() {
        let alphabet = Alphabet::from("ABCD");
            assert_eq!(alphabet.show(-1),"DABC");
    }
 
    #[test]
    fn nth_offset_neg()  {
        let alphabet = Alphabet::from("ABCD");
        assert_eq!(alphabet.nth(3,-1).unwrap(),'C');
    }
 
    #[test]
    fn pos_offset_neg() {
        let alphabet = Alphabet::from("ABCD");
        assert_eq!(alphabet.pos('C',-1).unwrap(),3);
    }

    #[test]
    fn offset_char_neg() {
        let alphabet = Alphabet::from("ABCD");
        assert_eq!(alphabet.offset_char('C',-1).unwrap(),'B');
    }
 
}