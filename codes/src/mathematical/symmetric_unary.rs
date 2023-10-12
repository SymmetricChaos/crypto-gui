use utils::text_functions::swap_ab;

use crate::{
    errors::CodeError,
    letter_word_code::{IOMode, LetterWordIntCode},
    traits::Code,
};

pub struct UnaryCode {
    pub maps: LetterWordIntCode,
    pub mode: IOMode,
    pub invert: bool,
}

impl UnaryCode {
    pub fn encode_usize(&self, n: usize) -> String {
        if n == 0 {
            return String::from("1");
        } else {
            format!("0{}0", "1".repeat(n - 1))
        }
    }

    pub fn recognize_code(&self, text: &str) -> Vec<String> {
        let mut output = Vec::new();
        let mut buffer = String::with_capacity(self.maps.alphabet.chars().count());

        for b in text.chars() {
            // Invalid characters immediatly give '?' response and restart
            if b != '0' && b != '1' {
                output.push(String::from('?'));
                buffer.clear();
                continue;
            }
            // The '1' bit on its own is a valid code
            if buffer.is_empty() && b == '1' {
                output.push(String::from("1"));
                continue;
            }
            // If the starting bit is '0' push it and continue
            if buffer.is_empty() && b == '0' {
                buffer.push(b);
            // Otherwise push the next bit on
            } else {
                buffer.push(b);
                if b == '0' {
                    output.push(buffer.clone());
                    buffer.clear();
                }
            }
        }
        // If anything remains in the buffer it is unknown
        if !buffer.is_empty() {
            output.push(String::from('?'))
        }
        output
    }
}

impl Default for UnaryCode {
    fn default() -> Self {
        let mut maps = LetterWordIntCode::new();
        maps.alphabet = String::from("ETAOINSHRDLCUMWFGYPBVKJXQZ");

        UnaryCode {
            maps,
            mode: IOMode::Letter,
            invert: false,
        }
    }
}

impl Code for UnaryCode {
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
        // let mut output = String::new();
        // let text = if self.invert {
        //     swap_ab('0', '1', text)
        // } else {
        //     text.to_string()
        // };
        // if self.mode == IOMode::Letter {
        //     for code in self.recognize_code(&text) {
        //         match self.maps.word_to_int(&code) {
        //             Some(s) => {
        //                 output.push(*s);
        //                 buffer.clear();
        //             }
        //             None => {
        //                 output.push('?');
        //                 buffer.clear();
        //             }
        //         }
        //     }
        // } else if self.mode == IOMode::Word {
        //     for code in self.recognize_code(&text) {
        //         match self.maps.word_map.get_by_right(&code) {
        //             Some(s) => {
        //                 output.push_str(s);
        //                 output.push(' ');
        //                 buffer.clear();
        //             }
        //             None => {
        //                 output.push_str("? ");
        //                 buffer.clear();
        //             }
        //         }
        //     }
        //     output.pop();
        // } else {
        //     for code in self.recognize_code(&text) {
        //         if code == "?" {
        //             output.push_str("? ")
        //         } else {
        //             output.push_str(&format!("{} ", code.chars().count()))
        //         }
        //     }
        // }

        // Ok(output)
        todo!()
    }
}

#[cfg(test)]
mod symmetric_unary_tests {
    use super::*;

    const PLAINTEXT: &'static str = "ETAO";
    const ENCODEDTEXT: &'static str = "1000100110";

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
