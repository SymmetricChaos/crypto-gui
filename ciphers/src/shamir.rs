use crate::{Cipher, CipherError};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use utils::math_functions::{eval_poly, is_prime32, modular_division};

// https://en.wikipedia.org/wiki/Shamir%27s_secret_sharing

lazy_static! {
    pub static ref PAIRS: Regex = Regex::new(r"\((\d+),\s*(\d+)\)+").unwrap();
}

pub struct ShamirSecretSharing {
    pub shares: i32,
    pub threshold: i32,
    pub polynomial: Vec<i32>, // The constant coefficient of the polynomial is the secret
    pub modulus: i32,
}

impl Default for ShamirSecretSharing {
    fn default() -> Self {
        Self {
            shares: 3,
            threshold: 3,
            polynomial: vec![65, 2347, 542],
            modulus: 2147483423,
        }
    }
}

impl ShamirSecretSharing {
    fn lagrange(&self, x: i32, pairs: Vec<(i32, i32)>) -> Option<i32> {
        let mut nums: Vec<i32> = Vec::new();
        let mut dens = Vec::new();

        for i in 0..pairs.len() {
            let mut others = pairs.iter().map(|(x, _)| *x).collect_vec();
            let cur = others.remove(i);
            nums.push(others.iter().map(|e| x - *e).product());
            dens.push(others.iter().map(|e| cur - *e).product());
        }

        let denominator = dens.iter().product::<i32>() % self.modulus;

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
        if !is_prime32(self.modulus as u32) {
            return Err(CipherError::state("modulus must be prime"));
        }
        if self.threshold < 1 {
            return Err(CipherError::state("threshold must be positive"));
        }
        if self.shares < 1 {
            return Err(CipherError::state("shares must be positive"));
        }
        if self.threshold > self.shares {
            return Err(CipherError::state(
                "cannot require a greater threshold than there are shares",
            ));
        };
        Ok(())
    }
}

impl Cipher for ShamirSecretSharing {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        self.check_state()?;

        if self.polynomial.len() < (self.threshold - 1) as usize {
            return Err(CipherError::State(format!(
                "a threshold of {} requires a polynomial with {} coefficients",
                self.threshold,
                self.threshold - 1
            )));
        }

        let secret =
            i32::from_str_radix(text, 10).map_err(|e| CipherError::Input(e.to_string()))?;

        let p = {
            let mut p = self.polynomial.clone();
            p.insert(0, secret);
            p
        };

        let mut out = Vec::with_capacity(self.shares as usize);
        for x in 1..=self.shares {
            out.push((x, eval_poly(x, &p, self.modulus) as i32))
        }

        Ok(out.iter().map(|p| format!("{p:?}")).join(", "))
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        self.check_state()?;
        let mut pairs = Vec::new();
        for p in PAIRS.captures_iter(text) {
            let x =
                i32::from_str_radix(&p[1], 10).map_err(|e| CipherError::Input(e.to_string()))?;
            let y =
                i32::from_str_radix(&p[2], 10).map_err(|e| CipherError::Input(e.to_string()))?;
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
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT)
    }

    #[test]
    fn decrypt_test() {
        let mut cipher = ShamirSecretSharing::default();
        cipher.modulus = 1613;
        cipher.polynomial = vec![166, 94];
        cipher.shares = 6;
        cipher.threshold = 4;
        assert_eq!(cipher.decrypt(CIPHERTEXT).unwrap(), PLAINTEXT)
    }
}
