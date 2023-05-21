use std::{cell::RefCell, collections::HashMap};

use itertools::Itertools;

use crate::{errors::CodeError, traits::Code};

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

    pub fn decode_to_u32(&self, text: &str) -> Result<Vec<u32>, CodeError> {
        // if !IS_BITSTRING.is_match(text) {
        //     return Err(CodeError::Input("Not a string of bits".into()));
        // }
        let mut vec = Vec::new();
        let mut bits = text.chars().peekable();
        loop {
            // Count the number of '1's until a '0' is encountered
            let mut ctr = 0;
            if bits.peek() == None {
                break;
            }
            while bits.next() == Some('1') {
                ctr += 1;
            }
            // If the COUNT is zero the value is 0
            if ctr == 0 {
                vec.push(0u32)
            } else {
                // Otherwise start with N = 1 and repeat the next step COUNT-1 times
                let mut n = 1_u32;
                for _ in 0..(ctr - 1) {
                    // Read N bits, prepend '1' and assign the value to N
                    let mut value = 1;
                    for _ in 0..n {
                        match bits.next() {
                            Some(c) => {
                                value <<= 1;
                                if c == '1' {
                                    value ^= 1;
                                }
                            }
                            None => {
                                return Err(CodeError::Input(
                                    "malformed coding, not enough bits".into(),
                                ))
                            }
                        }
                    }
                    n = value
                }
                vec.push(n)
            }
        }

        Ok(vec)
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
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        let mut out = String::new();
        for s in text.split(" ") {
            let n = u32::from_str_radix(s, 10).map_err(|_| CodeError::invalid_input_group(s))?;
            out.push_str(&self.encode_u32(n))
        }
        Ok(out)
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        Ok(self
            .decode_to_u32(text)?
            .into_iter()
            .map(|n| n.to_string())
            .join(" "))
    }
}

#[cfg(test)]
mod levenshtein_int_tests {
    use super::*;

    const PLAINTEXT: &'static str = "16 0 2 10";
    const ENCODEDTEXT: &'static str = "1111000000000110011101010";

    #[test]
    fn encode_test() {
        let code = LevenshteinCodeIntegers::default();
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT);
    }

    #[test]
    fn decode_test() {
        let code = LevenshteinCodeIntegers::default();
        assert_eq!(code.decode(ENCODEDTEXT).unwrap(), PLAINTEXT);
    }

    #[test]
    fn remove_bit() {
        let code = LevenshteinCodeIntegers::default();
        for i in 0..10 {
            println!("{}", code.encode_u32(i))
        }
    }

    #[test]
    fn decode_to_nums() {
        let code = LevenshteinCodeIntegers::default();
        println!("{:?}", code.decode_to_u32(ENCODEDTEXT));
    }
}
