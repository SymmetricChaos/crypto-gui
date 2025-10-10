use super::string_to_i32s;
use crate::traits::Code;
use itertools::Itertools;
use num::{Integer, Zero};
use utils::{errors::GeneralError, text_functions::num_to_digit};

pub struct NegativeBaseN {
    pub radix: i32,
    pub little_endian: bool,
}

impl Default for NegativeBaseN {
    fn default() -> Self {
        Self {
            radix: -2,
            little_endian: true,
        }
    }
}

impl NegativeBaseN {
    pub fn validate(&self) -> Result<(), GeneralError> {
        if self.radix > -2 || self.radix < -36 {
            return Err(GeneralError::state(
                "radix must be between -2 and -36, inclusive",
            ));
        }
        Ok(())
    }

    pub fn encode_i32(&self, n: i32) -> Result<String, GeneralError> {
        if n.is_zero() {
            return Ok(String::from("0"));
        }
        let mut n = n;
        let mut s = Vec::new();
        while n != 0 {
            let (q, r) = n.div_rem(&self.radix);
            if r < 0 {
                s.push(
                    num_to_digit((r - self.radix) as u32)
                        .expect("remainder should always be less than 36"),
                );
                n = q + 1;
            } else {
                s.push(num_to_digit(r as u32).expect("remainder should always be less than 36"));
                n = q;
            }
        }
        if self.little_endian {
            Ok(s.iter().rev().collect())
        } else {
            Ok(s.iter().collect())
        }
    }

    pub fn decode_to_i32(&self, s: &str) -> Result<i32, GeneralError> {
        let mut out = 0;
        let mut base = 1;
        if self.little_endian {
            for c in s.chars().rev() {
                if let Some(n) = c.to_digit(-self.radix as u32) {
                    out += (n as i32) * base;
                } else {
                    return Err(GeneralError::invalid_input_char(c));
                }
                base *= self.radix;
            }
        } else {
            for c in s.chars() {
                if let Some(n) = c.to_digit(-self.radix as u32) {
                    out += (n as i32) * base;
                } else {
                    return Err(GeneralError::invalid_input_char(c));
                }
                base *= self.radix;
            }
        };

        Ok(out)
    }
}

impl Code for NegativeBaseN {
    fn encode(&self, text: &str) -> Result<String, GeneralError> {
        self.validate()?;
        let mut output = Vec::new();

        for n in string_to_i32s(text, ",")? {
            output.push(self.encode_i32(n)?);
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
            output.push(self.decode_to_i32(s)?.to_string())
        }

        Ok(output.into_iter().join(", "))
    }
}

#[cfg(test)]
mod negative_base_n_tests {
    use super::*;

    const PTEXT: &'static str = "-5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5";
    const ENCODEDTEXT: &'static str = "1111, 1100, 1101, 10, 11, 0, 1, 110, 111, 100, 101";

    #[test]
    fn encode_test() {
        let code = NegativeBaseN::default();
        assert_eq!(code.encode(PTEXT).unwrap(), ENCODEDTEXT);
    }

    #[test]
    fn decode_test() {
        let code = NegativeBaseN::default();
        assert_eq!(code.decode(ENCODEDTEXT).unwrap(), PTEXT);
    }
}
