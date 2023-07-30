use itertools::Itertools;

use crate::{errors::CodeError, traits::Code};

const ROMAN_PV: [[&'static str; 10]; 3] = [
    ["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"],
    ["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"],
    ["", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"],
];

pub struct RomanNumeral {
    extended: bool,
}

impl Default for RomanNumeral {
    fn default() -> Self {
        Self { extended: false }
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
}

impl Code for RomanNumeral {
    fn encode(&self, text: &str) -> Result<String, crate::errors::CodeError> {
        let mut out = String::new();
        for number in text.split(" ").filter(|w| !w.is_empty()) {
            let n =
                usize::from_str_radix(number, 10).map_err(|e| CodeError::Input(e.to_string()))?;

            out.push_str(&"M".repeat(n / 1000));
            out.push_str(ROMAN_PV[2][(n % 1000) / 100]);
            out.push_str(ROMAN_PV[1][(n % 100) / 10]);
            out.push_str(ROMAN_PV[0][n % 10]);
            out.push(' ');
        }
        out.pop();
        Ok(out)
    }

    fn decode(&self, text: &str) -> Result<String, crate::errors::CodeError> {
        let mut out = String::new();
        for numeral in text.split(" ").filter(|w| !w.is_empty()) {
            let mut n = 0;
            for (l, r) in numeral.chars().tuple_windows() {
                let a = Self::char_to_num(l)?;
                let b = Self::char_to_num(r)?;
                if a > b {
                    n += a
                } else {
                    n -= a
                }
            }
            n += Self::char_to_num(numeral.chars().last().unwrap())?;
            out.push_str(&format!("{} ", n));
        }
        out.pop();
        Ok(out)
    }
}

#[cfg(test)]
mod roman_numeral_tests {
    use super::*;

    const PLAINTEXT: &'static str = "39 246 789 2421";
    const ENCODEDTEXT: &'static str = "XXXIX CCXLVI DCCLXXXIX MMCDXXI";

    #[test]
    fn encrypt_test() {
        let code = RomanNumeral::default();
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT);
    }

    #[test]
    fn decrypt_test() {
        let code = RomanNumeral::default();
        assert_eq!(code.decode(ENCODEDTEXT).unwrap(), PLAINTEXT);
    }
}
