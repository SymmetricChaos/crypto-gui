use utils::errors::GeneralError;

use super::{decode_prefix_to_strings, i32_to_u32_zigzag, string_to_i32s, string_to_u32s};
use crate::traits::Code;

// https://en.wikipedia.org/wiki/Levenshtein_coding

pub fn u32_to_levenshtein(n: u32) -> String {
    if n == 0 {
        return String::from("0");
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

    bits
}

pub fn levenshtein_to_u32(text: &str) -> Vec<Option<u32>> {
    let mut vec = Vec::new();
    let mut bits = text.chars().filter(|c| *c == '0' || *c == '1').peekable();
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
            vec.push(Some(0u32))
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
                        None => vec.push(None),
                    }
                }
                n = value
            }
            vec.push(Some(n))
        }
    }

    vec
}

fn levenshtein_to_u32_single(text: &str) -> Option<u32> {
    let o = levenshtein_to_u32(text);
    if o.len() != 1 {
        return None;
    } else {
        return o[0];
    }
}

pub struct LevenshteinCode {
    pub spaced: bool,
    pub signed: bool,
}

impl Default for LevenshteinCode {
    fn default() -> Self {
        LevenshteinCode {
            spaced: false,
            signed: false,
        }
    }
}

impl LevenshteinCode {
    pub fn encode_u32(&self, n: u32) -> String {
        u32_to_levenshtein(n)
    }

    pub fn encode_i32(&self, n: i32) -> String {
        if let Some(x) = i32_to_u32_zigzag(n) {
            self.encode_u32(x)
        } else {
            String::from("�")
        }
    }
}

impl Code for LevenshteinCode {
    fn encode(&self, text: &str) -> Result<String, GeneralError> {
        let mut out = Vec::new();

        if self.signed {
            for n in string_to_i32s(text, ",")? {
                out.push(self.encode_i32(n))
            }
        } else {
            for n in string_to_u32s(text, ",")? {
                out.push(self.encode_u32(n))
            }
        }

        if self.spaced {
            Ok(out.join(", "))
        } else {
            Ok(out.join(""))
        }
    }

    fn decode(&self, text: &str) -> Result<String, GeneralError> {
        let mut out = Vec::new();

        if self.spaced {
            for section in text.split(",").map(|s| s.trim()) {
                decode_prefix_to_strings(levenshtein_to_u32_single(section), self.signed, &mut out);
            }
        } else {
            for section in levenshtein_to_u32(&text) {
                decode_prefix_to_strings(section, self.signed, &mut out);
            }
        }

        Ok(out.join(", "))
    }
}

#[cfg(test)]
mod levenshtein_int_tests {
    use super::*;

    const PTEXT: &'static str = "0, 1, 2, 3";
    const PTEXT_SIGNED: &'static str = "0, -1, 1, -2";
    const ENCODEDTEXT: &'static str = "01011001101";
    const ENCODEDTEXT_SP: &'static str = "0, 10, 1100, 1101";

    #[test]
    fn encode_test() {
        let mut code = LevenshteinCode::default();
        assert_eq!(code.encode(PTEXT).unwrap(), ENCODEDTEXT);
        code.signed = true;
        code.spaced = true;
        assert_eq!(code.encode(PTEXT_SIGNED).unwrap(), ENCODEDTEXT_SP);
    }

    #[test]
    fn encode_sp_test() {
        let mut code = LevenshteinCode::default();
        code.spaced = true;
        assert_eq!(code.encode(PTEXT).unwrap(), ENCODEDTEXT_SP);
    }

    #[test]
    fn decode_test() {
        let code = LevenshteinCode::default();
        assert_eq!(code.decode(ENCODEDTEXT).unwrap(), PTEXT);
    }

    #[test]
    fn decode_sp_test() {
        let code = LevenshteinCode::default();
        assert_eq!(code.decode(ENCODEDTEXT_SP).unwrap(), PTEXT);
    }
}
