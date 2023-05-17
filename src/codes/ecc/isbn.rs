use crate::{codes::Code, errors::Error};

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref ISBN_10: Regex =
        Regex::new(r"^([0-9]\-[0-9]{3}\-[0-9]{5}\-[0-9X])|([0-9]{9}[0-9X])$").unwrap();
    pub static ref ISBN_13: Regex =
        Regex::new(r"^([0-9]{3}\-[0-9]\-[0-9]{3}\-[0-9]{5}\-[0-9])|([0-9]{13})$").unwrap();
}

pub enum IsbnVariant {
    Ten,
    Thirteen,
}

pub struct Isbn {
    pub variant: IsbnVariant,
}

impl Isbn {
    fn is_valid_isbn_10(&self, text: &str) -> bool {
        if !ISBN_10.is_match(text) {
            return false;
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
        check % 11 == 0
    }

    fn is_valid_isbn_13(&self, text: &str) -> bool {
        if !ISBN_13.is_match(text) {
            return false;
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
        check % 10 == 0
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
    fn encode(&self, text: &str) -> Result<String, Error> {
        if text.is_empty() {
            return Err(Error::input("input cannot be empty"));
        }

        todo!()
    }

    fn decode(&self, text: &str) -> Result<String, Error> {
        if text.is_empty() {
            return Err(Error::input("input cannot be empty"));
        }
        todo!()
    }

    fn randomize(&mut self) {}

    fn reset(&mut self) {}
}

#[cfg(test)]
mod isbn_tests {
    use super::*;

    #[test]
    fn test_isbn_10_valid() {
        let code = Isbn::default();
        assert!(code.is_valid_isbn_10("0-306-40615-2"));
        assert!(code.is_valid_isbn_10("0306406152"));
        assert!(!code.is_valid_isbn_10("0-306-4615-2"));
        assert!(!code.is_valid_isbn_10("0306-4615-2"));
        assert!(!code.is_valid_isbn_10("0-306-40615-1"));
        assert!(!code.is_valid_isbn_10("0-306-40165-2"));
    }

    #[test]
    fn test_isbn_13_valid() {
        let code = Isbn::default();
        assert!(code.is_valid_isbn_13("978-0-306-40615-7"));
        assert!(code.is_valid_isbn_13("9780306406157"));
        assert!(!code.is_valid_isbn_13("978-0-306-4015-7"));
        assert!(!code.is_valid_isbn_13("978-0-306-406157"));
        assert!(!code.is_valid_isbn_13("978-0-306-40615-3"));
        assert!(!code.is_valid_isbn_13("978-0-360-40615-7"));
    }

    #[test]
    fn test_encode() {
        let code = Isbn::default();
        assert_eq!(code.encode("").unwrap(), "");
    }

    #[test]
    fn test_decode() {
        let code = Isbn::default();
        assert_eq!(code.decode("").unwrap(), "");
    }

    #[test]
    fn test_decode_with_err() {
        let code = Isbn::default();
        assert_eq!(
            code.decode("").unwrap_err(),
            Error::input("check digit does not match")
        );
    }
}
