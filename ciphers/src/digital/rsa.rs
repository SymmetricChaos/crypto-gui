use num::{bigint::ToBigUint, BigUint, Integer};
use utils::math_functions::mul_inv;

use crate::{Cipher, CipherError};

use super::ByteFormat;

pub struct Rsa {
    pub output_format: ByteFormat,
    pub input_format: ByteFormat,
    pub n: BigUint,
    pub d: BigUint,
    pub e: BigUint,
    pub lambda: BigUint,
}

impl Default for Rsa {
    fn default() -> Self {
        Self {
            output_format: ByteFormat::Hex,
            input_format: ByteFormat::Hex,
            n: BigUint::default(),
            d: BigUint::default(),
            e: BigUint::default(),
            lambda: BigUint::default(),
        }
    }
}

impl Rsa {
    pub fn key_length(&self) -> u64 {
        self.n.bits()
    }

    pub fn set_key<N: ToBigUint>(&mut self, p: &N, q: &N) {
        let p = p.to_biguint().expect("p could not be converted to BigUint");
        let q = q.to_biguint().expect("q could not be converted to BigUint");
        let n = &p * &q;
        let one = &BigUint::from(1_u32);
        let lambda = (&p - one).lcm(&(&q - one));
        let e = BigUint::from(65537_u32); // A prime constant
        let d = mul_inv(&e, &lambda).expect("modular multiplicative inverse could not be computed");

        self.n = n;
        self.lambda = lambda;
        self.e = e;
        self.d = BigUint::try_from(d)
            .expect("modular multiplicative inverse could not convert to BigUint");
    }

    pub fn public_key(&self) {}

    pub fn encrypt_bytes(&self, bytes: &[u8]) -> Result<Vec<u8>, CipherError> {
        let mut out = Vec::with_capacity(bytes.len());

        Ok(out)
    }

    pub fn decrypt_bytes(&self, bytes: &[u8]) -> Result<Vec<u8>, CipherError> {
        let mut out = Vec::with_capacity(bytes.len());

        Ok(out)
    }
}

impl Cipher for Rsa {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        let mut bytes = self.input_format.text_to_bytes(text)?;
        let out = self.encrypt_bytes(&mut bytes)?;
        Ok(self.output_format.bytes_to_text(&out))
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        let mut bytes = self.input_format.text_to_bytes(text)?;
        let out = self.decrypt_bytes(&mut bytes)?;
        Ok(self.output_format.bytes_to_text(&out))
    }
}

#[cfg(test)]
mod aes_tests {

    use super::*;

    #[test]
    fn test_ksa() {
        let mut cipher = Rsa::default();
        cipher.set_key(&5623, &5869);
    }
}
