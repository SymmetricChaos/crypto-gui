use rand::Rng;
use super::cipher_trait::Cipher;

pub struct Caesar {
    key: usize,
    alphabet: String,
    length: usize,
}

impl Caesar {
    pub fn new(key: usize, alphabet: &str) -> Caesar {
        Caesar{ key, alphabet: alphabet.to_string(), length: alphabet.chars().count() }
    }

    fn char_to_val(&self, c: char) -> Option<usize> {
        self.alphabet.chars().position(|x| x == c)
    }

    fn val_to_char(&self, v: usize) -> Option<char> {
        self.alphabet.chars().nth(v)
    }
}

impl Cipher for Caesar {
    fn encrypt(&self, text: &str) -> Result<String,&'static str> {
        let symbols = text.chars();
        let mut out = "".to_string();
        for s in symbols {
            let val = self.char_to_val(s);
            let n = match val {
                Some(v) => (v + self.key) % self.length,
                None => return Err("Unknown character encountered")
            };
            let char = match self.val_to_char(n) {
                Some(c) => c,
                None => return Err("Unknown character encountered")
            };
            out.push(char)
        }
        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String,&'static str> {
        let symbols = text.chars();
        let mut out = "".to_string();
        for s in symbols {
            let val = self.char_to_val(s);
            let n = match val {
                Some(v) => (v + self.length - self.key) % self.length,
                None => return Err("Unknown character encountered")
            };
            let char = match self.val_to_char(n) {
                Some(c) => c,
                None => return Err("Unknown character encountered")
            };
            out.push(char)
        }
        Ok(out)
    }

    fn randomize(&mut self) {
        let mut rng = rand::thread_rng();
        let length = self.alphabet.len();
        self.key = rng.gen_range(0..length);
    }
}