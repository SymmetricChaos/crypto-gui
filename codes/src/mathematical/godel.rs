use itertools::Itertools;
use num::{BigUint, Integer, Num, One};
use primal::Primes;

use crate::{
    errors::CodeError,
    letter_word_code::{IOMode, IntegerCodeMaps},
    traits::Code,
};

const MESSAGE_LIMIT: usize = 50;

pub struct Godel {
    pub maps: IntegerCodeMaps,
    primes: Vec<usize>,
    pub mode: IOMode,
}

impl Default for Godel {
    fn default() -> Self {
        let mut maps = IntegerCodeMaps::new();
        maps.set_words(
            "0, s, +, ×, =, (, ), implies, not, forall, exists, and, or, x1, P1, x2, P2, x3, P3, x4, P4, x5, P5",
        );
        maps.alphabet = String::from("ETAOINSHRDLCUMWFGYPBVKJXQZ");

        let primes = Primes::all().take(MESSAGE_LIMIT).collect_vec();

        Self {
            primes,
            maps,
            mode: IOMode::Word,
        }
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

        if self.mode == IOMode::Letter {
            for (c, prime) in text.chars().zip(self.primes.iter()) {
                match self.maps.char_to_int(c) {
                    Ok(v) => out *= BigUint::from(*prime).pow((v + 1) as u32),
                    Err(e) => return Err(e),
                }
            }
        } else if self.mode == IOMode::Word {
            for (s, prime) in text.split(" ").zip(self.primes.iter()) {
                match self.maps.word_to_int(s) {
                    Ok(v) => out *= BigUint::from(*prime).pow((v + 1) as u32),
                    Err(e) => return Err(e),
                }
            }
        } else {
            return Err(CodeError::state(
                "Godel encoding is not currently defined for IOMode::Integer",
            ));
        }
        return Ok(out.to_str_radix(10));
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        let mut num = match BigUint::from_str_radix(text, 10) {
            Ok(n) => n,
            Err(_) => return Err(CodeError::Input("unable to parse input as a number".into())),
        };

        if self.mode == IOMode::Word {
            let mut words = Vec::with_capacity(MESSAGE_LIMIT);
            for p in self.primes.iter() {
                let mut ctr = 0;
                let big_p = BigUint::from(*p);
                while num.is_multiple_of(&big_p) {
                    ctr += 1;
                    num = num.div_floor(&big_p)
                }
                if ctr != 0 {
                    let c = match self.maps.int_to_word(ctr - 1) {
                        Ok(c) => c,
                        Err(_) => "�",
                    };
                    words.push(c);
                }
                if ctr == 0 {
                    words.push("�")
                }
                if num.is_one() {
                    break;
                }
            }
            Ok(words.iter().join(" "))
        } else if self.mode == IOMode::Letter {
            let mut letters = Vec::with_capacity(MESSAGE_LIMIT);
            for p in self.primes.iter() {
                let mut ctr = 0;
                let big_p = BigUint::from(*p);
                while num.is_multiple_of(&big_p) {
                    ctr += 1;
                    num = num.div_floor(&big_p)
                }
                if ctr != 0 {
                    let c = match self.maps.int_to_char(ctr - 1) {
                        Ok(c) => c,
                        Err(_) => '�',
                    };
                    letters.push(c);
                }
                if ctr == 0 {
                    letters.push('�')
                }
                if num.is_one() {
                    break;
                }
            }
            Ok(letters.iter().collect())
        } else {
            Err(CodeError::state(
                "Godel encoding is not currently defined for IOMode::Integer",
            ))
        }
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
