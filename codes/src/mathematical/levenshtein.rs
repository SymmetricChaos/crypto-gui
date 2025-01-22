use num::Integer;

use super::{string_to_i32s, string_to_u32s};
use crate::{errors::CodeError, traits::Code};

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

impl Code for LevenshteinCode {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        let mut out = Vec::new();

        if self.signed {
            for n in string_to_i32s(text, ",")? {
                if n.is_negative() {
                    out.push(u32_to_levenshtein((n.abs() * 2 - 1) as u32));
                } else {
                    out.push(u32_to_levenshtein((n.abs() * 2) as u32))
                }
            }
        } else {
            for n in string_to_u32s(text, ",")? {
                out.push(u32_to_levenshtein(n));
            }
        }

        if self.spaced {
            Ok(out.join(", "))
        } else {
            Ok(out.join(""))
        }
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        let mut out = Vec::new();

        for n in levenshtein_to_u32(text).into_iter() {
            if let Some(val) = n {
                if self.signed {
                    let v: i32 = match val.try_into() {
                        Ok(v) => v,
                        Err(_) => {
                            return Err(CodeError::input("encountered code group out of i32 range"))
                        }
                    };
                    if val.is_even() {
                        out.push((v / 2).to_string());
                    } else {
                        out.push(((-v - 1) / 2).to_string());
                    }
                } else {
                    out.push(val.to_string())
                }
            } else {
                out.push(String::from("�"))
            }
        }

        Ok(out.join(", "))
    }
}

#[cfg(test)]
mod levenshtein_int_tests {
    use super::*;

    const PLAINTEXT: &'static str = "0, 1, 2, 3";
    const PLAINTEXT_SIGNED: &'static str = "0, -1, 1, -2";
    const ENCODEDTEXT: &'static str = "01011001101";
    const ENCODEDTEXT_SP: &'static str = "0, 10, 1100, 1101";

    #[test]
    fn encode_test() {
        let mut code = LevenshteinCode::default();
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT);
        code.signed = true;
        code.spaced = true;
        assert_eq!(code.encode(PLAINTEXT_SIGNED).unwrap(), ENCODEDTEXT_SP);
    }

    #[test]
    fn encode_sp_test() {
        let mut code = LevenshteinCode::default();
        code.spaced = true;
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT_SP);
    }

    #[test]
    fn decode_test() {
        let code = LevenshteinCode::default();
        assert_eq!(code.decode(ENCODEDTEXT).unwrap(), PLAINTEXT);
    }

    #[test]
    fn decode_sp_test() {
        let code = LevenshteinCode::default();
        assert_eq!(code.decode(ENCODEDTEXT_SP).unwrap(), PLAINTEXT);
    }
}
