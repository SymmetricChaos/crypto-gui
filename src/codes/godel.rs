use super::Code;
use crate::errors::Error;
use bimap::BiMap;
use itertools::Itertools;
use num::{BigUint, Integer, Num, One};
use primal::Primes;

const MESSAGE_LIMIT: usize = 50;

pub struct Godel {
    words: Vec<String>,
    words_string: String,
    sep: String,
    primes: Vec<usize>,
    map: BiMap<String, usize>,
}

impl Godel {
    // fn _print_mapping(&self) {
    //     for s in self.words.iter() {
    //         println!("{} {}", s, self.map.get_by_left(s).unwrap())
    //     }
    // }

    pub fn control_words(&mut self) -> &mut String {
        self.words = self
            .words_string
            .split(&self.sep)
            .map(|w| w.to_string())
            .collect_vec();
        for (n, c) in self.words.iter().enumerate() {
            self.map.insert(c.clone(), n + 1);
        }
        &mut self.words_string
    }

    pub fn chars_codes(&self) -> impl Iterator<Item = (&usize, &String)> + '_ {
        self.words
            .iter()
            .map(|x| (self.map.get_by_left(x).unwrap(), x))
    }
}

impl Default for Godel {
    fn default() -> Self {
        let words_string =
            String::from("0 s + × = ( ) implies not forall exists and or x1 x2 x3 x4 x5");

        let sep = String::from(" ");
        let words = words_string
            .split(&sep)
            .map(|w| w.to_string())
            .collect_vec();

        let primes = Primes::all().take(MESSAGE_LIMIT).collect_vec();
        let mut map = BiMap::new();
        for (n, c) in words.iter().cloned().enumerate() {
            map.insert(c, n + 1);
        }
        Self {
            words,
            words_string,
            sep,
            primes,
            map,
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
        for (c, prime) in text.split(&self.sep).zip(self.primes.iter()) {
            match self.map.get_by_left(c) {
                Some(v) => out *= BigUint::from(*prime).pow(*v as u32),
                None => return Err(Error::invalid_input_group(c)),
            }
        }
        Ok(out.to_str_radix(10))
    }

    fn decode(&self, text: &str) -> Result<String, Error> {
        let mut num = match BigUint::from_str_radix(text, 10) {
            Ok(n) => n,
            Err(_) => return Err(Error::Input("unable to parse input as a number".into())),
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
                let c = match self.map.get_by_right(&ctr) {
                    Some(c) => c,
                    None => {
                        return Err(Error::Input(
                            "exponent does not map to a symnol in the alpabet".into(),
                        ))
                    }
                };
                words.push(c);
            }
        }
        Ok(words.iter().join(&self.sep))
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
