use rand::Rng;

pub trait Cipher {
    fn encrypt(&self, text: &str) -> String;
    fn decrypt(&self, text: &str) -> String;
    fn randomize(&mut self);
}

pub const LATIN: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub struct Caesar {
    key: usize,
    alphabet: String,
    length: usize,
}

impl Caesar {
    pub fn new(key: usize, alphabet: &str) -> Caesar {
        Caesar{ key, alphabet: alphabet.to_string(), length: alphabet.chars().count() }
    }

    fn char_to_val(&self, c: char) -> usize {
        self.alphabet.chars().position(|x| x == c).unwrap()
    }

    fn val_to_char(&self, v: usize) -> char {
        self.alphabet.chars().nth(v).unwrap()
    }
}

impl Cipher for Caesar {
    fn encrypt(&self, text: &str) -> String {
        let symbols = text.chars();
        let mut out = "".to_string();
        for s in symbols {
            let n = (self.char_to_val(s) + self.key) % self.length;
            out.push(self.val_to_char(n))
        }
        out
    }

    fn decrypt(&self, text: &str) -> String {
        let symbols = text.chars();
        let mut out = "".to_string();
        for s in symbols {
            let n = (self.char_to_val(s) + self.length - self.key) % self.length;
            out.push(self.val_to_char(n))
        }
        out
    }

    fn randomize(&mut self) {
        let mut rng = rand::thread_rng();
        let length = self.alphabet.len();
        self.key = rng.gen_range(0..length);
    }
}