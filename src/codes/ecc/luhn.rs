use num::Integer;

use crate::{codes::Code, errors::Error};

pub struct LuhnAlgorithm {
    pub modulus: u32,
}

impl LuhnAlgorithm {}

impl Default for LuhnAlgorithm {
    fn default() -> Self {
        Self { modulus: 10 }
    }
}

fn digital_sum(n: u32, m: u32) -> u32 {
    let mut t = n;
    let mut s = 0;
    while t != 0 {
        let (q, r) = t.div_rem(&m);
        s += r;
        t = q
    }
    s
}

impl Code for LuhnAlgorithm {
    fn encode(&self, text: &str) -> Result<String, Error> {
        if self.modulus % 2 != 0 {
            return Err(Error::state("modulus must be even"));
        }

        if self.modulus < 2 || self.modulus > 36 {
            return Err(Error::state("modulus must be between 2 and 36, inclusive"));
        }

        let mut check = 0;
        for (p, c) in text.chars().rev().enumerate() {
            let n = c
                .to_digit(self.modulus)
                .ok_or(Error::invalid_input_char(c))?;
            if p % 2 == 0 {
                check += digital_sum(n * 2, self.modulus);
            } else {
                check += n;
            }
        }
        let digit = char::from_u32((self.modulus - (check % self.modulus)) + 48).unwrap();

        let mut out = String::with_capacity(text.len() + 1);
        out.push_str(text);
        out.push(digit);
        Ok(out)
    }

    fn decode(&self, text: &str) -> Result<String, Error> {
        if text.is_empty() {
            return Err(Error::input("input cannot be empty"));
        }

        if self.modulus % 2 != 0 {
            return Err(Error::state("modulus must be even"));
        }

        if self.modulus < 2 || self.modulus > 36 {
            return Err(Error::state("modulus must be between 2 and 36, inclusive"));
        }

        let stored_check_num = text
            .chars()
            .last()
            .unwrap()
            .to_digit(10)
            .ok_or(Error::input("check digit is not a valid digit"))?;

        let mut check = 0;
        for (p, c) in text.chars().rev().skip(1).enumerate() {
            let n = c
                .to_digit(self.modulus)
                .ok_or(Error::invalid_input_char(c))?;
            if p % 2 == 0 {
                check += digital_sum(n * 2, self.modulus);
            } else {
                check += n;
            }
        }

        if stored_check_num == (self.modulus - (check % self.modulus)) {
            Ok(text[0..text.len() - 1].to_string())
        } else {
            Err(Error::input("check digit does not match"))
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
        let code = LuhnAlgorithm::default();
        assert_eq!(code.encode("7992739871").unwrap(), "79927398713");
    }

    #[test]
    fn test_decode() {
        let code = LuhnAlgorithm::default();
        assert_eq!(code.decode("79927398713").unwrap(), "7992739871");
    }

    #[test]
    fn test_decode_with_err() {
        let code = LuhnAlgorithm::default();
        assert_eq!(
            code.decode("79297398713").unwrap_err(),
            Error::input("check digit does not match")
        );
    }
}
