use std::{cell::RefCell, collections::HashMap};

use super::Code;
use crate::errors::Error;

// https://en.wikipedia.org/wiki/Fibonacci_coding
pub struct FibSeq {
    a: u32,
    b: u32,
}

// Note offset to start sequence in the right place
impl FibSeq {
    pub fn new() -> FibSeq {
        FibSeq { a: 1, b: 1 }
    }
}

impl Iterator for FibSeq {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let t = self.a + self.b;
        self.a = self.b;
        self.b = t;

        Some(self.a)
    }
}

pub struct FibonacciCodeIntegers {
    fib_gen: RefCell<FibSeq>,
    cached_fib_seq: RefCell<Vec<u32>>,
    cached_codes: RefCell<HashMap<u32, String>>,
}

impl FibonacciCodeIntegers {
    pub fn encode_u32(&self, n: u32) -> String {
        // Quickly check if the number has been encoded before
        if let Some(code) = self.cached_codes.borrow().get(&n) {
            return code.clone();
        }

        // Extend the cached list of fibonnaci numbers if needed
        while self.cached_fib_seq.borrow().last().unwrap() < &n {
            self.extend_seq()
        }

        let mut bits = String::new();

        bits.push('1');
        let mut val = n;
        for f in self
            .cached_fib_seq
            .borrow()
            .iter()
            .filter(|x| x <= &&n)
            .rev()
        {
            if *f <= val {
                bits.push('1');
                val -= f;
            } else {
                bits.push('0')
            }
        }

        // Reverse the bits, collect them into a String
        bits.chars().rev().collect::<String>()
    }

    pub fn decode_to_u32(&self, text: &str) -> Result<Vec<u32>, Error> {
        let mut output = Vec::new();
        let mut prev = '0';
        let mut ctr = 0;
        let mut n = 0;
        for bit in text.chars() {
            if prev == '1' && bit == '1' {
                output.push(n);
                prev = '0';
                ctr = 0;
                n = 0;
                continue;
            }
            match bit {
                '0' => (),
                '1' => n += self.get_nth_fib(ctr),
                _ => return Err(Error::invalid_input_char(bit)),
            }

            ctr += 1;
            prev = bit;
        }
        Ok(output)
    }

    fn get_nth_fib(&self, index: usize) -> u32 {
        if let Some(n) = self.cached_fib_seq.borrow().get(index) {
            return *n;
        }

        while self.cached_fib_seq.borrow().len() <= index {
            self.extend_seq()
        }
        self.cached_fib_seq.borrow()[index]
    }

    fn extend_seq(&self) {
        let new = self.fib_gen.borrow_mut().next().unwrap();
        self.cached_fib_seq.borrow_mut().push(new);
    }
}

impl Default for FibonacciCodeIntegers {
    fn default() -> Self {
        let mut fg = FibSeq::new();
        let mut fs = Vec::new();
        fs.push(fg.next().unwrap());
        Self {
            fib_gen: RefCell::new(fg),
            cached_fib_seq: RefCell::new(fs),
            cached_codes: RefCell::new(HashMap::new()),
        }
    }
}

impl Code for FibonacciCodeIntegers {
    fn encode(&self, text: &str) -> Result<String, Error> {
        let mut out = String::new();
        for s in text.split(" ") {
            let n = u32::from_str_radix(s, 10).map_err(|_| Error::invalid_input_group(s))?;
            out.push_str(&self.encode_u32(n))
        }
        Ok(out)
    }

    fn decode(&self, text: &str) -> Result<String, Error> {
        let mut output = Vec::new();
        let mut prev = '0';
        let mut ctr = 0;
        let mut n = 0;
        for bit in text.chars() {
            if prev == '1' && bit == '1' {
                output.push(format!("{n}"));
                prev = '0';
                ctr = 0;
                n = 0;
                continue;
            }
            match bit {
                '0' => (),
                '1' => n += self.get_nth_fib(ctr),
                _ => return Err(Error::invalid_input_char(bit)),
            }

            ctr += 1;
            prev = bit;
        }
        Ok(output.join(" "))
    }

    fn randomize(&mut self) {}

    fn reset(&mut self) {}
}

#[cfg(test)]
mod fibonacci_int_tests {
    use super::*;

    const PLAINTEXT: &'static str = "65 1 7";
    const ENCODEDTEXT: &'static str = "01001000111101011";

    #[test]
    fn encode_test() {
        let code = FibonacciCodeIntegers::default();
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT);
    }

    #[test]
    fn decode_test() {
        let code = FibonacciCodeIntegers::default();
        assert_eq!(code.decode(ENCODEDTEXT).unwrap(), PLAINTEXT);
    }
}
