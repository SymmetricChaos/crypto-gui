use super::elias_integers::EliasCodeIntegers;
use crate::{
    errors::CodeError,
    traits::{Code, IOMode, LetterAndWordCode},
};
use itertools::Itertools;

pub struct EliasCode {
    pub maps: LetterAndWordCode<u32>,
    pub integer_code: EliasCodeIntegers,
    pub mode: IOMode,
}

impl EliasCode {}

impl Default for EliasCode {
    fn default() -> Self {
        let codes = EliasCodeIntegers::default();

        let mut maps = LetterAndWordCode::<u32>::default();
        maps.alphabet = String::from("ETAOINSHRDLCUMWFGYPBVKJXQZ");
        maps.set_letter_map(|(n, _)| (n + 1) as u32);

        Self {
            mode: IOMode::Integer,
            integer_code: codes,
            maps,
        }
    }
}

impl Code for EliasCode {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        let mut out = String::new();
        if self.mode == IOMode::Integer {
            for group in text.split(" ") {
                let n = u32::from_str_radix(group, 10)
                    .map_err(|_| CodeError::invalid_input_group(group))?;
                out.push_str(&self.integer_code.encode_u32(n));
            }
        } else if self.mode == IOMode::Letter {
            for c in text.chars() {
                let n = self.maps.get_by_letter(c)?;
                out.push_str(&self.integer_code.encode_u32(*n));
            }
        } else {
            for w in text.split(" ") {
                let n = self.maps.get_by_word(w)?;
                out.push_str(&self.integer_code.encode_u32(*n));
            }
        }
        Ok(out)
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        let mut out = String::new();
        let nums = self.integer_code.decode_to_u32(text)?;

        if self.mode == IOMode::Integer {
            Ok(nums.into_iter().join(" "))
        } else if self.mode == IOMode::Letter {
            for n in nums {
                out.push(*self.maps.get_letter_by_code(&n)?);
                out.push(' ');
            }
            out.pop();
            Ok(out)
        } else {
            for n in nums {
                out.push_str(self.maps.get_word_by_code(&n)?);
                out.push(' ');
            }
            out.pop();
            Ok(out)
        }
    }
}

#[cfg(test)]
mod elias_tests {
    use super::*;

    const PLAINTEXT: &'static str = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    const ENCODEDTEXT: &'static str = "";

    #[test]
    fn encrypt_test() {
        let code = EliasCode::default();
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT);
    }

    #[test]
    fn decrypt_test() {
        let code = EliasCode::default();
        assert_eq!(code.decode(ENCODEDTEXT).unwrap(), PLAINTEXT);
    }
}
