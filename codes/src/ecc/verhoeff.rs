use crate::{errors::CodeError, traits::Code};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref VERHOEFF_PERM_TABLE: [[u8; 10]; 8] = [
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
        [1, 5, 7, 6, 2, 8, 3, 0, 9, 4],
        [5, 8, 0, 3, 7, 9, 6, 1, 4, 2],
        [8, 9, 1, 6, 0, 4, 3, 5, 2, 7],
        [9, 4, 5, 3, 1, 2, 6, 8, 7, 0],
        [4, 2, 8, 6, 5, 7, 3, 9, 0, 1],
        [2, 7, 9, 3, 8, 0, 6, 4, 1, 5],
        [7, 0, 4, 6, 9, 1, 3, 2, 5, 8]
    ];
    pub static ref VERHOEFF_MUL_TABLE: [[u8; 10]; 10] = [
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
        [1, 2, 3, 4, 0, 6, 7, 8, 9, 5],
        [2, 3, 4, 0, 1, 7, 8, 9, 5, 6],
        [3, 4, 0, 1, 2, 8, 9, 5, 6, 7],
        [4, 0, 1, 2, 3, 9, 5, 6, 7, 8],
        [5, 9, 8, 7, 6, 0, 4, 3, 2, 1],
        [6, 5, 9, 8, 7, 1, 0, 4, 3, 2],
        [7, 6, 5, 9, 8, 2, 1, 0, 4, 3],
        [8, 7, 6, 5, 9, 3, 2, 1, 0, 4],
        [9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
    ];
}

pub struct VerhoeffAlgorithm {}

impl VerhoeffAlgorithm {
    fn mul(a: char, b: char) -> Result<char, CodeError> {
        let j = a
            .to_digit(10)
            .ok_or_else(|| CodeError::invalid_input_char(a))? as usize;
        let k = b
            .to_digit(10)
            .ok_or_else(|| CodeError::invalid_input_char(b))? as usize;
        Ok((VERHOEFF_MUL_TABLE[j][k] + 48) as char)
    }

    fn perm(pos: usize, num: char) -> Result<char, CodeError> {
        let n = num
            .to_digit(10)
            .ok_or_else(|| CodeError::invalid_input_char(num))? as usize;

        Ok((VERHOEFF_PERM_TABLE[pos][n] + 48) as char)
    }

    fn inv(a: char) -> Result<char, CodeError> {
        match a {
            '0' => Ok('0'),
            '1' => Ok('4'),
            '2' => Ok('3'),
            '3' => Ok('2'),
            '4' => Ok('1'),
            '5' => Ok('5'),
            '6' => Ok('6'),
            '7' => Ok('7'),
            '8' => Ok('8'),
            '9' => Ok('9'),
            _ => Err(CodeError::invalid_input_char(a)),
        }
    }

    pub fn check_csv_verhoeff(&self, list: &str) -> String {
        let mut out = String::new();
        for line in list.split(",").into_iter() {
            let result = self.decode(line);
            if result.is_ok() {
                out.push_str(line.trim());
                out.push_str(" [valid],\n");
            } else {
                out.push_str(line.trim());
                out.push_str(" [");
                out.push_str(&result.unwrap_err().inner());
                out.push(']');
                out.push_str(",\n");
            }
        }
        out
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
        for (i, n) in text.chars().chain(std::iter::once('0')).rev().enumerate() {
            check = Self::mul(check, Self::perm(i % 8, n)?)?;
        }
        let mut out = text.to_string();
        out.push(Self::inv(check)?);
        Ok(out)
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        if text.is_empty() {
            return Err(CodeError::input("input cannot be empty"));
        }
        let mut check = '0';
        for (i, c) in text.chars().rev().enumerate() {
            check = Self::mul(check, Self::perm(i % 8, c)?)?;
        }
        if check != '0' {
            return Err(CodeError::input("invalid check digit"));
        } else {
            Ok(text[0..text.len() - 1].to_string())
        }
    }
}

#[cfg(test)]
mod verhoeff_tests {
    use super::*;

    #[test]
    fn test_encode() {
        let code = VerhoeffAlgorithm::default();
        assert_eq!(code.encode("236").unwrap(), "2363");
    }

    #[test]
    fn test_decode() {
        let code = VerhoeffAlgorithm::default();
        assert_eq!(code.decode("2363").unwrap(), "236");
    }

    #[test]
    fn test_decode_with_err() {
        let code = VerhoeffAlgorithm::default();
        assert_eq!(
            code.decode("2365").unwrap_err(),
            CodeError::input("invalid check digit")
        );
    }
}
