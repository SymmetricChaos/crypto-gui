use super::{elias_integers::EliasCodeIntegers, string_to_u32s};
use crate::{errors::CodeError, traits::Code};
use itertools::Itertools;
use std::cell::RefCell;

pub struct EliasCode {
    pub integer_code: RefCell<EliasCodeIntegers>,
    pub spaced: bool,
    pub sep: String,
}

impl Default for EliasCode {
    fn default() -> Self {
        let mut codes = EliasCodeIntegers::default();
        codes.extend_all(33);

        Self {
            integer_code: RefCell::new(codes),
            spaced: false,
            sep: String::from(" "),
        }
    }
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

impl Code for EliasCode {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        let mut out = Vec::new();

        for n in string_to_u32s(text, &self.sep)? {
            self.integer_code.borrow_mut().extend_all(n);
            out.push(self.integer_code.borrow().encode_u32(n).unwrap().clone());
        }

        if self.spaced {
            Ok(out.into_iter().join(&self.sep))
        } else {
            Ok(out.into_iter().join(""))
        }
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        let text = text.replace(&self.sep, "");
        Ok(self
            .integer_code
            .borrow()
            .decode_u32(&text)?
            .into_iter()
            .join(&self.sep))
    }
}

#[cfg(test)]
mod elias_tests {
    use crate::mathematical::elias_integers::EliasVariant;

    use super::*;

    const PLAINTEXT_INT: &'static str = "1 2 3";
    const ENCODEDTEXT_DELTA: &'static str = "101000101";
    const ENCODEDTEXT_GAMMA: &'static str = "1010011";
    const ENCODEDTEXT_OMEGA: &'static str = "0100110";

    #[test]
    fn encode_test_int() {
        let code = EliasCode::default();
        assert_eq!(code.encode(PLAINTEXT_INT).unwrap(), ENCODEDTEXT_DELTA);
        code.integer_code.borrow_mut().variant = EliasVariant::Gamma;
        assert_eq!(code.encode(PLAINTEXT_INT).unwrap(), ENCODEDTEXT_GAMMA);
        code.integer_code.borrow_mut().variant = EliasVariant::Omega;
        assert_eq!(code.encode(PLAINTEXT_INT).unwrap(), ENCODEDTEXT_OMEGA);
    }

    #[test]
    fn decode_test_int() {
        let code = EliasCode::default();
        assert_eq!(code.decode(ENCODEDTEXT_DELTA).unwrap(), PLAINTEXT_INT);
        // code.integer_code.borrow_mut().variant = EliasVariant::Gamma;
        // assert_eq!(code.decode(ENCODEDTEXT_GAMMA).unwrap(), PLAINTEXT_INT);
        // code.integer_code.borrow_mut().variant = EliasVariant::Omega;
        // assert_eq!(code.decode(ENCODEDTEXT_OMEGA).unwrap(), PLAINTEXT_INT);
    }
}
