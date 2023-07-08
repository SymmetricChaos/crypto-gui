use std::num::ParseIntError;

use crate::{Cipher, CipherError};
use itertools::Itertools;
use lazy_static::lazy_static;
use rand::{thread_rng, Rng};
use regex::Regex;
use utils::math_functions::{eval_poly, is_prime32, modular_division};

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
            shares: 4,
            threshold: 4,
            polynomial: vec![65, 2347, 542],
            modulus: 4294967029,
            random_shares: true,
        }
    }
}

impl ShamirSecretSharing {
    pub fn sting_to_vec(&mut self, text: &str) -> Result<(), ParseIntError> {
        self.polynomial.clear();
        let groups = text.split(",");
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

    fn lagrange(&self, x: u32, pairs: Vec<(u32, u32)>) -> Option<u32> {
        let mut nums: Vec<u32> = Vec::new();
        let mut dens = Vec::new();

        for i in 0..pairs.len() {
            let mut others = pairs.iter().map(|(x, _)| *x).collect_vec();
            let cur = others.remove(i);
            nums.push(others.iter().map(|e| x - *e).product());
            dens.push(others.iter().map(|e| cur - *e).product());
        }

        let denominator = dens.iter().product::<u32>() % self.modulus;

        let numerator = {
            let mut n = 0;
            for i in 0..pairs.len() {
                n += modular_division(
                    (nums[i] * denominator * pairs[i].1) % self.modulus,
                    dens[i],
                    self.modulus,
                )?;
            }
            n %= self.modulus;
            n
        };

        Some(
            (modular_division(numerator, denominator, self.modulus)? + self.modulus) % self.modulus,
        )
    }

    fn check_state(&self) -> Result<(), CipherError> {
        if self.modulus < 1 {
            return Err(CipherError::state("modulus must be positive"));
        }
        if !is_prime32(self.modulus) {
            return Err(CipherError::state("modulus must be prime"));
        }
        if self.threshold < 3 {
            return Err(CipherError::state("threshold must be at least 3"));
        }
        if self.threshold > self.modulus {
            return Err(CipherError::state(
                "threshold must be less than the order of the field",
            ));
        }
        if self.shares < 3 {
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

        if self.polynomial.len() != (self.threshold - 2) as usize {
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
            p.insert(0, secret);
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

        match self.lagrange(0, pairs) {
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
