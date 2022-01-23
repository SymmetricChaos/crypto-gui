use rand::{Rng, prelude::ThreadRng};
use super::Cipher;

pub struct DecoderRing {
    pub index: usize,
    pub alphabet: String,
}

impl DecoderRing {

    pub fn new(index: usize, alphabet: &str) -> Self {
        DecoderRing{ index, alphabet: alphabet.to_string() }
    }

    pub fn length(&self) -> usize {
        self.alphabet.chars().count()
    }

    pub fn annie(&mut self) {
        self.alphabet = String::from("_ASLWIMVHFKXDPOEJBTNQZGUYRC");
    }

    pub fn midnight(&mut self) {
        self.alphabet = String::from("_AEXDTZKNYCJWSGUMBOQHRIVFPL");
    }

    fn valid_code_group(&self, s: &str) -> Result<usize, &'static str> {
        match s.parse::<usize>() {
            Ok(n) => if n < self.length() { Ok(n) } else { Err("Invalid code group") },
            Err(_) => return Err("Code groups must be numbers"),
        }
    }
}

impl Default for DecoderRing {
    fn default() -> Self {
        Self { index: 0, alphabet: String::from("_ABCDEFGHIJKLMNOPWRSTUVWXYZ") }
    }
}

impl Cipher for DecoderRing {

    fn encrypt(&self, text: &str) -> Result<String,&'static str> {
        let symbols = text.chars();
        let mut out = Vec::new();
        for s in symbols {
            let pos = self.alphabet.chars().position(|x| x == s);
            let n = match pos {
                Some(v) => (v + self.index) % self.length(),
                None => return Err("Unknown character encountered"),
            };
            out.push( format!("{}",n) )
        }
        Ok(out.join(" "))
    }

    fn decrypt(&self, text: &str) -> Result<String,&'static str> {
        let code_groups = text.split(' ');
        let nums =  {
            let mut v = Vec::with_capacity(code_groups.clone().count());
            for s in code_groups {
                let n = self.valid_code_group(s)?;
                v.push( (n + self.length() - self.index) % self.length());
            }
            v
        };
        let mut out = String::with_capacity(nums.len());
        for n in nums {
            // Unwrap is justified by the valid_code_groups method which catches both possibles sorts of errors
            out.push(self.alphabet.chars().nth(n).unwrap());
        }
        Ok(out)
    }

    fn randomize(&mut self, rng: &mut ThreadRng) {
        self.index = rng.gen_range(0..self.alphabet.len());
    }

    fn input_alphabet(&mut self) -> &mut String {
        &mut self.alphabet
    }

    fn output_alphabet(&mut self) -> &mut String {
        &mut self.alphabet
    }
}