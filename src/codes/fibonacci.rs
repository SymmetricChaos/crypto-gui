use super::{Code, FibonacciCodeIntegers, IOMode, LetterAndWordCode};
use crate::errors::Error;
use itertools::Itertools;

// https://en.wikipedia.org/wiki/Fibonacci_coding

pub struct FibonacciCode {
    pub maps: LetterAndWordCode<String>,
    pub mode: IOMode,
    pub integer_code: FibonacciCodeIntegers,
}

impl FibonacciCode {
    pub fn set_letter_map(&mut self) {
        self.maps
            .set_letter_map(|(n, _)| self.integer_code.encode_u32((n + 1) as u32))
    }

    pub fn set_word_map(&mut self) {
        self.maps
            .set_word_map(|(n, _)| self.integer_code.encode_u32((n + 1) as u32))
    }
}

impl Default for FibonacciCode {
    fn default() -> Self {
        let codes = FibonacciCodeIntegers::default();

        let mut maps = LetterAndWordCode::<String>::default();
        maps.alphabet = String::from("ETAOINSHRDLCUMWFGYPBVKJXQZ");
        maps.set_letter_map(|(n, _)| codes.encode_u32((n + 1) as u32));
        FibonacciCode {
            mode: IOMode::Integer,
            integer_code: codes,
            maps,
        }
    }
}

impl Code for FibonacciCode {
    fn encode(&self, text: &str) -> Result<String, Error> {
        if self.mode == IOMode::Integer {
            self.integer_code.encode(text)
        } else if self.mode == IOMode::Letter {
            let mut output = String::new();
            for c in text.chars() {
                let code = self.maps.get_by_letter(c)?;
                output.push_str(&code)
            }
            Ok(output)
        } else {
            let mut output = String::new();
            for w in text.split(" ") {
                let code = self.maps.get_by_word(w)?;
                output.push_str(code)
            }
            Ok(output)
        }
    }

    fn decode(&self, text: &str) -> Result<String, Error> {
        if self.mode == IOMode::Integer {
            self.integer_code.decode(text)
        } else if self.mode == IOMode::Letter {
            let mut output = String::new();
            for n in self.integer_code.decode_to_u32(text)?.into_iter() {
                // n == 0 can only occur as the last number and only as a signal that the final code was incomplete
                if n == 0 {
                    output.push('�')
                }
                match self.maps.alphabet.chars().nth((n - 1) as usize) {
                    Some(w) => output.push(w),
                    None => output.push('�'),
                }
            }
            Ok(output)
        } else {
            let mut output = Vec::new();
            let e = String::from("�");
            for n in self.integer_code.decode_to_u32(text)?.into_iter() {
                if n == 0 {
                    output.push(&e);
                }
                match self.maps.words.get((n - 1) as usize) {
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
        let mut code = FibonacciCode::default();
        code.mode = IOMode::Letter;
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT);
    }

    #[test]
    fn decode_test() {
        let mut code = FibonacciCode::default();
        code.mode = IOMode::Letter;
        assert_eq!(code.decode(ENCODEDTEXT).unwrap(), PLAINTEXT);
    }

    #[test]
    fn encode_test_words() {
        let mut code = FibonacciCode::default();
        code.mode = IOMode::Word;
        code.maps.words_string = String::from(WORDS);
        code.set_word_map();
        assert_eq!(code.encode(PLAINTEXT_WORDS).unwrap(), ENCODEDTEXT_WORDS);
    }

    #[test]
    fn decode_test_words() {
        let mut code = FibonacciCode::default();
        code.mode = IOMode::Word;
        code.maps.words_string = String::from(WORDS);
        code.set_word_map();
        assert_eq!(code.decode(ENCODEDTEXT_WORDS).unwrap(), PLAINTEXT_WORDS);
    }
}
