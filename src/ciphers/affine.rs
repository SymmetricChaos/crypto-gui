use rand::Rng;
use super::cipher_trait::Cipher;

use crate::math::mul_inv;

pub struct Affine {
    add_key: usize,
    mul_key: usize,
    mul_key_inv: Option<usize>,
    alphabet: String,
    length: usize,
}

impl Affine {
    pub fn new(add_key: usize, mul_key: usize, alphabet: &str) -> Self {
        let mul_key_inv = mul_inv(mul_key, alphabet.chars().count());
        Self{ add_key, mul_key, mul_key_inv, alphabet: alphabet.to_string(), length: alphabet.chars().count() }
    }

    fn char_to_val(&self, c: char) -> Option<usize> {
        self.alphabet.chars().position(|x| x == c)
    }

    fn val_to_char(&self, v: usize) -> Option<char> {
        self.alphabet.chars().nth(v)
    }
}

impl Cipher for Affine {
    fn encrypt(&self, text: &str) -> Result<String,&'static str> {
        let symbols = text.chars();
        let mut out = "".to_string();
        match self.mul_key_inv {
            Some(n) => n,
            None => return Err("The multiplicative key of an Affine Cipher must have an inverse modulo the length of the alphabet")
        };
        for s in symbols {
            let val = self.char_to_val(s);
            let n = match val {
                Some(v) => (v * self.mul_key + self.add_key) % self.length,
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
        let mki = match self.mul_key_inv {
            Some(n) => n,
            None => return Err("The multiplicative key of an Affine Cipher must have an inverse modulo the length of the alphabet")
        };
        for s in symbols {
            let val = self.char_to_val(s);
            let n = match val {
                Some(v) => ((v + self.length - self.add_key) * mki) % self.length,
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
        self.add_key = rng.gen_range(0..length);
        let (mul, mult_inv) = loop  {
            let mul = rng.gen_range(0..length);
            if let Some(n) = mul_inv(mul, self.length) {
                break (mul, n)
            };
        };
        self.mul_key = mul;
        self.mul_key_inv = Some(mult_inv);
    }
}