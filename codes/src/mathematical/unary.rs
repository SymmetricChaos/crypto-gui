use crate::{
    errors::CodeError,
    traits::{Code, IOMode, LetterAndWordCode},
};

pub struct UnaryCode {
    pub maps: LetterAndWordCode<String>,
    pub mode: IOMode,
    pub invert: bool,
    pub symmetric: bool,
}

impl UnaryCode {
    pub fn set_letter_map(&mut self) {
        self.maps.set_letter_map(|(n, _)| "1".repeat(n) + "0")
    }

    pub fn set_word_map(&mut self) {
        self.maps.set_word_map(|(n, _)| "1".repeat(n) + "0")
    }

    pub fn usize_to_unary(&self, n: usize) -> String {
        "1".repeat(n) + "0"
    }

    pub fn swap_01(s: String) -> String {
        s.chars()
            .map(|c| {
                if c == '0' {
                    '1'
                } else if c == '1' {
                    '0'
                } else {
                    c
                }
            })
            .collect()
    }
}

impl Default for UnaryCode {
    fn default() -> Self {
        let mut maps = LetterAndWordCode::<String>::default();
        maps.alphabet = String::from("ETAOINSHRDLCUMWFGYPBVKJXQZ");
        maps.set_letter_map(|(n, _)| "1".repeat(n) + "0");
        UnaryCode {
            maps,
            mode: IOMode::Letter,
            invert: false,
            symmetric: false,
        }
    }
}

impl Code for UnaryCode {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        let mut output = String::new();

        if self.mode == IOMode::Letter {
            for c in text.chars() {
                let code = self.maps.get_by_letter(c)?;
                output.push_str(&code)
            }
        } else if self.mode == IOMode::Word {
            for w in text.split(" ") {
                let code = self.maps.get_by_word(w)?;
                output.push_str(code)
            }
        } else {
            for w in text.split(" ") {
                let n =
                    usize::from_str_radix(w, 10).map_err(|e| CodeError::Input(e.to_string()))?;
                output.push_str(&"1".repeat(n));
                output.push('0');
            }
        }
        if self.invert {
            Ok(Self::swap_01(output))
        } else {
            Ok(output)
        }
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        let mut output = String::new();
        let mut buffer = String::with_capacity(self.maps.letter_map.len());
        let text = if self.invert {
            Self::swap_01(text.to_string())
        } else {
            text.to_string()
        };
        if self.mode == IOMode::Letter {
            for b in text.chars() {
                buffer.push(b);
                if b == '0' {
                    match self.maps.letter_map.get_by_right(&buffer) {
                        Some(s) => {
                            output.push(*s);
                            buffer.clear();
                        }
                        None => {
                            output.push('?');
                            buffer.clear();
                        }
                    }
                }
            }
        } else if self.mode == IOMode::Word {
            for b in text.chars() {
                buffer.push(b);
                if b == '0' {
                    match self.maps.word_map.get_by_right(&buffer) {
                        Some(s) => {
                            output.push_str(s);
                            output.push(' ');
                            buffer.clear();
                        }
                        None => {
                            output.push('?');
                            buffer.clear();
                        }
                    }
                }
            }
            output.pop();
        } else {
            let mut ctr = 0;
            for b in text.chars() {
                if b == '1' {
                    ctr += 1
                } else if b == '0' {
                    output.push_str(&ctr.to_string());
                    output.push(' ');
                    ctr = 0;
                } else {
                    output.push_str("? ");
                    ctr = 0;
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
    fn encode_test() {
        let code = UnaryCode::default();
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT);
    }

    #[test]
    fn decode_test() {
        let code = UnaryCode::default();
        assert_eq!(code.decode(ENCODEDTEXT).unwrap(), PLAINTEXT);
    }
}
