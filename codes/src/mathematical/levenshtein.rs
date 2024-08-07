use crate::{
    errors::CodeError,
    letter_word_code::{IOMode, IntegerCodeMaps},
    traits::Code,
};

use super::levenshtein_integers::LevenshteinCodeIntegers;

// https://en.wikipedia.org/wiki/Levenshtein_coding

pub struct LevenshteinCode {
    pub maps: IntegerCodeMaps,
    pub mode: IOMode,
    pub integer_code: LevenshteinCodeIntegers,
    pub spaced: bool,
}

impl Default for LevenshteinCode {
    fn default() -> Self {
        let codes = LevenshteinCodeIntegers::default();
        let mut maps = IntegerCodeMaps::new();
        maps.alphabet = String::from("ETAOINSHRDLCUMWFGYPBVKJXQZ");
        LevenshteinCode {
            mode: IOMode::Integer,
            integer_code: codes,
            maps,
            spaced: false,
        }
    }
}

impl Code for LevenshteinCode {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        let mut output = String::new();
        match self.mode {
            IOMode::Letter => {
                for c in text.chars() {
                    let code = self.maps.char_to_int(c)? as u32;
                    output.push_str(&self.integer_code.encode_u32(code));
                    if self.spaced {
                        output.push(' ');
                    }
                }
            }
            IOMode::Word => {
                for w in text.split(" ") {
                    let code = self.maps.word_to_int(w)? as u32;
                    output.push_str(&self.integer_code.encode_u32(code));
                    if self.spaced {
                        output.push(' ');
                    }
                }
            }
            IOMode::Integer => {
                for s in text.split(" ") {
                    let n = u32::from_str_radix(s, 10)
                        .map_err(|_| CodeError::invalid_input_group(s))?;
                    output.push_str(&self.integer_code.encode_u32(n));
                    if self.spaced {
                        output.push(' ');
                    }
                }
            }
        }
        if self.spaced {
            output.pop();
        }
        Ok(output)
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        let text = &text.replace(" ", "");
        let mut output = String::new();

        for n in self.integer_code.decode_to_u32(text).into_iter() {
            match self.mode {
                IOMode::Letter => {
                    if let Some(val) = n {
                        match self.maps.alphabet.chars().nth(val as usize) {
                            Some(w) => output.push(w),
                            None => output.push('�'),
                        }
                    } else {
                        output.push('�')
                    }
                }
                IOMode::Word => {
                    if let Some(val) = n {
                        match self.maps.words.get(val as usize) {
                            Some(w) => output.push_str(w),
                            None => output.push('�'),
                        }
                    } else {
                        output.push('�')
                    }
                }
                IOMode::Integer => {
                    if let Some(val) = n {
                        output.push_str(&val.to_string())
                    } else {
                        output.push('�')
                    }
                }
            }
        }

        Ok(output)
    }
}

#[cfg(test)]
mod levenshtein_int_tests {
    use super::*;

    const PLAINTEXT: &'static str = "ETAO";
    const PLAINTEXT_INT: &'static str = "0 1 2 3";
    const ENCODEDTEXT: &'static str = "01011001101";
    const ENCODEDTEXT_SPACED: &'static str = "0 10 1100 1101";

    #[test]
    fn encode_test() {
        let mut code = LevenshteinCode::default();

        assert_eq!(code.encode(PLAINTEXT_INT).unwrap(), ENCODEDTEXT);
        code.mode = IOMode::Letter;
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT);
        code.spaced = true;
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT_SPACED);
    }

    #[test]
    fn decode_test() {
        let mut code = LevenshteinCode::default();
        code.mode = IOMode::Letter;
        assert_eq!(code.decode(ENCODEDTEXT).unwrap(), PLAINTEXT);
        assert_eq!(code.decode(ENCODEDTEXT_SPACED).unwrap(), PLAINTEXT);
    }
}
