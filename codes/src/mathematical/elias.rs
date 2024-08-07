use super::elias_integers::EliasCodeIntegers;
use crate::{
    errors::CodeError,
    letter_word_code::{IOMode, IntegerCodeMaps},
    traits::Code,
};
use itertools::Itertools;
use std::cell::RefCell;

pub struct EliasCode {
    pub maps: IntegerCodeMaps,
    pub integer_code: RefCell<EliasCodeIntegers>,
    pub mode: IOMode,
    pub spaced: bool,
}

impl EliasCode {
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
                .gamma_cache
                .values()
                .cloned()
                .collect_vec(),
            super::elias_integers::EliasVariant::Omega => self
                .integer_code
                .borrow()
                .omega_cache
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

        let mut maps = IntegerCodeMaps::new();
        maps.alphabet = String::from("ETAOINSHRDLCUMWFGYPBVKJXQZ");

        Self {
            mode: IOMode::Integer,
            integer_code: RefCell::new(codes),
            maps,
            spaced: false,
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
                if self.spaced {
                    out.push(' ');
                }
            }
        } else if self.mode == IOMode::Letter {
            for c in text.chars() {
                let n = self.maps.char_to_int(c)?;
                self.integer_code.borrow_mut().extend_all(n as u32);
                out.push_str(
                    self.integer_code
                        .borrow()
                        .encode_u32((n + 1) as u32)
                        .unwrap(),
                );
                if self.spaced {
                    out.push(' ');
                }
            }
        } else {
            for w in text.split(" ") {
                let n = self.maps.word_to_int(w)?;
                self.integer_code.borrow_mut().extend_all(n as u32);
                out.push_str(
                    self.integer_code
                        .borrow()
                        .encode_u32((n + 1) as u32)
                        .unwrap(),
                );
                if self.spaced {
                    out.push(' ');
                }
            }
        }
        if self.spaced {
            out.pop();
        }
        Ok(out)
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        let mut out = String::new();
        let text = text.replace(" ", "");
        let nums = self.integer_code.borrow().decode_u32(&text)?;

        if self.mode == IOMode::Integer {
            Ok(nums.into_iter().join(" "))
        } else if self.mode == IOMode::Letter {
            for n in nums {
                out.push(self.maps.int_to_char((n - 1) as usize)?);
            }
            Ok(out)
        } else {
            for n in nums {
                out.push_str(self.maps.int_to_word((n - 1) as usize)?);
                out.push(' ');
            }
            out.pop();
            Ok(out)
        }
    }
}

// impl_code_for_integer_code!(EliasCode);

#[cfg(test)]
mod elias_tests {
    use crate::mathematical::elias_integers::EliasVariant;

    use super::*;

    const PLAINTEXT: &'static str = "ETA";
    const PLAINTEXT_INT: &'static str = "1 2 3";
    const ENCODEDTEXT_DELTA: &'static str = "101000101";
    const ENCODEDTEXT_GAMMA: &'static str = "1010011";
    const ENCODEDTEXT_OMEGA: &'static str = "0100110";
    const ENCODEDTEXT_OMEGA_SPACED: &'static str = "0 100 110";

    #[test]
    fn encode_test() {
        let mut code = EliasCode::default();
        code.mode = IOMode::Letter;
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT_DELTA);
        code.integer_code.borrow_mut().variant = EliasVariant::Gamma;
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT_GAMMA);
        code.integer_code.borrow_mut().variant = EliasVariant::Omega;
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT_OMEGA);
        code.spaced = true;
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT_OMEGA_SPACED);
    }

    #[test]
    fn encode_test_int() {
        let mut code = EliasCode::default();
        code.mode = IOMode::Integer;
        assert_eq!(code.encode(PLAINTEXT_INT).unwrap(), ENCODEDTEXT_DELTA);
        code.integer_code.borrow_mut().variant = EliasVariant::Gamma;
        assert_eq!(code.encode(PLAINTEXT_INT).unwrap(), ENCODEDTEXT_GAMMA);
        code.integer_code.borrow_mut().variant = EliasVariant::Omega;
        assert_eq!(code.encode(PLAINTEXT_INT).unwrap(), ENCODEDTEXT_OMEGA);
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
        assert_eq!(code.decode(ENCODEDTEXT_OMEGA_SPACED).unwrap(), PLAINTEXT);
    }

    #[test]
    fn decode_test_int() {
        let mut code = EliasCode::default();
        code.mode = IOMode::Integer;
        assert_eq!(code.decode(ENCODEDTEXT_DELTA).unwrap(), PLAINTEXT_INT);
        code.integer_code.borrow_mut().variant = EliasVariant::Gamma;
        assert_eq!(code.decode(ENCODEDTEXT_GAMMA).unwrap(), PLAINTEXT_INT);
        code.integer_code.borrow_mut().variant = EliasVariant::Omega;
        assert_eq!(code.decode(ENCODEDTEXT_OMEGA).unwrap(), PLAINTEXT_INT);
    }
}
