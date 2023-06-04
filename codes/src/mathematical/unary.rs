use crate::{
    errors::CodeError,
    traits::{Code, LetterAndWordCode},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnaryMode {
    Letter,
    Word,
}

pub struct UnaryCode {
    pub maps: LetterAndWordCode<String>,
    pub mode: UnaryMode,
}

impl UnaryCode {
    pub fn set_letter_map(&mut self) {
        self.maps.set_letter_map(|(n, _)| "1".repeat(n) + "0")
    }

    pub fn set_word_map(&mut self) {
        self.maps.set_word_map(|(n, _)| "1".repeat(n) + "0")
    }
}

impl Default for UnaryCode {
    fn default() -> Self {
        let mut maps = LetterAndWordCode::<String>::default();
        maps.alphabet = String::from("ETAOINSHRDLCUMWFGYPBVKJXQZ");
        maps.set_letter_map(|(n, _)| "1".repeat(n) + "0");
        UnaryCode {
            maps,
            mode: UnaryMode::Letter,
        }
    }
}

impl Code for UnaryCode {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        if self.mode == UnaryMode::Letter {
            let mut output = String::new();
            for c in text.chars() {
                let code = self.maps.get_by_letter(c)?;
                output.push_str(&code)
            }
            Ok(output)
        } else {
            let mut output = String::new();
            for w in text.split(" ") {
                let code = self.maps.get_by_word(w)?;
                output.push_str(code)
            }
            Ok(output)
        }
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        let mut output = String::new();
        let mut buffer = String::with_capacity(self.maps.letter_map.len());
        if self.mode == UnaryMode::Letter {
            for b in text.chars() {
                buffer.push(b);
                if b == '0' {
                    match self.maps.letter_map.get_by_right(&buffer) {
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
                    match self.maps.word_map.get_by_right(&buffer) {
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
