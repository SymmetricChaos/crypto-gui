use std::{cell::RefCell, collections::HashMap};

use super::Code;
use crate::errors::Error;

pub struct LevenshteinCodeIntegers {
    cached_codes: RefCell<HashMap<u32, String>>,
}

impl LevenshteinCodeIntegers {
    pub fn encode_u32(&self, n: u32) -> String {
        // Quickly check if the number has been encoded before
        if let Some(code) = self.cached_codes.borrow().get(&n) {
            return code.clone();
        }

        let mut bits = String::new();
        let mut t = n;
        // Start counter a 1
        let mut ctr = 1;
        loop {
            // Write t in standard binary without the leading 1 to the start of the code
            let code = &format!("{:b}", t)[1..];
            bits.insert_str(0, code);
            // Let M be the number of bits written in step 2
            let m = code.len() as u32;
            // if M != 0 increase C by 1, make M the new value of t, return to step 2
            if m != 0 {
                ctr += 1;
                t = m;
            } else {
                break;
            }
        }
        // Write C 1s and a 0 to the start of the code
        bits.insert(0, '0');
        bits.insert_str(0, &"1".repeat(ctr));

        self.cached_codes.borrow_mut().insert(n, bits.clone());

        bits
    }

    pub fn decode_to_u32(&self, text: &str) -> Result<Vec<u32>, Error> {
        todo!()
    }
}

impl Default for LevenshteinCodeIntegers {
    fn default() -> Self {
        let map = HashMap::from_iter([(0, "0".into())].into_iter());
        Self {
            cached_codes: RefCell::new(map),
        }
    }
}

impl Code for LevenshteinCodeIntegers {
    fn encode(&self, text: &str) -> Result<String, Error> {
        let mut out = String::new();
        for s in text.split(" ") {
            let n = u32::from_str_radix(s, 10).map_err(|_| Error::invalid_input_group(s))?;
            out.push_str(&self.encode_u32(n))
        }
        Ok(out)
    }

    fn decode(&self, text: &str) -> Result<String, Error> {
        todo!()
    }

    fn randomize(&mut self) {}

    fn reset(&mut self) {}
}

#[cfg(test)]
mod fibonacci_int_tests {
    use super::*;

    const PLAINTEXT: &'static str = "16 0 2 10";
    const ENCODEDTEXT: &'static str = "1111000000000110011101010";

    #[test]
    fn encode_test() {
        let code = LevenshteinCodeIntegers::default();
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT);
    }

    // #[test]
    // fn decode_test() {
    //     let code = LevenshteinCodeIntegers::default();
    //     assert_eq!(code.decode(ENCODEDTEXT).unwrap(), PLAINTEXT);
    // }

    #[test]
    fn remove_bit() {
        let code = LevenshteinCodeIntegers::default();
        for i in 0..10 {
            println!("{}", code.encode_u32(i))
        }
    }
}
