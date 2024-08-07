use crate::{
    errors::CodeError,
    letter_word_code::{IOMode, IntegerCodeMaps},
    traits::Code,
};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

const ROMAN_PV: [[&'static str; 10]; 3] = [
    ["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"],
    ["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"],
    ["", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"],
];

lazy_static! {
    // pub static ref STRICT_ROMAN: Regex =
    //     Regex::new(r"^M{0,3}(CM|CD|D?C{0,3})(XC|XL|L?X{0,3})(IX|IV|V?I{0,3})$").unwrap();
    pub static ref RELAXED_ROMAN: Regex = Regex::new(r"^[IVXLCDM]+$").unwrap();
}

pub struct RomanNumeral {
    pub maps: IntegerCodeMaps,
    pub mode: IOMode,
    // pub apostrophus: bool,
}

impl Default for RomanNumeral {
    fn default() -> Self {
        let mut maps = IntegerCodeMaps::new();
        maps.alphabet = String::from("ETAOINSHRDLCUMWFGYPBVKJXQZ");

        Self {
            maps,
            mode: IOMode::Integer,
        }
    }
}

impl RomanNumeral {
    fn char_to_num(c: char) -> Result<i32, CodeError> {
        Ok(match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => return Err(CodeError::invalid_input_char(c)),
        })
    }

    pub fn encode_int(n: usize) -> Result<String, CodeError> {
        if n > 3999 {
            return Err(CodeError::input(
                "standard Roman Numerals cannot be greater than 3999",
            ));
        }
        if n == 0 {
            return Err(CodeError::input(
                "there is no standard Roman Numeral representation for 0",
            ));
        }
        let mut out = "M".repeat(n / 1000);
        out.push_str(ROMAN_PV[2][(n % 1000) / 100]);
        out.push_str(ROMAN_PV[1][(n % 100) / 10]);
        out.push_str(ROMAN_PV[0][n % 10]);
        Ok(out)
    }

    fn decode_to_int(numeral: &str) -> Result<usize, CodeError> {
        if !RELAXED_ROMAN.is_match(numeral) {
            return Err(CodeError::Input(format!(
                "the Roman Numeral `{}` contains invalid characters and cannot be decoded",
                numeral
            )));
        }

        let mut n = 0;
        for (l, r) in numeral.chars().tuple_windows() {
            let a = Self::char_to_num(l)?;
            let b = Self::char_to_num(r)?;
            if a >= b {
                n += a
            } else {
                n -= a
            }
        }
        n += Self::char_to_num(numeral.chars().last().unwrap())?;
        if n <= 0 {
            return Err(CodeError::Input(format!(
                "the Roman Numeral `{}` evaluates to 0 or less which is not valid",
                numeral
            )));
        }
        Ok(n as usize)
    }
}

impl Code for RomanNumeral {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        let mut output = String::new();
        if self.mode == IOMode::Integer {
            for group in text.split(" ") {
                if group.is_empty() {
                    continue;
                }
                let n = usize::from_str_radix(group, 10)
                    .map_err(|_| CodeError::invalid_input_group(group))?;
                output.push_str(&Self::encode_int(n)?);
                output.push(' ')
            }
        } else if self.mode == IOMode::Letter {
            for c in text.chars() {
                let n = self.maps.char_to_int(c)?;
                output.push_str(&Self::encode_int(n + 1)?);
                output.push(' ')
            }
        } else {
            for w in text.split(" ") {
                if w.is_empty() {
                    continue;
                }
                let n = self.maps.word_to_int(w)?;
                output.push_str(&Self::encode_int(n + 1)?);
                output.push(' ')
            }
        }
        output.pop();
        Ok(output)
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        let mut output = String::new();
        if self.mode == IOMode::Integer {
            for s in text.split(" ") {
                if s.is_empty() {
                    continue;
                }
                output.push_str(&format!("{} ", Self::decode_to_int(s)?))
            }
            output.pop();
        } else if self.mode == IOMode::Letter {
            for s in text.split(" ") {
                if s.is_empty() {
                    continue;
                }
                let n = Self::decode_to_int(s)?;
                output.push(self.maps.int_to_char(n - 1)?);
            }
        } else {
            for s in text.split(" ") {
                if s.is_empty() {
                    continue;
                }
                let n = Self::decode_to_int(s)?;
                output.push_str(self.maps.int_to_word(n - 1)?);
                output.push(' ');
            }
            output.pop();
        }

        Ok(output)
    }
}

#[cfg(test)]
mod roman_numeral_tests {
    use super::*;

    const PLAINTEXT: &'static str = "39 246 789 2421 9";
    const PLAINTEXT_LTR: &'static str = "ETAOI";
    const ENCODEDTEXT: &'static str = "XXXIX CCXLVI DCCLXXXIX MMCDXXI IX";
    const ENCODEDTEXT_LTR: &'static str = "I II III IV V";

    #[test]
    fn encode_test() {
        let code = RomanNumeral::default();
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT);
    }

    #[test]
    fn decode_test() {
        let code = RomanNumeral::default();
        assert_eq!(code.decode(ENCODEDTEXT).unwrap(), PLAINTEXT);
    }

    #[test]
    fn encode_test_letter() {
        let mut code = RomanNumeral::default();
        code.mode = IOMode::Letter;
        assert_eq!(code.encode(PLAINTEXT_LTR).unwrap(), ENCODEDTEXT_LTR);
    }

    #[test]
    fn decode_test_letter() {
        let mut code = RomanNumeral::default();
        code.mode = IOMode::Letter;
        assert_eq!(code.decode(ENCODEDTEXT_LTR).unwrap(), PLAINTEXT_LTR);
    }
}
