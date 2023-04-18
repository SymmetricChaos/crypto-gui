use super::{Code, IOMode, LetterAndWordCode, LevenshteinCodeIntegers};
use crate::errors::Error;

// https://en.wikipedia.org/wiki/Levenshtein_coding

pub struct LevenshteinCode {
    pub maps: LetterAndWordCode<String>,
    pub mode: IOMode,
    pub integer_code: LevenshteinCodeIntegers,
}

impl LevenshteinCode {
    pub fn set_letter_map(&mut self) {
        self.maps
            .set_letter_map(|(n, _)| self.integer_code.encode_u32((n + 1) as u32))
    }

    pub fn set_word_map(&mut self) {
        self.maps
            .set_word_map(|(n, _)| self.integer_code.encode_u32((n + 1) as u32))
    }
}

impl Default for LevenshteinCode {
    fn default() -> Self {
        let codes = LevenshteinCodeIntegers::default();

        let mut maps = LetterAndWordCode::<String>::default();
        maps.alphabet = String::from("ETAOINSHRDLCUMWFGYPBVKJXQZ");
        maps.set_letter_map(|(n, _)| codes.encode_u32((n + 1) as u32));
        LevenshteinCode {
            mode: IOMode::Integer,
            integer_code: codes,
            maps,
        }
    }
}

impl Code for LevenshteinCode {
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
        todo!()
    }

    fn randomize(&mut self) {
        todo!()
    }

    fn reset(&mut self) {
        todo!()
    }
}
