use crate::{
    errors::CodeError,
    letter_word_code::{IOMode, LetterWordIntCode},
    traits::Code,
};
use itertools::Itertools;
use num::{Integer, Zero};
use utils::text_functions::num_to_digit;

pub struct BaseN {
    pub maps: LetterWordIntCode,
    pub radix: u32,
    pub mode: IOMode,
    pub bijective: bool,
    pub little_endian: bool,
}

impl Default for BaseN {
    fn default() -> Self {
        let mut maps = LetterWordIntCode::new();
        maps.alphabet = String::from("ETAOINSHRDLCUMWFGYPBVKJXQZ");

        Self {
            mode: IOMode::Integer,
            maps,
            radix: 2,
            bijective: false,
            little_endian: true,
        }
    }
}

impl BaseN {
    pub fn validate(&self) -> Result<(), CodeError> {
        if self.bijective {
            if self.radix < 1 || self.radix > 35 {
                return Err(CodeError::state(
                    "bijective radix must be between 1 and 35, inclusive",
                ));
            }
        }
        if self.radix < 2 || self.radix > 36 {
            return Err(CodeError::state(
                "radix must be between 2 and 36, inclusive",
            ));
        }
        Ok(())
    }

    pub fn encode_bijective_u32(&self, n: u32) -> Result<String, CodeError> {
        if n == 0 {
            return Err(CodeError::input(
                "in bijective representation 0 is the empty string and cannot be represented",
            ));
        };

        if self.radix == 1 {
            return Ok("1".repeat(n as usize));
        }

        let mut out = Vec::with_capacity(32);
        let mut n = n;
        loop {
            let q = num::integer::div_ceil(n, self.radix) - 1;
            let a = n - q * self.radix;
            out.push(num_to_digit(a).expect("remainder should always be less than 35"));
            n = q;
            if n == 0 {
                break;
            }
        }
        if self.little_endian {
            Ok(out.iter().rev().collect())
        } else {
            Ok(out.iter().collect())
        }
    }

    pub fn encode_regular_u32(&self, n: u32) -> String {
        if n.is_zero() {
            return String::from("0");
        }
        let mut n = n;
        let mut s = Vec::new();
        while n != 0 {
            let (q, r) = n.div_rem(&self.radix);
            s.push(num_to_digit(r).expect("remainder should always be less than 36"));

            n = q;
        }
        if self.little_endian {
            s.iter().rev().collect()
        } else {
            s.iter().collect()
        }
    }

    pub fn encode_u32(&self, n: u32) -> Result<String, CodeError> {
        match self.bijective {
            true => self.encode_bijective_u32(n),
            false => Ok(self.encode_regular_u32(n)),
        }
    }

    pub fn decode_to_u32(&self, s: &str) -> Result<u32, CodeError> {
        let word: String = if self.little_endian {
            s.chars().collect()
        } else {
            s.chars().rev().collect()
        };
        if self.bijective {
            let mut base = 1;
            let mut out = 0;
            for c in word.chars().rev() {
                let n = c
                    .to_digit(36)
                    .ok_or_else(|| CodeError::invalid_input_char(c))?;
                out += base * n;
                base *= self.radix;
            }
            Ok(out)
        } else {
            u32::from_str_radix(&word, self.radix).map_err(|e| CodeError::Input(e.to_string()))
        }
    }
}

impl Code for BaseN {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        self.validate()?;
        let mut output = Vec::new();
        if self.mode == IOMode::Integer {
            for group in text.split(" ") {
                if group.is_empty() {
                    continue;
                }
                let n = u32::from_str_radix(group, 10)
                    .map_err(|_| CodeError::invalid_input_group(group))?;
                output.push(self.encode_u32(n)?);
            }
        } else if self.mode == IOMode::Letter {
            for c in text.chars() {
                let n = self.maps.char_to_int(c)?;
                if self.bijective {
                    output.push(self.encode_u32((n + 1) as u32)?);
                } else {
                    output.push(self.encode_u32(n as u32)?);
                }
            }
        } else {
            for w in text.split(" ") {
                if w.is_empty() {
                    continue;
                }
                let n = self.maps.word_to_int(w)?;
                if self.bijective {
                    output.push(self.encode_u32((n + 1) as u32)?);
                } else {
                    output.push(self.encode_u32(n as u32)?);
                }
            }
        }
        Ok(output.into_iter().join(" "))
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        self.validate()?;
        let mut output = String::new();
        if self.mode == IOMode::Integer {
            for s in text.split(" ") {
                if s.is_empty() {
                    continue;
                }
                output.push_str(&format!("{} ", self.decode_to_u32(s)?))
            }
            output.pop();
        } else if self.mode == IOMode::Letter {
            for s in text.split(" ") {
                if s.is_empty() {
                    continue;
                }
                let n = self.decode_to_u32(s)?;
                if self.bijective {
                    output.push(self.maps.int_to_char((n - 1) as usize)?)
                } else {
                    output.push(self.maps.int_to_char(n as usize)?)
                }
            }
        } else {
            for s in text.split(" ") {
                if s.is_empty() {
                    continue;
                }
                let n = self.decode_to_u32(s)?;
                if self.bijective {
                    output.push_str(self.maps.int_to_word((n - 1) as usize)?)
                } else {
                    output.push_str(self.maps.int_to_word(n as usize)?)
                }
                output.push(' ');
            }
            output.pop();
        }

        Ok(output)
    }
}

#[cfg(test)]
mod base_n_tests {
    use super::*;

    const PLAINTEXT_INT: &'static str = "0 1 2 3 4 5";
    const PLAINTEXT_LET: &'static str = "ETAOIN";
    const ENCODEDTEXT: &'static str = "0 1 10 11 100 101";

    const PLAINTEXT_INT_BE: &'static str = "0 1 2 3 4 5";
    const PLAINTEXT_LET_BE: &'static str = "ETAOIN";
    const ENCODEDTEXT_BE: &'static str = "0 1 01 11 001 101";

    const PLAINTEXT_INT_BIJ: &'static str = "1 2 3 4 5 6";
    const PLAINTEXT_LET_BIJ: &'static str = "ETAOIN";
    const ENCODEDTEXT_BIJ: &'static str = "1 2 11 12 21 22";

    #[test]
    fn encode_test() {
        let mut code = BaseN::default();
        assert_eq!(code.encode(PLAINTEXT_INT).unwrap(), ENCODEDTEXT);
        code.mode = IOMode::Letter;
        assert_eq!(code.encode(PLAINTEXT_LET).unwrap(), ENCODEDTEXT);
    }

    #[test]
    fn encode_test_be() {
        let mut code = BaseN::default();
        code.little_endian = false;
        assert_eq!(code.encode(PLAINTEXT_INT_BE).unwrap(), ENCODEDTEXT_BE);
        code.mode = IOMode::Letter;
        assert_eq!(code.encode(PLAINTEXT_LET_BE).unwrap(), ENCODEDTEXT_BE);
    }

    #[test]
    fn encode_test_bijective() {
        let mut code = BaseN::default();
        code.bijective = true;
        assert_eq!(code.encode(PLAINTEXT_INT_BIJ).unwrap(), ENCODEDTEXT_BIJ);
        code.mode = IOMode::Letter;
        assert_eq!(code.encode(PLAINTEXT_LET_BIJ).unwrap(), ENCODEDTEXT_BIJ);
    }

    #[test]
    fn decode_test() {
        let mut code = BaseN::default();
        assert_eq!(code.decode(ENCODEDTEXT).unwrap(), PLAINTEXT_INT);
        code.mode = IOMode::Letter;
        assert_eq!(code.decode(ENCODEDTEXT).unwrap(), PLAINTEXT_LET);
    }

    #[test]
    fn decode_test_be() {
        let mut code = BaseN::default();
        code.little_endian = false;
        assert_eq!(code.decode(ENCODEDTEXT_BE).unwrap(), PLAINTEXT_INT_BE);
        code.mode = IOMode::Letter;
        assert_eq!(code.decode(ENCODEDTEXT_BE).unwrap(), PLAINTEXT_LET_BE);
    }

    #[test]
    fn decode_test_bijective() {
        let mut code = BaseN::default();
        code.bijective = true;
        assert_eq!(code.decode(ENCODEDTEXT_BIJ).unwrap(), PLAINTEXT_INT_BIJ);
        code.mode = IOMode::Letter;
        assert_eq!(code.decode(ENCODEDTEXT_BIJ).unwrap(), PLAINTEXT_LET_BIJ);
    }
}
