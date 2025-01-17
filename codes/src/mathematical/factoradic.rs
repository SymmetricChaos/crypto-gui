use itertools::Itertools;
use lazy_static::lazy_static;
use num::Integer;
use regex::Regex;

use crate::{errors::CodeError, traits::Code};

use super::string_to_u64s;

lazy_static! {
    pub static ref TUPLE: Regex = Regex::new(r"(([0-9]+:)*[0-9]+)").unwrap();
}

pub fn encode_u64(mut n: u64) -> String {
    if n == 0 {
        return String::from("0");
    }
    let mut out = Vec::new();
    let mut divisor = 1;
    while n != 0 {
        let (q, r) = n.div_rem(&divisor);
        out.push(r);
        n = q;
        divisor += 1;
    }
    out.iter().rev().join(":")
}

pub fn decode_u64(s: &str) -> Option<u64> {
    let mut base = 1;
    let mut ctr = 1;
    let mut value = 0;
    for n in s.rsplit(":").filter(|s| !s.is_empty()) {
        let x = u64::from_str_radix(n.trim(), 10)
            .expect("captures by the regex should always be valid numbers");
        if x >= ctr {
            return None;
        }
        value += x * base;
        base *= ctr;
        ctr += 1;
    }
    Some(value)
}

pub fn extract_code_groups(text: &str) -> Vec<Option<u64>> {
    let mut output = Vec::new();

    for cap in TUPLE.captures_iter(text) {
        let s = match cap.get(1) {
            Some(m) => m.as_str(),
            None => {
                output.push(None);
                continue;
            }
        };
        output.push(decode_u64(s));
    }

    output
}

pub struct Factoradic {}

impl Default for Factoradic {
    fn default() -> Self {
        Factoradic {}
    }
}

impl Factoradic {}

impl Code for Factoradic {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        let mut v = Vec::new();

        for n in string_to_u64s(text, ",")? {
            v.push(encode_u64(n));
        }

        Ok(v.join(", "))
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        let mut v = Vec::new();

        for group in extract_code_groups(text) {
            if let Some(n) = group {
                v.push(n.to_string());
            } else {
                v.push(String::from("INVALID"));
            }
        }

        Ok(v.join(", "))
    }
}

#[cfg(test)]
mod factoradic_tests {
    use super::*;

    const PLAINTEXT: &'static str = "0, 1, 2, 3, 4, 5";
    const ENCODEDTEXT: &'static str = "0, 1:0, 1:0:0, 1:1:0, 2:0:0, 2:1:0";

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
        let code = Factoradic::default();
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT);
    }

    #[test]
    fn decode_test() {
        let code = Factoradic::default();
        assert_eq!(code.decode(ENCODEDTEXT).unwrap(), PLAINTEXT);
    }
}
