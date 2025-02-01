use itertools::Itertools;
use num::{BigUint, Integer, Num, One};
use primal::Primes;

use crate::{errors::CodeError, traits::Code};

const MESSAGE_LIMIT: usize = 100;

pub struct Godel {
    pub words: Vec<String>,
    primes: Vec<usize>,
}

impl Default for Godel {
    fn default() -> Self {
        let words = [
            "0", "s", "+", "×", "=", "(", ")", "implies", "not", "forall", "exists", "and", "or",
            "x1", "P1", "x2", "P2", "x3", "P3", "x4", "P4", "x5", "P5",
        ]
        .into_iter()
        .map(|s| s.to_string())
        .collect_vec();

        let primes = Primes::all().take(MESSAGE_LIMIT).collect_vec();

        Self { words, primes }
    }
}

impl Godel {
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

impl Code for Godel {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        if text.split(" ").count() > MESSAGE_LIMIT {
            return Err(CodeError::Input(format!(
                "The Godel encoding is currently limited to {} code points",
                MESSAGE_LIMIT
            )));
        }
        let mut out = BigUint::one();

        for (s, prime) in text.split(" ").map(|s| s.trim()).zip(self.primes.iter()) {
            match self
                .words
                .iter()
                .position(|x| x == s)
                .ok_or_else(|| CodeError::invalid_input_group(s))
            {
                Ok(v) => out *= BigUint::from(*prime).pow((v + 1) as u32),
                Err(e) => return Err(e),
            }
        }

        return Ok(out.to_str_radix(10));
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        let mut num = match BigUint::from_str_radix(text, 10) {
            Ok(n) => n,
            Err(_) => return Err(CodeError::Input("unable to parse input as a number".into())),
        };

        let mut words = Vec::with_capacity(MESSAGE_LIMIT);
        for p in self.primes.iter() {
            let mut ctr = 0;
            let big_p = BigUint::from(*p);
            while num.is_multiple_of(&big_p) {
                ctr += 1;
                num = num.div_floor(&big_p)
            }
            if ctr != 0 {
                match self.words.get(ctr - 1) {
                    Some(c) => words.push(c.to_string()),
                    None => words.push(String::from("�")),
                };
            }
            if ctr == 0 {
                words.push(String::from("�"))
            }
            if num.is_one() {
                break;
            }
        }
        Ok(words.iter().join(" "))
    }
}

#[cfg(test)]
mod godel_tests {
    use super::*;

    const PLAINTEXT: &'static str = "0 s +";
    const ENCODEDTEXT: &'static str = "2250";

    #[test]
    fn encode_test() {
        let code = Godel::default();
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT);
    }

    #[test]
    fn decode_test() {
        let code = Godel::default();
        assert_eq!(code.decode(ENCODEDTEXT).unwrap(), PLAINTEXT);
    }
}
