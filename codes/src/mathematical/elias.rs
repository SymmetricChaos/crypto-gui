use super::elias_integers::EliasCodeIntegers;
use crate::{
    errors::CodeError,
    traits::{Code, IOMode, LetterAndWordCode},
};
use itertools::Itertools;
use std::cell::RefCell;

pub struct EliasCode {
    pub maps: LetterAndWordCode<u32>,
    pub integer_code: RefCell<EliasCodeIntegers>,
    pub mode: IOMode,
}

impl EliasCode {
    pub fn set_letter_map(&mut self) {
        self.maps.set_letter_map(|(n, _)| (n + 1) as u32)
    }

    pub fn set_word_map(&mut self) {
        self.maps.set_word_map(|(n, _)| (n + 1) as u32)
    }

    pub fn values(&self) -> Vec<String> {
        match self.integer_code.borrow().variant {
            super::elias_integers::EliasVariant::Delta => self
                .integer_code
                .borrow()
                .delta_cache
                .values()
                .cloned()
                .collect_vec(),
            super::elias_integers::EliasVariant::Gamma => self
                .integer_code
                .borrow()
                .delta_cache
                .values()
                .cloned()
                .collect_vec(),
            super::elias_integers::EliasVariant::Omega => self
                .integer_code
                .borrow()
                .delta_cache
                .values()
                .cloned()
                .collect_vec(),
        }
    }
}

impl Default for EliasCode {
    fn default() -> Self {
        let mut codes = EliasCodeIntegers::default();
        codes.extend_all(33);

        let mut maps = LetterAndWordCode::<u32>::default();
        maps.alphabet = String::from("ETAOINSHRDLCUMWFGYPBVKJXQZ");
        maps.set_letter_map(|(n, _)| (n + 1) as u32);

        Self {
            mode: IOMode::Integer,
            integer_code: RefCell::new(codes),
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
                self.integer_code.borrow_mut().extend_all(n);
                out.push_str(self.integer_code.borrow().encode_u32(n).unwrap());
            }
        } else if self.mode == IOMode::Letter {
            for c in text.chars() {
                let n = self.maps.get_by_letter(c)?;
                self.integer_code.borrow_mut().extend_all(*n);
                out.push_str(self.integer_code.borrow().encode_u32(*n).unwrap());
            }
        } else {
            for w in text.split(" ") {
                let n = self.maps.get_by_word(w)?;
                self.integer_code.borrow_mut().extend_all(*n);
                out.push_str(self.integer_code.borrow().encode_u32(*n).unwrap());
            }
        }
        Ok(out)
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        let mut out = String::new();
        let nums = self.integer_code.borrow().decode_to_u32(text)?;

        if self.mode == IOMode::Integer {
            Ok(nums.into_iter().join(" "))
        } else if self.mode == IOMode::Letter {
            for n in nums {
                out.push(*self.maps.get_letter_by_code(&n)?);
            }
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
    use crate::mathematical::elias_integers::EliasVariant;

    use super::*;

    const PLAINTEXT: &'static str = "ETA";
    const ENCODEDTEXT_DELTA: &'static str = "101000101";
    const ENCODEDTEXT_GAMMA: &'static str = "1010011";
    const ENCODEDTEXT_OMEGA: &'static str = "0100110";

    #[test]
    fn encode_test() {
        let mut code = EliasCode::default();
        code.mode = IOMode::Letter;
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT_DELTA);
        code.integer_code.borrow_mut().variant = EliasVariant::Gamma;
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT_GAMMA);
        code.integer_code.borrow_mut().variant = EliasVariant::Omega;
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT_OMEGA);
    }

    #[test]
    fn decode_test() {
        let mut code = EliasCode::default();
        code.mode = IOMode::Letter;
        assert_eq!(code.decode(ENCODEDTEXT_DELTA).unwrap(), PLAINTEXT);
        code.integer_code.borrow_mut().variant = EliasVariant::Gamma;
        assert_eq!(code.decode(ENCODEDTEXT_GAMMA).unwrap(), PLAINTEXT);
        code.integer_code.borrow_mut().variant = EliasVariant::Omega;
        assert_eq!(code.decode(ENCODEDTEXT_OMEGA).unwrap(), PLAINTEXT);
    }
}
