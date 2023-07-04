use crate::{Cipher, CipherError};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use utils::math_functions::eval_poly;

lazy_static! {
    pub static ref PAIRS: Regex = Regex::new(r"\((\d+),\s*(\d+)\)+").unwrap();
}

pub struct ShamirSecretSharing {
    shares: u32,
    threshold: u32,
    polynomial: Vec<u32>, // The constant coefficient of the polynomial is the secret
    modulus: u32,
}

impl Default for ShamirSecretSharing {
    fn default() -> Self {
        Self {
            shares: 6,
            threshold: 4,
            polynomial: vec![166, 94],
            modulus: 1613,
        }
    }
}

impl Cipher for ShamirSecretSharing {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        if self.threshold > self.shares {
            return Err(CipherError::state(
                "cannot require a greater threshold than there are shares",
            ));
        };
        let secret =
            u32::from_str_radix(text, 10).map_err(|e| CipherError::Input(e.to_string()))?;

        let p = {
            let mut p = self.polynomial.clone();
            p.insert(0, secret);
            p
        };

        let mut out = Vec::with_capacity(self.shares as usize);
        for x in 1..=self.shares {
            out.push((x, eval_poly(x, &p, self.modulus)))
        }

        Ok(out.iter().map(|p| format!("{p:?}")).join(", "))
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        let mut pairs = Vec::new();
        for p in PAIRS.captures_iter(text) {
            let a =
                u32::from_str_radix(&p[1], 10).map_err(|e| CipherError::Input(e.to_string()))?;
            let b =
                u32::from_str_radix(&p[2], 10).map_err(|e| CipherError::Input(e.to_string()))?;
            pairs.push((a, b))
        }

        // do the interpolation thing
        todo!()
    }
}

#[cfg(test)]
mod m209_tests {

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
        let cipher = ShamirSecretSharing::default();
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT)
    }
}
