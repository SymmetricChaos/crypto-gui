use std::collections::HashMap;

use super::Code;
use crate::errors::Error;
use itertools::Itertools;
use num::{BigUint, Integer, Num, One};
use primal::Primes;

const MESSAGE_LIMIT: usize = 50;

pub struct Godel {
    alphabet: String,
    primes: Vec<usize>,
    map: HashMap<char, usize>,
    map_inv: HashMap<usize, char>,
}

impl Godel {
    fn _print_mapping(&self) {
        for c in self.alphabet.chars() {
            println!("{} {}", c, self.map.get(&c).unwrap())
        }
    }

    pub fn control_alphabet(&mut self) -> &mut String {
        for (n, c) in self.alphabet.chars().enumerate() {
            self.map.insert(c, n + 1);
            self.map_inv.insert(n + 1, c);
        }
        &mut self.alphabet
    }

    pub fn chars_codes(&self) -> impl Iterator<Item = (&usize, char)> + '_ {
        self.alphabet
            .chars()
            .map(|x| (self.map.get(&x).unwrap(), x))
    }
}

impl Default for Godel {
    fn default() -> Self {
        let alphabet = String::from("ETAOINSHRDLCUMWFGYPBVKJXQZ");
        let primes = Primes::all().take(MESSAGE_LIMIT).collect_vec();
        let mut map = HashMap::new();
        let mut map_inv = HashMap::new();
        for (n, c) in alphabet.chars().enumerate() {
            map.insert(c, n + 1);
            map_inv.insert(n + 1, c);
        }
        Self {
            alphabet,
            primes,
            map,
            map_inv,
        }
    }
}

impl Godel {
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

impl Code for Godel {
    fn encode(&self, text: &str) -> Result<String, Error> {
        if text.chars().count() > MESSAGE_LIMIT {
            return Err(Error::Input(format!(
                "The Godel encoding is currently limited to {} characters",
                MESSAGE_LIMIT
            )));
        }
        let mut out = BigUint::one();
        for (c, prime) in text.chars().zip(self.primes.iter()) {
            match self.map.get(&c) {
                Some(v) => out *= BigUint::from(*prime).pow(*v as u32),
                None => {
                    return Err(Error::Input(format!(
                        "The symbol `{}` is not in the alphabet provided",
                        c
                    )))
                }
            }
        }
        Ok(out.to_str_radix(10))
    }

    fn decode(&self, text: &str) -> Result<String, Error> {
        let mut num = match BigUint::from_str_radix(text, 10) {
            Ok(n) => n,
            Err(_) => return Err(Error::Input("unable to parse input as a number".into())),
        };
        let mut characters = Vec::with_capacity(MESSAGE_LIMIT);
        for p in self.primes.iter() {
            let mut ctr = 0;
            let big_p = BigUint::from(*p);
            while num.is_multiple_of(&big_p) {
                ctr += 1;
                num = num.div_floor(&big_p)
            }
            if ctr != 0 {
                let c = match self.map_inv.get(&ctr) {
                    Some(c) => c,
                    None => {
                        return Err(Error::Input(
                            "exponent does not map to a symnol in the alpabet".into(),
                        ))
                    }
                };
                characters.push(*c);
            }
        }
        Ok(characters.iter().collect())
    }

    fn randomize(&mut self) {}

    fn reset(&mut self) {}
}

#[cfg(test)]
mod godel_tests {
    use super::*;

    const PLAINTEXT: &'static str = "THE";
    const ENCODEDTEXT: &'static str = "131220";

    #[test]
    fn encrypt_test() {
        let code = Godel::default();
        //code._print_mapping();
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT);
    }

    #[test]
    fn decrypt_test() {
        let code = Godel::default();
        assert_eq!(code.decode(ENCODEDTEXT).unwrap(), PLAINTEXT);
    }
}
