use crate::{errors::CodeError, traits::Code};
use itertools::Itertools;
use std::cell::RefCell;

use super::{fibonacci_integers::FibonacciCodeIntegers, string_to_u32s};

// https://en.wikipedia.org/wiki/Fibonacci_coding

pub struct FibonacciCode {
    pub integer_code: RefCell<FibonacciCodeIntegers>,
    pub spaced: bool,
    pub sep: String,
}

impl Default for FibonacciCode {
    fn default() -> Self {
        let codes = FibonacciCodeIntegers::default();
        FibonacciCode {
            integer_code: RefCell::new(codes),
            spaced: false,
            sep: String::from(" "),
        }
    }
}

impl Code for FibonacciCode {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        let mut output = Vec::new();

        for n in string_to_u32s(text, &self.sep)? {
            output.push(self.integer_code.borrow_mut().encode_u32(n).clone());
        }

        if self.spaced {
            Ok(output.join(&self.sep))
        } else {
            Ok(output.join(""))
        }
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        let text = text.replace(&self.sep, "");
        let nums = self.integer_code.borrow_mut().decode_to_u32(&text)?;

        Ok(nums.into_iter().map(|n| n.to_string()).join(&self.sep))
    }
}

#[cfg(test)]
mod fibonacci_tests {
    use super::*;

    const PLAINTEXT: &'static str = "1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17";
    const ENCODEDTEXT: &'static str = "110110011101100011100110101100001110001101001100101110101100000111000011010001100100111010011";

    #[test]
    fn encode_test_integer() {
        let code = FibonacciCode::default();
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT);
    }

    #[test]
    fn decode_test_integer() {
        let code = FibonacciCode::default();
        assert_eq!(code.decode(ENCODEDTEXT).unwrap(), PLAINTEXT);
    }
}
