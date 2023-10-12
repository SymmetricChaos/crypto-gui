use itertools::Itertools;

use crate::{
    errors::CodeError,
    letter_word_code::{IOMode, LetterWordIntCode},
    traits::Code,
};

use super::fibonacci_integers::FibonacciCodeIntegers;

// https://en.wikipedia.org/wiki/Fibonacci_coding

pub struct FibonacciCode {
    pub maps: LetterWordIntCode,
    pub mode: IOMode,
    pub integer_code: FibonacciCodeIntegers,
}

impl Default for FibonacciCode {
    fn default() -> Self {
        let codes = FibonacciCodeIntegers::default();

        let mut maps = LetterWordIntCode::new();
        maps.alphabet = String::from("ETAOINSHRDLCUMWFGYPBVKJXQZ");
        FibonacciCode {
            mode: IOMode::Integer,
            integer_code: codes,
            maps,
        }
    }
}

impl Code for FibonacciCode {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        if self.mode == IOMode::Integer {
            self.integer_code.encode(text)
        } else if self.mode == IOMode::Letter {
            let mut output = String::new();
            for c in text.chars() {
                let n = self.maps.char_to_int(c)?;
                output.push_str(&self.integer_code.encode_u32(n as u32))
            }
            Ok(output)
        } else {
            let mut output = String::new();
            for w in text.split(" ") {
                let n = self.maps.word_to_int(w)?;
                output.push_str(&self.integer_code.encode_u32(n as u32))
            }
            Ok(output)
        }
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
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
    fn encode_test_integer() {
        let mut code = FibonacciCode::default();
        code.mode = IOMode::Integer;
        assert_eq!(code.encode("1").unwrap(), "11");
    }

    #[test]
    fn decode_test_integer() {
        let mut code = FibonacciCode::default();
        code.mode = IOMode::Integer;
        assert_eq!(code.decode("11").unwrap(), "1");
    }

    #[test]
    fn encode_test_words() {
        let mut code = FibonacciCode::default();
        code.mode = IOMode::Word;
        code.maps.set_words(WORDS);
        assert_eq!(code.encode(PLAINTEXT_WORDS).unwrap(), ENCODEDTEXT_WORDS);
    }

    #[test]
    fn decode_test_words() {
        let mut code = FibonacciCode::default();
        code.mode = IOMode::Word;
        code.maps.set_words(WORDS);
        assert_eq!(code.decode(ENCODEDTEXT_WORDS).unwrap(), PLAINTEXT_WORDS);
    }
}
