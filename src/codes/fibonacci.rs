use super::{Code, FibonacciCodeIntegers};
use crate::errors::Error;
use bimap::BiMap;

// https://en.wikipedia.org/wiki/Fibonacci_coding

pub struct FibonacciCode {
    map: BiMap<char, String>,
    pub alphabet: String,
    old_alphabet: String,
    max_code_len: usize,
    pub integer_mode: bool,
    pub integer_code: FibonacciCodeIntegers,
}

impl FibonacciCode {
    // This needs to be called before encoding or decoding to be
    // sure that the maps are up to date. In the egui interface
    // this is taken care of by embedding it in the chars_codes()
    // method.
    // It would make more sense to put it in the control_alphabet()
    // method but that causes a panic due to interaction with
    // the chars_codes() method.
    pub fn set_map(&mut self) {
        if self.alphabet != self.old_alphabet {
            self.map.clear();
            for (n, c) in self.alphabet.chars().enumerate() {
                self.map
                    .insert(c.clone(), self.integer_code.encode_u32((n + 1) as u32));
            }
            self.max_code_len = self
                .map
                .get_by_left(&self.alphabet.chars().last().unwrap())
                .unwrap()
                .chars()
                .count();
            self.old_alphabet = self.alphabet.clone();
        }
    }

    pub fn chars_codes(&mut self) -> impl Iterator<Item = (char, &String)> + '_ {
        self.set_map();
        self.alphabet
            .chars()
            .map(|x| (x, self.map.get_by_left(&x).unwrap()))
    }
}

impl Default for FibonacciCode {
    fn default() -> Self {
        let alphabet = String::from("ETAOINSHRDLCUMWFGYPBVKJXQZ");
        let codes = FibonacciCodeIntegers::default();
        let mut map = BiMap::new();
        for (n, c) in alphabet.chars().enumerate() {
            map.insert(c, codes.encode_u32((n + 1) as u32));
        }
        FibonacciCode {
            map,
            alphabet: alphabet.clone(),
            old_alphabet: alphabet,
            max_code_len: 8,
            integer_mode: false,
            integer_code: codes,
        }
    }
}

impl Code for FibonacciCode {
    fn encode(&self, text: &str) -> Result<String, Error> {
        if self.integer_mode {
            self.integer_code.encode(text)
        } else {
            let mut output = String::new();
            for s in text.chars() {
                let code = self
                    .map
                    .get_by_left(&s)
                    .ok_or_else(|| Error::invalid_input_char(s))?;
                output.push_str(&code)
            }
            Ok(output)
        }
    }

    fn decode(&self, text: &str) -> Result<String, Error> {
        if self.integer_mode {
            self.integer_code.decode(text)
        } else {
            let mut output = String::new();
            let mut buffer = String::with_capacity(self.max_code_len);
            let mut prev = '0';
            for b in text.chars() {
                buffer.push(b);
                if prev == '1' && b == '1' {
                    match self.map.get_by_right(&buffer) {
                        Some(s) => {
                            output.push(*s);
                        }
                        None => {
                            output.push('�');
                        }
                    }
                    buffer.clear();
                    prev = '0';
                    continue;
                }
                prev = b;
            }
            Ok(output)
        }
    }

    fn randomize(&mut self) {}

    fn reset(&mut self) {}
}

#[cfg(test)]
mod fibonacci_tests {
    use super::*;

    const PLAINTEXT: &'static str = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    const ENCODEDTEXT: &'static str = "01100001111101000110000011000111010111000001101010111000111011010001110011001001110110010001101000011000001110000111001011010111011000000111110001101100001111001011001100010011000101101001110111010011";

    #[test]
    fn encrypt_test() {
        let code = FibonacciCode::default();
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT);
    }

    #[test]
    fn decrypt_test() {
        let code = FibonacciCode::default();
        assert_eq!(code.decode(ENCODEDTEXT).unwrap(), PLAINTEXT);
    }
}
