use crate::{Cipher, CipherError};
use itertools::Itertools;
use lazy_static::lazy_static;
use rand::{thread_rng, Rng};
use regex::Regex;
use std::num::ParseIntError;
use utils::{
    math_functions::is_prime32,
    polynomials::{eval_poly, lagrange},
};

// https://en.wikipedia.org/wiki/Shamir%27s_secret_sharing

lazy_static! {
    pub static ref PAIRS: Regex = Regex::new(r"\((\d+),\s*(\d+)\)+").unwrap();
    pub static ref NUMBER: Regex = Regex::new(r"(\d+)").unwrap();
}

pub struct ShamirSecretSharing {
    pub shares: u32,
    pub threshold: u32,
    pub polynomial: Vec<u32>, // The constant coefficient of the polynomial is the secret
    pub modulus: u32,
    pub random_shares: bool,
}

impl Default for ShamirSecretSharing {
    fn default() -> Self {
        Self {
            shares: 3,
            threshold: 3,
            polynomial: vec![0, 65, 2347, 542],
            modulus: 4294967029,
            random_shares: true,
        }
    }
}

impl ShamirSecretSharing {
    pub fn sting_to_vec(&mut self, text: &str) -> Result<(), ParseIntError> {
        self.polynomial.clear();
        let groups = text.split(",");
        self.polynomial.push(0);
        for group in groups {
            match u32::from_str_radix(group.trim(), 10) {
                Ok(n) => self.polynomial.push(n),
                Err(e) => {
                    self.polynomial.clear();
                    return Err(e);
                }
            }
        }
        Ok(())
    }

    fn check_state(&self) -> Result<(), CipherError> {
        if self.modulus < 1 {
            return Err(CipherError::state("modulus must be positive"));
        }
        if !is_prime32(self.modulus) {
            return Err(CipherError::state("modulus must be prime"));
        }
        if self.threshold < 2 {
            return Err(CipherError::state("threshold must be at least 3"));
        }
        if self.threshold > self.modulus {
            return Err(CipherError::state(
                "threshold must be less than the order of the field",
            ));
        }
        if self.shares < 2 {
            return Err(CipherError::state("there must be at least 3 shares"));
        }
        if self.threshold > self.shares {
            return Err(CipherError::state(
                "cannot require a greater threshold than shares",
            ));
        };
        Ok(())
    }
}

impl Cipher for ShamirSecretSharing {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        self.check_state()?;

        if self.polynomial.len() != (self.threshold - 1) as usize {
            return Err(CipherError::State(format!(
                "a threshold of {} requires a polynomial with exactly {} coefficients",
                self.threshold,
                self.threshold - 2
            )));
        }

        let secret =
            u32::from_str_radix(text, 10).map_err(|e| CipherError::Input(e.to_string()))?;

        let p = {
            let mut p = self.polynomial.clone();
            p[0] = secret;
            p
        };

        let mut out = Vec::with_capacity(self.shares as usize);

        if self.random_shares {
            let mut rng = thread_rng();
            let mut used = Vec::with_capacity(self.threshold as usize);

            for _ in 0..self.shares {
                let x = {
                    loop {
                        let t = rng.gen_range(1..self.modulus);
                        if !used.contains(&t) {
                            used.push(t);
                            break t;
                        }
                    }
                };
                let y = u32::try_from(eval_poly(x, &p, self.modulus, true))
                    .expect("conversion from BigInt to u32 failed");
                out.push((x, y))
            }
        } else {
            for x in 1..=self.shares {
                let y = u32::try_from(eval_poly(x, &p, self.modulus, true))
                    .expect("conversion from BigInt to u32 failed");
                out.push((x, y))
            }
        }

        Ok(out.iter().map(|p| format!("{p:?}")).join(", "))
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        self.check_state()?;

        let mut pairs = Vec::new();
        for p in PAIRS.captures_iter(text) {
            let x =
                u32::from_str_radix(&p[1], 10).map_err(|e| CipherError::Input(e.to_string()))?;
            let y =
                u32::from_str_radix(&p[2], 10).map_err(|e| CipherError::Input(e.to_string()))?;
            pairs.push((x, y));
        }

        if pairs.len() < self.threshold as usize {
            return Err(CipherError::Input(format!(
                "threshold requires at least {} pairs",
                self.threshold
            )));
        }

        match lagrange(0, &pairs, self.modulus) {
            Some(n) => Ok(format!("{n}")),
            None => Err(CipherError::input("Lagrange interpolation failed")),
        }
    }
}

#[cfg(test)]
mod shamir_tests {

    use super::*;

    const PLAINTEXT: &'static str = "1234";
    const CIPHERTEXT: &'static str = "(1, 1494), (2, 329), (3, 965), (4, 176), (5, 1188), (6, 775)";

    // #[test]
    // fn capture_test() {
    //     for p in PAIRS.captures_iter(CIPHERTEXT) {
    //         println!("{} {}", &p[1], &p[2])
    //     }
    // }

    #[test]
    fn encrypt_test() {
        let mut cipher = ShamirSecretSharing::default();
        cipher.modulus = 1613;
        cipher.polynomial = vec![166, 94];
        cipher.shares = 6;
        cipher.threshold = 4;
        cipher.random_shares = false;
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT)
    }

    #[test]
    fn decrypt_test() {
        let mut cipher = ShamirSecretSharing::default();
        cipher.modulus = 1613;
        cipher.polynomial = vec![166, 94];
        cipher.shares = 6;
        cipher.threshold = 4;
        cipher.random_shares = false;
        assert_eq!(cipher.decrypt(CIPHERTEXT).unwrap(), PLAINTEXT)
    }
}
