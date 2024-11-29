use itertools::Itertools;
use lazy_static::lazy_static;
use num::Integer;
use regex::Regex;

use crate::{errors::CodeError, traits::Code};

use super::string_to_u64s;

// All the primorials that fit in a u64 (1 excluded)
const PRIMORIALS: [u64; 16] = [
    1,
    2,
    6,
    30,
    210,
    2310,
    30030,
    510510,
    9699690,
    223092870,
    6469693230,
    200560490130,
    7420738134810,
    304250263527210,
    13082761331670030,
    614889782588491410,
];

const PRIMES: [u64; 15] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];

lazy_static! {
    pub static ref TUPLE: Regex = Regex::new(r"(([0-9]+:)*[0-9]+)").unwrap();
}

pub fn encode_u64(mut n: u64) -> String {
    if n == 0 {
        return String::from("0");
    }
    let mut out = Vec::new();
    let mut ctr = 0;
    while n != 0 {
        let (q, r) = n.div_rem(&PRIMES[ctr]);
        out.push(r);
        n = q;
        ctr += 1;
    }
    out.iter().rev().join(":")
}

pub fn decode_u64(s: &str) -> Option<u64> {
    let mut ctr = 0;
    let mut value = 0;
    for n in s.rsplit(":").filter(|s| !s.is_empty()) {
        let x = u64::from_str_radix(n.trim(), 10)
            .expect("captures by the regex should always be valid numbers");
        if x >= PRIMES[ctr] {
            return None;
        }
        value += x * PRIMORIALS[ctr];
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

pub struct Primorial {
    pub sep: String,
}

impl Default for Primorial {
    fn default() -> Self {
        Primorial {
            sep: String::from(", "),
        }
    }
}

impl Primorial {}

impl Code for Primorial {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        let mut v = Vec::new();

        for n in string_to_u64s(text, &self.sep)? {
            v.push(encode_u64(n));
        }

        Ok(v.join(&self.sep))
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

        Ok(v.join(&self.sep))
    }
}

#[cfg(test)]
mod factoradic_tests {
    use super::*;

    const PLAINTEXT: &'static str = "0, 1, 2, 3, 4, 5, 6, 7, 2100";
    const ENCODEDTEXT: &'static str = "0, 1, 1:0, 1:1, 2:0, 2:1, 1:0:0, 1:0:1, 10:0:0:0:0";

    #[test]
    fn encode_test() {
        let code = Primorial::default();
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT);
    }

    #[test]
    fn decode_test() {
        let code = Primorial::default();
        assert_eq!(code.decode(ENCODEDTEXT).unwrap(), PLAINTEXT);
    }
}
