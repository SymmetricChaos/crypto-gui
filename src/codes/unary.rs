use super::{Code, LetterAndWordCode};
use crate::errors::Error;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnaryMode {
    Letter,
    Word,
}

pub struct UnaryCode {
    pub code: LetterAndWordCode<String>,
    pub mode: UnaryMode,
}

impl UnaryCode {
    pub fn set_letter_map(&mut self) {
        self.code.set_letter_map(|(n, _)| "1".repeat(n) + "0")
    }

    pub fn set_word_map(&mut self) {
        self.code.set_word_map(|(n, _)| "1".repeat(n) + "0")
    }
}

impl Default for UnaryCode {
    fn default() -> Self {
        let mut code = LetterAndWordCode::<String>::default();
        code.alphabet = String::from("ETAOINSHRDLCUMWFGYPBVKJXQZ");
        code.set_letter_map(|(n, _)| "1".repeat(n) + "0");
        UnaryCode {
            code,
            mode: UnaryMode::Letter,
        }
    }
}

impl Code for UnaryCode {
    fn encode(&self, text: &str) -> Result<String, Error> {
        if self.mode == UnaryMode::Letter {
            let mut output = String::new();
            for s in text.chars() {
                let code = self
                    .code
                    .letter_map
                    .get_by_left(&s)
                    .ok_or_else(|| Error::invalid_input_char(s))?;
                output.push_str(&code)
            }
            Ok(output)
        } else {
            let mut output = String::new();
            for w in text.split(" ") {
                let code = self
                    .code
                    .word_map
                    .get_by_left(w)
                    .ok_or_else(|| Error::invalid_input_group(w))?;
                output.push_str(code)
            }
            Ok(output)
        }
    }

    fn decode(&self, text: &str) -> Result<String, Error> {
        let mut output = String::new();
        let mut buffer = String::with_capacity(self.code.letter_map.len());
        if self.mode == UnaryMode::Letter {
            for b in text.chars() {
                buffer.push(b);
                if b == '0' {
                    match self.code.letter_map.get_by_right(&buffer) {
                        Some(s) => {
                            output.push(*s);
                            buffer.clear();
                        }
                        None => {
                            output.push('�');
                            buffer.clear();
                        }
                    }
                }
            }
        } else {
            for b in text.chars() {
                buffer.push(b);
                if b == '0' {
                    match self.code.word_map.get_by_right(&buffer) {
                        Some(s) => {
                            output.push_str(s);
                            buffer.clear();
                        }
                        None => {
                            output.push('�');
                            buffer.clear();
                        }
                    }
                }
            }
        }

        Ok(output)
    }

    fn randomize(&mut self) {}

    fn reset(&mut self) {}
}

#[cfg(test)]
mod unary_tests {
    use super::*;

    const PLAINTEXT: &'static str = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    const ENCODEDTEXT: &'static str = "1011111110011111111111111111111111101111111111110111101111111111101111111111111111111110111111111111111111101111111101110111111111111110111110111111111111111011101111111111111111111111101111111111111111111111011111111111101111111111111011111111111111111101111110111011111111111111111111001111111101011111110011111111110110111111111111111111111111101111111111111111101111111110111011111111111111110";

    #[test]
    fn encrypt_test() {
        let code = UnaryCode::default();
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT);
    }

    #[test]
    fn decrypt_test() {
        let code = UnaryCode::default();
        assert_eq!(code.decode(ENCODEDTEXT).unwrap(), PLAINTEXT);
    }
}
