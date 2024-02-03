use num::{bigint::ToBigUint, BigUint};

use crate::{Cipher, CipherError};

use super::ByteFormat;

pub struct ElGamal {
    pub output_format: ByteFormat,
    pub input_format: ByteFormat,
    pub group_size: BigUint,
    pub generator: BigUint,
    pub private_key: BigUint,
    pub point: BigUint,
    pub message_key: BigUint,
}

impl Default for ElGamal {
    fn default() -> Self {
        Self {
            output_format: ByteFormat::Hex,
            input_format: ByteFormat::Hex,
            group_size: BigUint::from(2357_u32),
            generator: BigUint::from(2_u32),
            private_key: BigUint::from(1751_u32),
            point: BigUint::from(1185_u32),
            message_key: BigUint::from(1520_u32),
        }
    }
}

impl ElGamal {
    pub fn set_key(&mut self) {
        self.point = self
            .generator
            .modpow(&self.private_key.to_biguint().unwrap(), &self.group_size);
    }

    pub fn encrypt_bytes(&self, bytes: &[u8]) -> Result<(Vec<u8>, Vec<u8>), CipherError> {
        let m = BigUint::from_bytes_be(bytes);
        if m > self.group_size {
            return Err(CipherError::input(
                "message length cannot be greater than group size",
            ));
        };
        let gamma = self.generator.modpow(&self.message_key, &self.group_size);
        let delta = (m * self.point.modpow(&self.message_key, &self.group_size)) % &self.group_size;

        Ok((gamma.to_bytes_be(), delta.to_bytes_be()))
    }

    pub fn decrypt_bytes(&self, gamma: &[u8], delta: &[u8]) -> Result<Vec<u8>, CipherError> {
        let gamma = BigUint::from_bytes_be(gamma);
        let delta = BigUint::from_bytes_be(delta);

        let inv = gamma.modpow(
            &(&self.group_size - &BigUint::from(1_u32) - &self.private_key),
            &self.group_size,
        );

        let m = (inv * delta) % &self.group_size;

        Ok(m.to_bytes_be())
    }
}

impl Cipher for ElGamal {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        let mut bytes = self.input_format.text_to_bytes(text)?;
        let pair = self.encrypt_bytes(&mut bytes)?;
        let out = format!(
            "{}\n{}",
            self.output_format.bytes_to_text(&pair.0),
            self.output_format.bytes_to_text(&pair.1)
        );
        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        if !text.contains('\n') {
            return Err(CipherError::input("no linebreak found"));
        }
        let (t_gamma, t_delta) = text.split_once('\n').unwrap();
        let gamma = self.input_format.text_to_bytes(t_gamma)?;
        let delta = self.input_format.text_to_bytes(t_delta)?;

        let out = self.decrypt_bytes(&gamma, &delta)?;
        Ok(self.output_format.bytes_to_text(&out))
    }
}

#[cfg(test)]
mod elgamal_tests {

    use super::*;

    #[test]
    fn encrypt_decrypt() {
        let cipher = ElGamal::default();
        let ptext = "07f3";
        let ctext = cipher.encrypt(ptext).unwrap();
        assert_eq!(cipher.decrypt(&ctext).unwrap(), ptext);
    }

    #[test]
    fn encrypt() {
        let cipher = ElGamal::default();
        let ptext = "07f3";
        let ctext = cipher.encrypt(ptext).unwrap();
        assert_eq!("0596\n02b9", ctext);
    }
}
