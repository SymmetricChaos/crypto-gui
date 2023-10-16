use itertools::Itertools;
use lazy_static::lazy_static;
use num::integer::binomial;
use regex::Regex;

use crate::{
    errors::CodeError,
    letter_word_code::{IOMode, LetterWordIntCode},
    traits::Code,
};

lazy_static! {
    pub static ref TUPLE: Regex = Regex::new(r"(([0-9]+:)*[0-9]+)").unwrap();
}

pub struct Combinadic {
    pub maps: LetterWordIntCode,
    pub mode: IOMode,
    pub k: usize,
}

impl Combinadic {
    pub fn encode_usize(&self, n: usize) -> String {
        let mut out = Vec::new();
        let mut base = self.k;
        let mut n = n;
        while base != 0 && n != 0 {
            let mut prev = 0;
            let mut c = base - 1;
            loop {
                let coef = binomial(c, base);
                if coef > n {
                    out.push(prev);
                    n -= prev;
                    break;
                }
                prev = coef;
                c += 1;
            }
            base -= 1;
        }
        format!("{}", out.iter().rev().join(":"))
    }

    fn code_string_to_num(s: &str) -> Option<usize> {
        // let mut base = 1;
        // let mut ctr = 1;
        // let mut value = 0;
        // for n in s.rsplit(":").filter(|s| !s.is_empty()) {
        //     let x = usize::from_str_radix(n.trim(), 10)
        //         .expect("captures by the regex should always be valid numbers");
        //     if x >= ctr {
        //         return None;
        //     }
        //     value += binomial(x, base);
        //     base *= ctr;
        //     ctr += 1;
        // }
        // Some(value)
        todo!()
    }

    pub fn recognize_code(text: &str) -> Vec<Option<usize>> {
        let mut output = Vec::new();

        for cap in TUPLE.captures_iter(text) {
            let s = match cap.get(1) {
                Some(m) => m.as_str(),
                None => {
                    output.push(None);
                    continue;
                }
            };
            output.push(Self::code_string_to_num(s));
        }

        output
    }
}

impl Default for Combinadic {
    fn default() -> Self {
        let mut maps = LetterWordIntCode::new();
        maps.alphabet = String::from("ETAOINSHRDLCUMWFGYPBVKJXQZ");
        Self {
            maps,
            mode: IOMode::Letter,
            k: 5,
        }
    }
}

impl Code for Combinadic {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        let mut output = String::new();

        if self.mode == IOMode::Letter {
            for c in text.chars() {
                let n = self.maps.char_to_int(c)?;
                output.push_str(&self.encode_usize(n));
                output.push(' ');
            }
        } else if self.mode == IOMode::Word {
            for w in text.split(" ") {
                let n = self.maps.word_to_int(w)?;
                output.push_str(&self.encode_usize(n));
                output.push(' ');
            }
        } else {
            for w in text.split(" ") {
                let n =
                    usize::from_str_radix(w, 10).map_err(|e| CodeError::Input(e.to_string()))?;
                output.push_str(&self.encode_usize(n));
                output.push(' ');
            }
        }
        output.pop();
        Ok(output)
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        let mut output = String::new();

        if self.mode == IOMode::Letter {
            for section in Self::recognize_code(&text) {
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
            for section in Self::recognize_code(&text) {
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
            for section in Self::recognize_code(&text) {
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
mod combinadic_tests {
    use super::*;

    const PLAINTEXT: &'static str = "ETAOIN";
    const ENCODEDTEXT: &'static str = "";

    #[test]
    #[ignore]
    fn tuple_test() {
        for m in TUPLE.find_iter(ENCODEDTEXT) {
            println!("{}", m.as_str())
        }
        for m in TUPLE.find_iter("10:9:222 0 :1:2:4: ::") {
            println!("{}", m.as_str())
        }
    }

    #[test]
    fn encode_test() {
        let code = Combinadic::default();
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT);
    }

    #[test]
    fn decode_test() {
        let code = Combinadic::default();
        assert_eq!(code.decode(ENCODEDTEXT).unwrap(), PLAINTEXT);
    }
}
