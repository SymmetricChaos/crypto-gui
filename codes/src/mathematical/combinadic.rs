use itertools::Itertools;
use lazy_static::lazy_static;
use num::integer::binomial;
use regex::Regex;

use crate::{errors::CodeError, traits::Code};

lazy_static! {
    pub static ref TUPLE: Regex = Regex::new(r"(([0-9]+:)*[0-9]+)").unwrap();
}

pub struct Combinadic {
    pub k: u64,
}

impl Default for Combinadic {
    fn default() -> Self {
        Self { k: 5 }
    }
}

impl Combinadic {
    pub fn encode_u64(&self, n: u64) -> String {
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

    fn code_string_to_num(s: &str) -> Option<u64> {
        // let mut base = 1;
        // let mut ctr = 1;
        // let mut value = 0;
        // for n in s.rsplit(":").filter(|s| !s.is_empty()) {
        //     let x = u64::from_str_radix(n.trim(), 10)
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

    pub fn recognize_code(text: &str) -> Vec<Option<u64>> {
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

impl Code for Combinadic {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        let mut output = String::new();

        for w in text.split(" ") {
            let n = u64::from_str_radix(w, 10).map_err(|e| CodeError::Input(e.to_string()))?;
            output.push_str(&self.encode_u64(n));
            output.push(' ');
        }

        output.pop();
        Ok(output)
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        // let mut output = String::new();

        //     for section in Self::recognize_code(&text) {
        //         if let Some(code) = section {
        //             output.push_str(&code.to_string());
        //             output.push(' ');
        //         } else {
        //             output.push_str("� ");
        //         }
        //     }
        // }

        // Ok(output)
        todo!()
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
