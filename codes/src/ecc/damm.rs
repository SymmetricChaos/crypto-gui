use crate::{errors::CodeError, traits::Code};
use std::cell::LazyCell;

pub const DAMM_TABLE: LazyCell<[[usize; 10]; 10]> = LazyCell::new(|| {
    [
        [0, 3, 1, 7, 5, 9, 8, 6, 4, 2],
        [7, 0, 9, 2, 1, 5, 4, 8, 6, 3],
        [4, 2, 0, 6, 8, 7, 1, 3, 5, 9],
        [1, 7, 5, 0, 9, 8, 3, 4, 2, 6],
        [6, 1, 2, 3, 0, 4, 5, 9, 7, 8],
        [3, 6, 7, 4, 2, 0, 9, 5, 8, 1],
        [5, 8, 6, 9, 7, 2, 0, 1, 3, 4],
        [8, 9, 4, 5, 3, 6, 2, 0, 1, 7],
        [9, 4, 3, 8, 6, 1, 7, 2, 0, 5],
        [2, 5, 8, 1, 4, 3, 6, 7, 9, 0],
    ]
});

fn check(digit: usize, interim: usize) -> usize {
    DAMM_TABLE[interim][digit]
}

pub struct Damm {}

impl Damm {
    pub fn check_csv_damm(&self, list: &str) -> String {
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

impl Default for Damm {
    fn default() -> Self {
        Self {}
    }
}

impl Code for Damm {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        if text.is_empty() {
            return Err(CodeError::input("input cannot be empty"));
        }
        let mut interim = 0;

        for c in text.chars() {
            let digit = c.to_digit(10).ok_or(CodeError::invalid_input_char(c))?;
            interim = check(digit as usize, interim);
        }

        let mut out = text.to_string();
        out.push(char::from_digit(interim as u32, 10).unwrap());
        Ok(out)
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        if text.is_empty() {
            return Err(CodeError::input("input cannot be empty"));
        }
        let mut interim = 0;

        for c in text.chars() {
            let digit = c.to_digit(10).ok_or(CodeError::invalid_input_char(c))?;
            interim = check(digit as usize, interim);
        }

        if interim != 0 {
            return Err(CodeError::input("invalid check digit"));
        } else {
            Ok(text[..text.len() - 1].to_string())
        }
    }
}

#[cfg(test)]
mod damm_tests {
    use super::*;

    #[test]
    fn test_encode() {
        let code = Damm::default();
        assert_eq!(code.encode("572").unwrap(), "5724");
    }

    #[test]
    fn test_decode() {
        let code = Damm::default();
        assert_eq!(code.decode("5724").unwrap(), "572");
    }

    #[test]
    fn test_decode_with_err() {
        let code = Damm::default();
        assert_eq!(
            code.decode("5723").unwrap_err(),
            CodeError::input("invalid check digit")
        );
    }
}
