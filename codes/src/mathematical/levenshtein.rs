use itertools::Itertools;

use crate::{
    errors::CodeError,
    letter_word_code::{IOMode, LetterWordIntCode},
    traits::Code,
};

use super::levenshtein_integers::LevenshteinCodeIntegers;

// https://en.wikipedia.org/wiki/Levenshtein_coding

pub struct LevenshteinCode {
    pub maps: LetterWordIntCode,
    pub mode: IOMode,
    pub integer_code: LevenshteinCodeIntegers,
}

impl Default for LevenshteinCode {
    fn default() -> Self {
        let codes = LevenshteinCodeIntegers::default();

        let mut maps = LetterWordIntCode::new();
        maps.alphabet = String::from("ETAOINSHRDLCUMWFGYPBVKJXQZ");
        // maps.set_letter_map(|(n, _)| codes.encode_u32((n + 1) as u32));
        LevenshteinCode {
            mode: IOMode::Integer,
            integer_code: codes,
            maps,
        }
    }
}

impl Code for LevenshteinCode {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        if self.mode == IOMode::Integer {
            self.integer_code.encode(text)
        } else if self.mode == IOMode::Letter {
            let mut output = String::new();
            for c in text.chars() {
                let code = self.maps.char_to_int(c)? as u32;
                output.push_str(&self.integer_code.encode_u32(code))
            }
            Ok(output)
        } else {
            let mut output = String::new();
            for w in text.split(" ") {
                let code = self.maps.word_to_int(w)? as u32;
                output.push_str(&self.integer_code.encode_u32(code))
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
                match self.maps.alphabet.chars().nth((n - 1) as usize) {
                    Some(w) => output.push(w),
                    None => output.push('?'),
                }
            }
            Ok(output)
        } else {
            let mut output = Vec::new();
            let e = String::from("?");
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
