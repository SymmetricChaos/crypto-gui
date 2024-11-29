use crate::{errors::CodeError, traits::Code};
use itertools::Itertools;
use utils::text_functions::swap_ab;

pub struct SymmetricUnaryCode {
    pub invert: bool,
    pub sep: String,
}

impl Default for SymmetricUnaryCode {
    fn default() -> Self {
        SymmetricUnaryCode {
            invert: false,
            sep: String::from(" "),
        }
    }
}

impl SymmetricUnaryCode {
    pub fn encode_usize(&self, n: usize) -> String {
        if n == 0 {
            return String::from("1");
        } else {
            format!("0{}0", "1".repeat(n - 1))
        }
    }

    pub fn recognize_code(&self, text: &str) -> Vec<Option<usize>> {
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
                    output.push(Some(buffer.chars().count()));
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

impl Code for SymmetricUnaryCode {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        let mut output = String::new();

        for w in text.split(" ") {
            let n = usize::from_str_radix(w, 10).map_err(|e| CodeError::Input(e.to_string()))?;
            if n == 0 {
                output.push('1');
            } else {
                output.push_str(&format!("0{}0", "1".repeat(n - 1)))
            }
        }

        if self.invert {
            Ok(swap_ab('0', '1', &output))
        } else {
            Ok(output)
        }
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        let mut output = Vec::new();

        let text = if self.invert {
            swap_ab('0', '1', text)
        } else {
            text.to_string()
        };

        for section in self.recognize_code(&text) {
            if let Some(code) = section {
                output.push(code.to_string());
            } else {
                output.push(String::from("�"));
            }
        }

        Ok(output.into_iter().join(&self.sep))
    }
}

#[cfg(test)]
mod symmetric_unary_tests {
    use super::*;

    const PLAINTEXT: &'static str = "ETAO";
    const ENCODEDTEXT: &'static str = "1000100110";

    #[test]
    fn encode_test() {
        let code = SymmetricUnaryCode::default();
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT);
    }

    #[test]
    fn decode_test() {
        let code = SymmetricUnaryCode::default();
        assert_eq!(code.decode(ENCODEDTEXT).unwrap(), PLAINTEXT);
    }
}
