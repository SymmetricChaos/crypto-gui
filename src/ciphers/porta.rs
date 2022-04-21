lazy_static! {
    pub static ref PORTA_TABLEAUX: [&'static str; 13] = vec![
        "NOPQRSTUVWXYZABCDEFGHIJKLM",
        "OPQRSTUVWXYZNMABCDEFGHIJKL",
        "PQRSTUVWXYZNOLMABCDEFGHIJK",
        "QRSTUVWXYZNOPKLMABCDEFGHIJ",
        "RSTUVWXYZNOPQJKLMABCDEFGHI",
        "STUVWXYZNOPQRIJKLMABCDEFGH",
        "TUVWXYZNOPQRSHIJKLMABCDEFG",
        "UVWXYZNOPQRSTGHIJKLMABCDEF",
        "VWXYZNOPQRSTUFGHIJKLMABCDE",
        "WXYZNOPQRSTUVEFGHIJKLMABCD",
        "XYZNOPQRSTUVWDEFGHIJKLMABC",
        "YZNOPQRSTUVWXCDEFGHIJKLMAB",
        "ZNOPQRSTUVWXYBCDEFGHIJKLMA"
    ];
}
 
 
/// Porta Cipher uses a sequence of 13 alphabets to encrypt characters. The visible pattern ensures the cipher is reciprocal.
pub struct Porta {
    tableaux: [&'static str; 13],
    pub key: String,
    key_vals: Vec<usize>,
    alpahbet: Alphabet,
}
 
impl Default for Porta {
    fn default() -> Self {
        Self{ 
            tableaux: PORTA_TABLEAUX.clone(), 
            key: String::new(), 
            key_vals: Vec::new(), 
            alphabet: Alphabet::from(PresetAlphabet::BasicLatin) 
        }
    }
}
 
impl Porta {
    pub fn control_key(&mut self) -> &mut String {
        self.key_vals = 
        &mut self.key
    }
 
    pub fn display_tableaux(&self) -> String {
 
    }
}
 
impl Cipher for Porta {
 
    fn encrypt(&self, text: &str) -> Result<String,CipherError> {
        let mut out = String::with_capacity(text.len());
        let ckey = self.key_vals.iter().cycle();
        for (c, k) in text.chars().zip(ckey) {
            let row = self.tableaux.get(k);
            let pos = row.chars().position(|x| x == c).unwrap();
            out.push(self.alphabet.chars().nth(p).unwrap())
        }
        Ok(out)
    }
 
    // The Porta cipher is reciprocal
    fn decrypt(&self, text: &str) -> Result<String,CipherError> {
        self.encrypt(text)
    }
 
    fn reset(&mut self) {
        *self = Self::default()
    }
 
}