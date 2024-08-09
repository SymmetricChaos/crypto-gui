use crate::{
    errors::CodeError,
    letter_word_code::{IOMode, IntegerCodeMaps},
    traits::Code,
};

pub struct UnaryCode {
    pub maps: IntegerCodeMaps,
    pub mode: IOMode,
    pub invert: bool,
    pub spaced: bool,
}

impl Default for UnaryCode {
    fn default() -> Self {
        let mut maps = IntegerCodeMaps::new();
        maps.alphabet = String::from("ETAOINSHRDLCUMWFGYPBVKJXQZ");
        UnaryCode {
            maps,
            mode: IOMode::Integer,
            invert: false,
            spaced: false,
        }
    }
}

impl UnaryCode {
    pub fn encode_usize(&self, n: usize) -> String {
        if self.invert {
            "0".repeat(n) + "1"
        } else {
            "1".repeat(n) + "0"
        }
    }

    pub fn recognize_code(&self, text: &str) -> Vec<Option<usize>> {
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
}

impl Code for UnaryCode {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        let mut output = String::new();

        if self.mode == IOMode::Letter {
            for c in text.chars() {
                let n = self.maps.char_to_int(c)?;
                output.push_str(&self.encode_usize(n));
                if self.spaced {
                    output.push(' ');
                }
            }
        } else if self.mode == IOMode::Word {
            for w in text.split(" ") {
                let n = self.maps.word_to_int(w)?;
                output.push_str(&self.encode_usize(n));
                if self.spaced {
                    output.push(' ');
                }
            }
        } else {
            for w in text.split(" ") {
                let n =
                    usize::from_str_radix(w, 10).map_err(|e| CodeError::Input(e.to_string()))?;
                output.push_str(&self.encode_usize(n));
                if self.spaced {
                    output.push(' ');
                }
            }
        }
        if self.spaced {
            output.pop();
        }
        Ok(output)
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        let mut output = String::new();

        if self.mode == IOMode::Letter {
            for section in self.recognize_code(&text.replace(" ", "")) {
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
mod unary_tests {
    use super::*;

    const PLAINTEXT: &'static str = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    const ENCODEDTEXT: &'static str = "1011111110011111111111111111111111101111111111110111101111111111101111111111111111111110111111111111111111101111111101110111111111111110111110111111111111111011101111111111111111111111101111111111111111111111011111111111101111111111111011111111111111111101111110111011111111111111111111001111111101011111110011111111110110111111111111111111111111101111111111111111101111111110111011111111111111110";

    #[test]
    fn encode_test() {
        let mut code = UnaryCode::default();
        code.mode = IOMode::Letter;
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT);
    }

    #[test]
    fn decode_test() {
        let mut code = UnaryCode::default();
        code.mode = IOMode::Letter;
        assert_eq!(code.decode(ENCODEDTEXT).unwrap(), PLAINTEXT);
    }
}
