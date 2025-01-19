use super::{string_to_i32s, string_to_u32s};
use crate::{errors::CodeError, traits::Code};
use itertools::Itertools;
use num::Integer;

pub struct UnaryCode {
    pub invert: bool,
    pub symmetric: bool,
    pub signed: bool,
    pub spaced: bool,
}

impl Default for UnaryCode {
    fn default() -> Self {
        UnaryCode {
            invert: false,
            symmetric: false,
            signed: false,
            spaced: false,
        }
    }
}

impl UnaryCode {
    pub fn encode_u32(&self, n: u32) -> String {
        let n = n as usize;
        if self.symmetric {
            if self.invert {
                if n == 0 {
                    return String::from("0");
                } else {
                    format!("1{}1", "0".repeat(n - 1))
                }
            } else {
                if n == 0 {
                    return String::from("1");
                } else {
                    format!("0{}0", "1".repeat(n - 1))
                }
            }
        } else {
            if self.invert {
                "0".repeat(n) + "1"
            } else {
                "1".repeat(n) + "0"
            }
        }
    }

    pub fn encode_i32(&self, n: i32) -> String {
        if n.is_negative() {
            self.encode_u32((n.abs() * 2 - 1) as u32)
        } else {
            self.encode_u32((n.abs() * 2) as u32)
        }
    }

    pub fn recognize_code(&self, text: &str) -> Vec<Option<u32>> {
        let mut output = Vec::new();

        let (z0, z1) = if self.invert { ('0', '1') } else { ('1', '0') };

        let mut ctr = 0;
        for c in text.chars() {
            if c.is_whitespace() {
                continue;
            }
            if c == z0 {
                ctr += 1
            } else if c == z1 {
                output.push(Some(ctr));
                ctr = 0;
            } else {
                output.push(None);
                ctr = 0;
            }
        }
        if ctr != 0 {
            output.push(None)
        }
        output
    }

    pub fn recognize_code_signed(&self, text: &str) -> Vec<Option<i32>> {
        let mut output = Vec::new();

        let (z0, z1) = if self.invert { ('0', '1') } else { ('1', '0') };

        let mut ctr = 0;
        for c in text.chars() {
            if c.is_whitespace() {
                continue;
            }
            if c == z0 {
                ctr += 1
            } else if c == z1 {
                if ctr.is_even() {
                    output.push(Some(ctr / 2));
                } else {
                    output.push(Some((-ctr - 1) / 2));
                }

                ctr = 0;
            } else {
                output.push(None);
                ctr = 0;
            }
        }
        if ctr != 0 {
            output.push(None)
        }
        output
    }

    pub fn recognize_code_symmetric(&self, text: &str) -> Vec<Option<u32>> {
        let mut output = Vec::new();
        let mut buffer = String::new();

        for b in text.chars() {
            // Invalid characters immediatly give '?' response and restart
            if b != '0' && b != '1' {
                output.push(None);
                buffer.clear();
                continue;
            }
            // The '1' bit on its own is a valid code
            if buffer.is_empty() && b == '1' {
                output.push(Some(0));
                buffer.clear();
                continue;
            }
            // If the starting bit is '0' push it and continue
            if buffer.is_empty() && b == '0' {
                buffer.push(b);
            // Otherwise push the next bit on
            } else {
                if b == '0' {
                    output.push(Some(buffer.chars().count() as u32));
                    buffer.clear();
                } else {
                    buffer.push('1')
                }
            }
        }
        // If anything remains in the buffer it is invalid
        if !buffer.is_empty() {
            output.push(None)
        }
        output
    }
}

impl Code for UnaryCode {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        let mut output = Vec::new();

        if self.signed {
            for n in string_to_i32s(text, ",")? {
                output.push(self.encode_i32(n));
            }
        } else {
            for n in string_to_u32s(text, ",")? {
                output.push(self.encode_u32(n));
            }
        }

        if self.spaced {
            Ok(output.into_iter().join(", "))
        } else {
            Ok(output.into_iter().join(""))
        }
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        let mut output = Vec::new();

        if self.signed {
            if self.symmetric {
                for section in self.recognize_code_symmetric(&text) {
                    if let Some(code) = section {
                        if code.is_even() {
                            output.push((code as i32 / 2).to_string());
                        } else {
                            output.push(((-(code as i32) - 1) / 2).to_string());
                        }
                    } else {
                        output.push(String::from("�"));
                    }
                }
            } else {
                for section in self.recognize_code(&text) {
                    if let Some(code) = section {
                        if code.is_even() {
                            output.push((code as i32 / 2).to_string());
                        } else {
                            output.push(((-(code as i32) - 1) / 2).to_string());
                        }
                    } else {
                        output.push(String::from("�"));
                    }
                }
            }
        } else {
            if self.symmetric {
                for section in self.recognize_code_symmetric(&text) {
                    if let Some(code) = section {
                        output.push(code.to_string());
                    } else {
                        output.push(String::from("�"));
                    }
                }
            } else {
                for section in self.recognize_code(&text) {
                    if let Some(code) = section {
                        output.push(code.to_string());
                    } else {
                        output.push(String::from("�"));
                    }
                }
            }
        }

        Ok(output.into_iter().join(", "))
    }
}

#[cfg(test)]
mod unary_tests {
    use super::*;

    const PLAINTEXT: &'static str = "0, 1, 2, 3, 4, 5, 6, 7, 8, 9";
    const PLAINTEXT_SIGNED: &'static str = "0, -1, 1, -2, 2, -3, 3, -4, 4, -5";
    const ENCODEDTEXT: &'static str = "0101101110111101111101111110111111101111111101111111110";
    const ENCODEDTEXT_SYM: &'static str = "1000100110011100111100111110011111100111111100111111110";

    #[test]
    fn encode_test() {
        let mut code = UnaryCode::default();
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT);
        code.symmetric = true;
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT_SYM);
    }

    #[test]
    fn encode_test_signed() {
        let mut code = UnaryCode::default();
        code.signed = true;
        assert_eq!(code.encode(PLAINTEXT_SIGNED).unwrap(), ENCODEDTEXT);
        code.symmetric = true;
        assert_eq!(code.encode(PLAINTEXT_SIGNED).unwrap(), ENCODEDTEXT_SYM);
    }

    #[test]
    fn decode_test() {
        let mut code = UnaryCode::default();
        assert_eq!(code.decode(ENCODEDTEXT).unwrap(), PLAINTEXT);
        code.symmetric = true;
        assert_eq!(code.decode(ENCODEDTEXT_SYM).unwrap(), PLAINTEXT);
    }

    #[test]
    fn decode_test_signed() {
        let mut code = UnaryCode::default();
        code.signed = true;
        assert_eq!(code.decode(ENCODEDTEXT).unwrap(), PLAINTEXT_SIGNED);
        code.symmetric = true;
        assert_eq!(code.decode(ENCODEDTEXT_SYM).unwrap(), PLAINTEXT_SIGNED);
    }
}
