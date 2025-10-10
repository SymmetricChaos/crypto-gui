use super::string_to_u32s;
use crate::traits::Code;
use itertools::Itertools;
use utils::errors::GeneralError;

const ROMAN_PV: [[&'static str; 10]; 3] = [
    ["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"],
    ["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"],
    ["", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"],
];

crate::lazy_regex!(
    RELAXED_ROMAN, r"^[IVXLCDM]+$";
    STRICT_ROMAN, r"^M{0,3}(CM|CD|D?C{0,3})(XC|XL|L?X{0,3})(IX|IV|V?I{0,3})$";
);

pub struct RomanNumeral {
    // pub apostrophus: bool,
}

impl Default for RomanNumeral {
    fn default() -> Self {
        Self {}
    }
}

impl RomanNumeral {
    fn char_to_num(c: char) -> Result<i32, GeneralError> {
        Ok(match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => return Err(GeneralError::invalid_input_char(c)),
        })
    }

    pub fn encode_int(n: u32) -> Result<String, GeneralError> {
        let n = n as usize;
        if n > 3999 {
            return Err(GeneralError::input(
                "standard Roman Numerals cannot be greater than 3999",
            ));
        }
        if n == 0 {
            return Err(GeneralError::input(
                "there is no standard Roman Numeral representation for 0",
            ));
        }
        let mut out = "M".repeat(n / 1000);
        out.push_str(ROMAN_PV[2][(n % 1000) / 100]);
        out.push_str(ROMAN_PV[1][(n % 100) / 10]);
        out.push_str(ROMAN_PV[0][n % 10]);
        Ok(out)
    }

    fn decode_to_int(numeral: &str) -> Result<u32, GeneralError> {
        if !RELAXED_ROMAN.is_match(numeral) {
            return Err(GeneralError::input(format!(
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
            return Err(GeneralError::input(format!(
                "the Roman Numeral `{}` evaluates to 0 or less which is not valid",
                numeral
            )));
        }
        Ok(n as u32)
    }
}

impl Code for RomanNumeral {
    fn encode(&self, text: &str) -> Result<String, GeneralError> {
        let mut output = Vec::new();

        for n in string_to_u32s(text, ",")? {
            output.push(Self::encode_int(n)?);
        }

        Ok(output.into_iter().join(", "))
    }

    fn decode(&self, text: &str) -> Result<String, GeneralError> {
        let mut output = Vec::new();

        for s in text.split(",").map(|s| s.trim()) {
            if s.is_empty() {
                continue;
            }
            output.push(Self::decode_to_int(s)?.to_string())
        }

        Ok(output.into_iter().join(", "))
    }
}

#[cfg(test)]
mod roman_numeral_tests {
    use super::*;

    const PTEXT: &'static str = "39, 246, 789, 2421, 9";
    const ENCODEDTEXT: &'static str = "XXXIX, CCXLVI, DCCLXXXIX, MMCDXXI, IX";

    #[test]
    fn encode_test() {
        let code = RomanNumeral::default();
        assert_eq!(code.encode(PTEXT).unwrap(), ENCODEDTEXT);
    }

    #[test]
    fn decode_test() {
        let code = RomanNumeral::default();
        assert_eq!(code.decode(ENCODEDTEXT).unwrap(), PTEXT);
    }
}
