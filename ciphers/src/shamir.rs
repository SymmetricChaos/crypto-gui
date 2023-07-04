use utils::functions::eval_poly;

use crate::{Cipher, CipherError};

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
        for i in 1..=self.shares {
            out.push((i, eval_poly(i, &p, self.modulus)))
        }

        Ok(format!("{:?}", out))
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        // parse the string into pairs
        // do the interpolation thing
        todo!()
    }
}
