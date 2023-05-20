use lazy_static::lazy_static;
use std::collections::HashMap;

use crate::{errors::CodeError, traits::Code};

lazy_static! {
    pub static ref VERHOEFF_INV_TABLE: HashMap<char, char> = HashMap::from_iter(
        [
            ('0', '0'),
            ('1', '4'),
            ('2', '3'),
            ('3', '2'),
            ('4', '1'),
            ('5', '5'),
            ('6', '6'),
            ('7', '7'),
            ('8', '8'),
            ('9', '9')
        ]
        .into_iter()
    );
    pub static ref VERHOEFF_MUL_TABLE: HashMap<(char, char), char> = HashMap::from_iter(
        [
            (('0', '0'), '0'),
            (('0', '1'), '1'),
            (('0', '2'), '2'),
            (('0', '3'), '3'),
            (('0', '4'), '4'),
            (('0', '5'), '5'),
            (('0', '6'), '6'),
            (('0', '7'), '7'),
            (('0', '8'), '8'),
            (('0', '9'), '9'),
        ]
        .into_iter()
    );
    pub static ref VERHOEFF_PERM_TABLE: HashMap<(usize, char), char> =
        HashMap::from_iter([((0, '0'), '0'), ((0, '1'), '1')]);
}

pub struct VerhoeffAlgorithm {}

impl VerhoeffAlgorithm {
    fn mul(a: char, b: char) -> Result<&'static char, CodeError> {
        VERHOEFF_MUL_TABLE
            .get(&(a, b))
            .ok_or(CodeError::Input(format!(
                "invalid mul input pair ({},{})",
                a, b
            )))
    }

    fn perm(n: usize, a: char) -> Result<&'static char, CodeError> {
        VERHOEFF_PERM_TABLE
            .get(&(n, a))
            .ok_or(CodeError::invalid_input_char(a))
    }

    fn inv(a: &char) -> Result<&'static char, CodeError> {
        VERHOEFF_INV_TABLE
            .get(a)
            .ok_or(CodeError::invalid_input_char(*a))
    }
}

impl Default for VerhoeffAlgorithm {
    fn default() -> Self {
        Self {}
    }
}

impl Code for VerhoeffAlgorithm {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        if text.is_empty() {
            return Err(CodeError::input("input cannot be empty"));
        }
        let mut check = '0';
        for (i, c) in text.chars().rev().chain(std::iter::once('0')).enumerate() {
            check = *Self::mul(check, *Self::perm(i % 8, c)?)?;
            //println!("{i} {c} {check}")
        }
        let mut out = text.to_string();
        out.push(*Self::inv(&check)?);
        Ok(out)
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        if text.is_empty() {
            return Err(CodeError::input("input cannot be empty"));
        }
        let mut check = '0';
        for (i, c) in text.chars().rev().chain(std::iter::once('0')).enumerate() {
            check = *Self::mul(check, *Self::perm(i % 8, c)?)?;
            //println!("{i} {c} {check}")
        }
        if check != '0' {
            return Err(CodeError::input("invalid check digit"));
        } else {
            Ok(text[0..text.len() - 1].to_string())
        }
    }

    fn randomize(&mut self) {}

    fn reset(&mut self) {}
}

#[cfg(test)]
mod luhn_tests {
    use super::*;

    #[test]
    fn test_encode() {
        let code = VerhoeffAlgorithm::default();
        assert_eq!(code.encode("7992739871").unwrap(), "79927398713");
    }

    #[test]
    fn test_decode() {
        let code = VerhoeffAlgorithm::default();
        assert_eq!(code.decode("79927398713").unwrap(), "7992739871");
    }

    #[test]
    fn test_decode_with_err() {
        let code = VerhoeffAlgorithm::default();
        assert_eq!(
            code.decode("79297398713").unwrap_err(),
            CodeError::input("check digit does not match")
        );
    }
}
