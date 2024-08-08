use crate::{errors::CodeError, traits::Code};

const MAX_LENGTH: usize = 1000;

pub struct UnaryCode {
    invert: bool,
}

impl Default for UnaryCode {
    fn default() -> Self {
        Self { invert: false }
    }
}

impl UnaryCode {
    pub fn encode_int(&self, n: usize) -> String {
        if self.invert {
            "0".repeat(n) + "1"
        } else {
            "1".repeat(n) + "0"
        }
    }

    pub fn recognize_codes(&self, text: &str) -> Vec<Option<usize>> {
        let mut output = Vec::new();

        let (z0, z1) = if self.invert { ('1', '0') } else { ('0', '1') };

        let mut ctr = 0;
        for b in text.chars() {
            if b == z0 {
                ctr += 1
            } else if b == z1 {
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
        let mut out = String::new();
        for w in text.split(" ") {
            let n = usize::from_str_radix(w, 10).map_err(|e| CodeError::Input(e.to_string()))?;
            if n > MAX_LENGTH {
                return Err(CodeError::Input(format!(
                    "Unary codes are limited to {} for encoding to avoid massive memory usage",
                    MAX_LENGTH
                )));
            }
            out.push_str(&self.encode_int(n))
        }

        Ok(out)
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        let mut out = String::new();

        for section in self.recognize_codes(&text) {
            if let Some(code) = section {
                out.push_str(&code.to_string());
                out.push(' ');
            } else {
                out.push_str("� ");
            }
        }
        Ok(out)
    }
}

// #[cfg(test)]
// mod unary_tests {
//     use super::*;

//     const PLAINTEXT: &'static str = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
//     const ENCODEDTEXT: &'static str = "1011111110011111111111111111111111101111111111110111101111111111101111111111111111111110111111111111111111101111111101110111111111111110111110111111111111111011101111111111111111111111101111111111111111111111011111111111101111111111111011111111111111111101111110111011111111111111111111001111111101011111110011111111110110111111111111111111111111101111111111111111101111111110111011111111111111110";

//     #[test]
//     fn encode_test() {
//         let code = UnaryCode::default();
//         assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT);
//     }

//     #[test]
//     fn decode_test() {
//         let code = UnaryCode::default();
//         assert_eq!(code.decode(ENCODEDTEXT).unwrap(), PLAINTEXT);
//     }
// }
