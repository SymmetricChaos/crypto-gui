use super::{Code, FibonacciCodeIntegers};
use crate::errors::Error;
use bimap::BiMap;
use itertools::Itertools;

// https://en.wikipedia.org/wiki/Fibonacci_coding

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FibMode {
    Letter,
    Word,
    Integer,
}

pub struct FibonacciCode {
    letter_map: BiMap<char, String>,
    word_map: BiMap<String, String>,
    pub alphabet: String,
    pub words: Vec<String>,
    pub words_string: String,
    pub mode: FibMode,
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
    pub fn set_letter_map(&mut self) {
        self.letter_map.clear();
        for (n, c) in self.alphabet.chars().enumerate() {
            self.letter_map
                .insert(c.clone(), self.integer_code.encode_u32((n + 1) as u32));
        }
    }

    pub fn set_word_map(&mut self) {
        self.words = self
            .words_string
            .split(",")
            .map(|w| w.trim().to_string())
            .collect_vec();
        self.word_map.clear();
        for (n, c) in self.words.iter().enumerate() {
            self.word_map
                .insert(c.clone(), self.integer_code.encode_u32((n + 1) as u32));
        }
    }

    pub fn chars_codes(&mut self) -> impl Iterator<Item = (char, &String)> + '_ {
        self.alphabet
            .chars()
            .map(|x| (x, self.letter_map.get_by_left(&x).unwrap()))
    }

    pub fn words_codes(&mut self) -> impl Iterator<Item = (&String, &String)> + '_ {
        self.words
            .iter()
            .map(|x| (x, self.word_map.get_by_left(x).unwrap()))
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
            letter_map: map,
            alphabet,
            mode: FibMode::Integer,
            integer_code: codes,
            word_map: BiMap::new(),
            words: Vec::new(),
            words_string: String::new(),
        }
    }
}

impl Code for FibonacciCode {
    fn encode(&self, text: &str) -> Result<String, Error> {
        if self.mode == FibMode::Integer {
            self.integer_code.encode(text)
        } else if self.mode == FibMode::Letter {
            let mut output = String::new();
            for s in text.chars() {
                let code = self
                    .letter_map
                    .get_by_left(&s)
                    .ok_or_else(|| Error::invalid_input_char(s))?;
                output.push_str(&code)
            }
            Ok(output)
        } else {
            let mut output = String::new();
            for w in text.split(" ") {
                let code = self
                    .word_map
                    .get_by_left(w)
                    .ok_or_else(|| Error::invalid_input_group(w))?;
                output.push_str(code)
            }
            Ok(output)
        }
    }

    fn decode(&self, text: &str) -> Result<String, Error> {
        if self.mode == FibMode::Integer {
            self.integer_code.decode(text)
        } else if self.mode == FibMode::Letter {
            let mut output = String::new();
            let mut buffer = String::with_capacity(8);
            let mut prev = '0';
            for b in text.chars() {
                buffer.push(b);
                if prev == '1' && b == '1' {
                    match self.letter_map.get_by_right(&buffer) {
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
        } else {
            let mut output = Vec::new();
            let e = String::from("�");
            for n in self.integer_code.decode_to_u32(text)?.into_iter() {
                match self.words.get((n - 1) as usize) {
                    Some(w) => output.push(w),
                    None => output.push(&e),
                }
            }
            Ok(output.into_iter().join(" "))
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

    const WORDS: &'static str = "at, attack, retreat, dusk, dawn, noon";
    const PLAINTEXT_WORDS: &'static str = "attack at noon";
    const ENCODEDTEXT_WORDS: &'static str = "0111110011";

    #[test]
    fn encode_test() {
        let code = FibonacciCode::default();
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT);
    }

    #[test]
    fn decode_test() {
        let code = FibonacciCode::default();
        assert_eq!(code.decode(ENCODEDTEXT).unwrap(), PLAINTEXT);
    }

    #[test]
    fn encode_test_words() {
        let mut code = FibonacciCode::default();
        code.mode = FibMode::Word;
        code.words_string = String::from(WORDS);
        code.set_word_map();
        assert_eq!(code.encode(PLAINTEXT_WORDS).unwrap(), ENCODEDTEXT_WORDS);
    }

    #[test]
    fn decode_test_words() {
        let mut code = FibonacciCode::default();
        code.mode = FibMode::Word;
        code.words_string = String::from(WORDS);
        code.set_word_map();
        assert_eq!(code.decode(ENCODEDTEXT_WORDS).unwrap(), PLAINTEXT_WORDS);
    }
}
