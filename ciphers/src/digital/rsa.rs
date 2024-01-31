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
            e: BigUint::from(65537_u32), // a small prime constant
            lambda: BigUint::default(),
        }
    }
}

impl Rsa {
    pub fn set_key<N: ToBigUint>(&mut self, p: &N, q: &N) {
        let p = p.to_biguint().expect("p could not be converted to BigUint");
        let q = q.to_biguint().expect("q could not be converted to BigUint");
        let n = &p * &q;
        let one = &BigUint::from(1_u32);
        let lambda = (&p - one).lcm(&(&q - one));
        let d = mul_inv(&self.e, &lambda)
            .expect("modular multiplicative inverse could not be computed");

        self.n = n;
        self.lambda = lambda;
        self.d = BigUint::try_from(d)
            .expect("modular multiplicative inverse could not convert to BigUint");
    }

    // pub fn padding(&self, bytes: &[u8]) -> Vec<u8> {
    //     todo!()
    // }

    // Returns n and e
    pub fn public_key(&self) -> (BigUint, BigUint) {
        (self.n.clone(), self.e.clone())
    }

    // Returns n and d
    pub fn private_key(&self) -> (BigUint, BigUint) {
        (self.n.clone(), self.d.clone())
    }

    pub fn encrypt_bytes(&self, bytes: &[u8]) -> Result<Vec<u8>, CipherError> {
        let m = BigUint::from_bytes_be(bytes);
        Ok(m.modpow(&self.e, &self.n).to_bytes_be())
    }

    pub fn decrypt_bytes(&self, bytes: &[u8]) -> Result<Vec<u8>, CipherError> {
        let c = BigUint::from_bytes_be(bytes);
        Ok(c.modpow(&self.d, &self.n).to_bytes_be())
    }
}

impl Cipher for Rsa {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        let mut bytes = self.input_format.text_to_bytes(text)?;
        if (bytes.len() * 8) > self.n.bits() as usize {
            return Err(CipherError::input(
                "message length, in bits, cannot be greater than the key size, in bits",
            ));
        }
        let out = self.encrypt_bytes(&mut bytes)?;
        Ok(self.output_format.bytes_to_text(&out))
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        let mut bytes = self.input_format.text_to_bytes(text)?;
        if (bytes.len() * 8) > self.n.bits() as usize {
            return Err(CipherError::input(
                "message length, in bits, cannot be greater than the key size, in bits",
            ));
        }
        let out = self.decrypt_bytes(&mut bytes)?;
        Ok(self.output_format.bytes_to_text(&out))
    }
}

#[cfg(test)]
mod rsa_tests {

    use super::*;

    #[test]
    fn test_ksa() {
        let mut cipher = Rsa::default();
        cipher.set_key(&5623, &5869);
    }

    #[test]
    fn encrypt_decrypt() {
        let mut cipher = Rsa::default();
        cipher.set_key(&5623, &5869);
        let ptext = "010203";
        let ctext = cipher.encrypt(ptext).unwrap();
        assert_eq!(cipher.decrypt(&ctext).unwrap(), ptext);
    }
}
