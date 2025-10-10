use super::string_to_u32s;
use crate::traits::Code;
use itertools::Itertools;
use num::{Integer, Zero};
use utils::{errors::GeneralError, text_functions::num_to_digit};

pub struct BaseN {
    pub radix: u32,
    pub little_endian: bool,
}

impl Default for BaseN {
    fn default() -> Self {
        Self {
            radix: 2,
            little_endian: true,
        }
    }
}

impl BaseN {
    pub fn validate(&self) -> Result<(), GeneralError> {
        if self.radix < 2 || self.radix > 36 {
            return Err(GeneralError::state(
                "radix must be between 2 and 36, inclusive",
            ));
        }
        Ok(())
    }

    pub fn encode_u32(&self, n: u32) -> Result<String, GeneralError> {
        if n.is_zero() {
            return Ok(String::from("0"));
        }
        let mut n = n;
        let mut s = Vec::new();
        while n != 0 {
            let (q, r) = n.div_rem(&self.radix);
            s.push(num_to_digit(r).expect("remainder should always be less than 36"));

            n = q;
        }
        if self.little_endian {
            Ok(s.iter().rev().collect())
        } else {
            Ok(s.iter().collect())
        }
    }

    pub fn decode_to_u32(&self, s: &str) -> Result<u32, GeneralError> {
        let word: String = if self.little_endian {
            s.chars().collect()
        } else {
            s.chars().rev().collect()
        };

        u32::from_str_radix(&word, self.radix).map_err(|e| GeneralError::input(e.to_string()))
    }
}

impl Code for BaseN {
    fn encode(&self, text: &str) -> Result<String, GeneralError> {
        self.validate()?;
        let mut output = Vec::new();

        for n in string_to_u32s(text, ",")? {
            output.push(self.encode_u32(n)?);
        }

        Ok(output.into_iter().join(", "))
    }

    fn decode(&self, text: &str) -> Result<String, GeneralError> {
        self.validate()?;
        let mut output = Vec::new();

        for s in text.split(",").map(|s| s.trim()) {
            if s.is_empty() {
                continue;
            }
            output.push(self.decode_to_u32(s)?.to_string())
        }
        Ok(output.into_iter().join(", "))
    }
}

#[cfg(test)]
mod base_n_tests {
    use super::*;

    const PTEXT_INT: &'static str = "0, 1, 2, 3, 4, 5";
    const ENCODEDTEXT: &'static str = "0, 1, 10, 11, 100, 101";

    const PTEXT_INT_BE: &'static str = "0, 1, 2, 3, 4, 5";
    const ENCODEDTEXT_BE: &'static str = "0, 1, 01, 11, 001, 101";

    #[test]
    fn encode_test() {
        let code = BaseN::default();
        assert_eq!(code.encode(PTEXT_INT).unwrap(), ENCODEDTEXT);
    }

    #[test]
    fn encode_test_be() {
        let mut code = BaseN::default();
        code.little_endian = false;
        assert_eq!(code.encode(PTEXT_INT_BE).unwrap(), ENCODEDTEXT_BE);
    }

    #[test]
    fn decode_test() {
        let code = BaseN::default();
        assert_eq!(code.decode(ENCODEDTEXT).unwrap(), PTEXT_INT);
    }

    #[test]
    fn decode_test_be() {
        let mut code = BaseN::default();
        code.little_endian = false;
        assert_eq!(code.decode(ENCODEDTEXT_BE).unwrap(), PTEXT_INT_BE);
    }
}
