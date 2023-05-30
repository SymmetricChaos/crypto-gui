use lazy_static::lazy_static;
use regex::Regex;

use crate::{errors::CodeError, traits::Code};

lazy_static! {
    pub static ref ISBN_10: Regex = Regex::new(r"^([0-9]{9}[0-9X])$").unwrap();
    pub static ref ISBN_13: Regex = Regex::new(r"^([0-9]{13})$").unwrap();
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum IsbnVariant {
    Ten,
    Thirteen,
}

pub struct Isbn {
    pub variant: IsbnVariant,
}

pub fn is_valid_isbn_10(text: &str) -> Result<(), CodeError> {
    if !ISBN_10.is_match(&text.chars().filter(|c| *c != '-').collect::<String>()) {
        return Err(CodeError::input("not a well formed ISBN-10 code"));
    }

    let mut check = 0;
    for (c, idx) in text
        .chars()
        .filter(|c| *c != '-')
        .zip([10, 9, 8, 7, 6, 5, 4, 3, 2, 1].into_iter())
    {
        match c.to_digit(10) {
            Some(n) => check += idx * n,
            None => check += idx * 10, // only case that can reach this is 'X'
        }
    }
    if check % 11 == 0 {
        Ok(())
    } else {
        Err(CodeError::input("invalid check digit"))
    }
}

pub fn is_valid_isbn_13(text: &str) -> Result<(), CodeError> {
    if !ISBN_13.is_match(&text.chars().filter(|c| *c != '-').collect::<String>()) {
        return Err(CodeError::input("not a well formed ISBN-13 code"));
    }

    let mut check = 0;
    for (c, idx) in text
        .chars()
        .filter(|c| *c != '-')
        .zip([1, 3].into_iter().cycle())
    {
        match c.to_digit(10) {
            Some(n) => check += idx * n,
            None => unreachable!("only valid digits can reach this point"),
        }
    }
    if check % 10 == 0 {
        Ok(())
    } else {
        Err(CodeError::input("invalid check digit"))
    }
}

impl Isbn {
    pub fn check_csv_isbn(&self, list: &str) -> String {
        let mut out = String::new();
        for line in list.split(",").into_iter() {
            let result = match self.variant {
                IsbnVariant::Ten => is_valid_isbn_10(line.trim()),
                IsbnVariant::Thirteen => is_valid_isbn_13(line.trim()),
            };
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

impl Default for Isbn {
    fn default() -> Self {
        Self {
            variant: IsbnVariant::Thirteen,
        }
    }
}

impl Code for Isbn {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        if text.is_empty() {
            return Err(CodeError::input("input cannot be empty"));
        }

        todo!()
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        if text.is_empty() {
            return Err(CodeError::input("input cannot be empty"));
        }
        match self.variant {
            IsbnVariant::Ten => {
                is_valid_isbn_10(text)?;
                Ok(text[0..text.len() - 1].to_string())
            }
            IsbnVariant::Thirteen => {
                is_valid_isbn_13(text)?;
                Ok(text[0..text.len() - 1].to_string())
            }
        }
    }
}

#[cfg(test)]
mod isbn_tests {
    use super::*;

    #[test]
    fn test_isbn_10_valid() {
        assert!(is_valid_isbn_10("0-306-40615-2").is_ok());
        assert!(is_valid_isbn_10("0306406152").is_ok());
        assert!(is_valid_isbn_10("0-306-4615-2").is_err());
        assert!(is_valid_isbn_10("0-306-40615-1").is_err());
        assert!(is_valid_isbn_10("0-306-40165-2").is_err());
    }

    #[test]
    fn test_isbn_13_valid() {
        assert!(is_valid_isbn_13("978-0-306-40615-7").is_ok());
        assert!(is_valid_isbn_13("9780306406157").is_ok());
        assert!(is_valid_isbn_13("978-0-306-4015-7").is_err());
        assert!(is_valid_isbn_13("978-0-306-40615-3").is_err());
        assert!(is_valid_isbn_13("978-0-360-40615-7").is_err());
    }
}
