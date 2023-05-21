use itertools::Itertools;
use num::{BigUint, Integer, Num, One};
use primal::Primes;

use crate::{
    errors::CodeError,
    traits::{Code, IOMode, LetterAndWordCode},
};

const MESSAGE_LIMIT: usize = 50;

pub struct Godel {
    pub maps: LetterAndWordCode<usize>,
    primes: Vec<usize>,
    pub mode: IOMode,
}

impl Godel {
    pub fn set_letter_map(&mut self) {
        self.maps.set_letter_map(|(n, _)| n + 1)
    }

    pub fn set_word_map(&mut self) {
        self.maps.set_word_map(|(n, _)| n + 1)
    }
}

impl Default for Godel {
    fn default() -> Self {
        let mut maps = LetterAndWordCode::<usize>::default();
        maps.words_string = String::from(
            "0, s, +, ×, =, (, ), implies, not, forall, exists, and, or, x1, x2, x3, x4, x5, P1, P2, P3, P4, P5",
        );
        maps.set_word_map(|(n, _)| n + 1);
        maps.alphabet = String::from("ETAOINSHRDLCUMWFGYPBVKJXQZ");
        maps.set_letter_map(|(n, _)| n + 1);

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
            for (s, prime) in text.chars().zip(self.primes.iter()) {
                match self.maps.get_by_letter(s) {
                    Ok(v) => out *= BigUint::from(*prime).pow(*v as u32),
                    Err(e) => return Err(e),
                }
            }
        } else {
            for (s, prime) in text.split(" ").zip(self.primes.iter()) {
                match self.maps.get_by_word(s) {
                    Ok(v) => out *= BigUint::from(*prime).pow(*v as u32),
                    Err(e) => return Err(e),
                }
            }
        }

        Ok(out.to_str_radix(10))
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
                    let c = match self.maps.get_word_by_code(&ctr) {
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
        } else {
            let mut words = Vec::with_capacity(MESSAGE_LIMIT);
            for p in self.primes.iter() {
                let mut ctr = 0;
                let big_p = BigUint::from(*p);
                while num.is_multiple_of(&big_p) {
                    ctr += 1;
                    num = num.div_floor(&big_p)
                }
                if ctr != 0 {
                    let c = match self.maps.get_letter_by_code(&ctr) {
                        Ok(c) => c,
                        Err(_) => &'�',
                    };
                    words.push(*c);
                }
                if ctr == 0 {
                    words.push('�')
                }
                if num.is_one() {
                    break;
                }
            }
            Ok(words.iter().collect())
        }
    }
}

#[cfg(test)]
mod godel_tests {
    use super::*;

    const PLAINTEXT: &'static str = "0 s +";
    const ENCODEDTEXT: &'static str = "2250";

    #[test]
    fn encrypt_test() {
        let code = Godel::default();
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT);
    }

    #[test]
    fn decrypt_test() {
        let code = Godel::default();
        assert_eq!(code.decode(ENCODEDTEXT).unwrap(), PLAINTEXT);
    }
}
