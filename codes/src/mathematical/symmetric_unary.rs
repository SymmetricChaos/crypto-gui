use utils::text_functions::swap_ab;

use crate::{
    errors::CodeError,
    letter_word_code::{IOMode, LetterWordIntCode},
    traits::Code,
};

pub struct SymmetricUnaryCode {
    pub maps: LetterWordIntCode,
    pub mode: IOMode,
    pub invert: bool,
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
        let mut buffer = String::with_capacity(self.maps.alphabet.chars().count());

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

impl Default for SymmetricUnaryCode {
    fn default() -> Self {
        let mut maps = LetterWordIntCode::new();
        maps.alphabet = String::from("ETAOINSHRDLCUMWFGYPBVKJXQZ");

        SymmetricUnaryCode {
            maps,
            mode: IOMode::Letter,
            invert: false,
        }
    }
}

impl Code for SymmetricUnaryCode {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        let mut output = String::new();

        if self.mode == IOMode::Letter {
            for c in text.chars() {
                let n = self.maps.char_to_int(c)?;
                output.push_str(&self.encode_usize(n))
            }
        } else if self.mode == IOMode::Word {
            for w in text.split(" ") {
                let n = self.maps.word_to_int(w)?;
                output.push_str(&self.encode_usize(n))
            }
        } else {
            for w in text.split(" ") {
                let n =
                    usize::from_str_radix(w, 10).map_err(|e| CodeError::Input(e.to_string()))?;
                if n == 0 {
                    output.push('1');
                } else {
                    output.push_str(&format!("0{}0", "1".repeat(n - 1)))
                }
            }
        }
        if self.invert {
            Ok(swap_ab('0', '1', &output))
        } else {
            Ok(output)
        }
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        let mut output = String::new();
        let text = if self.invert {
            swap_ab('0', '1', text)
        } else {
            text.to_string()
        };
        if self.mode == IOMode::Letter {
            for section in self.recognize_code(&text) {
                if let Some(code) = section {
                    if let Ok(c) = self.maps.int_to_char(code) {
                        output.push(c);
                    } else {
                        output.push('�');
                    }
                } else {
                    output.push('�');
                }
            }
        } else if self.mode == IOMode::Word {
            for section in self.recognize_code(&text) {
                if let Some(code) = section {
                    if let Ok(s) = self.maps.int_to_word(code) {
                        output.push_str(s);
                        output.push(' ');
                    } else {
                        output.push_str("� ");
                    }
                } else {
                    output.push_str("� ");
                }
            }
            output.pop();
        } else {
            for section in self.recognize_code(&text) {
                if let Some(code) = section {
                    output.push_str(&code.to_string());
                    output.push(' ');
                } else {
                    output.push_str("� ");
                }
            }
        }

        Ok(output)
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
